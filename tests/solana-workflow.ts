import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolanaWorkflow } from '../target/types/solana_workflow';
import { SingleChoice } from '../target/types/single_choice';
import { DocInput } from '../target/types/doc_input';

import {
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  clusterApiUrl,
  Keypair,
} from '@solana/web3.js';
import * as borsh from 'borsh';
import axios from 'axios';

type InputCheckpoint = anchor.IdlTypes<SolanaWorkflow>['InputCheckPoint'];
type InputVoteDocInput = anchor.IdlTypes<DocInput>['InputVote'];
type InputVoteSingleChoice = anchor.IdlTypes<SingleChoice>['InputVote'];

type Workflow = {
  id: number;
  title: string;
  start: number;
  checkpoints: Array<InputCheckpoint>;
  noVariable: number;
};

const workflow: Workflow = {
  title: 'My first workflow',
  start: 1,
  id: 1,
  noVariable: 4,
  checkpoints: [
    {
      id: 1,
      title: '1st check: Do you want to proceed?',
      voteMachineAddress: new PublicKey(
        'D1gMCgf8gHdUNDmpUfe1fHuUQci2JJFCw7CGv184hNMv'
      ),
      data: null,
      options: [
        {
          title: 'OK',
          nextId: 2,
        },
        {
          title: 'Cancel',
          nextId: 3,
        },
      ],
    },
    {
      id: 2,
      title: '2nd check: Do you want to proceed?',
      voteMachineAddress: new PublicKey(
        '3XR9BbkbddGNFCbEG59XXEy9MydHZmGZb6jEq4VxQWY7'
      ),
      data: Buffer.from([2]),
      options: [
        {
          title: 'Cancel',
          nextId: 3,
        },
        {
          title: 'OK',
          nextId: 4,
        },
      ],
    },
    {
      id: 3,
      title: 'You have cancelled the workflow.',
      voteMachineAddress: new PublicKey(
        'D1gMCgf8gHdUNDmpUfe1fHuUQci2JJFCw7CGv184hNMv'
      ),
      options: null,
      data: null,
    },
    {
      id: 4,
      title: 'Horray, success!',
      voteMachineAddress: new PublicKey(
        'D1gMCgf8gHdUNDmpUfe1fHuUQci2JJFCw7CGv184hNMv'
      ),
      options: null,
      data: null,
    },
  ],
};

const dashSdk = {
  create_workflow: async function (data) {
    const workflow = {
      title: data.title,
      start: 1,
    };
    const checkpoints = data.checkpoints.map((checkpoint, index) => {
      return {
        id: checkpoint.id,
        title: checkpoint.title,
        options: checkpoint.options.map((option) => {
          return {
            title: option.title,
            nextId: option.nextId,
          };
        }),
      };
    });
  },
};

describe('solana-workflow', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const solanaWorkflow = anchor.workspace
    .SolanaWorkflow as Program<SolanaWorkflow>;
  const docInput = anchor.workspace.DocInput as Program<DocInput>;
  const singleChoice = anchor.workspace.SingleChoice as Program<SingleChoice>;

  const anchorProvider = solanaWorkflow.provider as anchor.AnchorProvider;

  it('Create workflow', async () => {
    const [workflowPDA, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from('workflow'), anchorProvider.wallet.publicKey.toBuffer()],
      solanaWorkflow.programId
    );

    let remainingAccounts: any[] = [];
    for (let i = 0; i < workflow.checkpoints.length; i++) {
      const [checkpointPDA, bump] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(borsh.serialize('u16', workflow.checkpoints[i].id)),
          Buffer.from('checkpoint'),
          workflowPDA.toBuffer(),
        ],
        solanaWorkflow.programId
      );

      remainingAccounts.push({
        pubkey: checkpointPDA,
        isWritable: true,
        isSigner: false,
      });
    }

    // Add your test here.
    const tx = await solanaWorkflow.methods
      .createWorkflow(
        workflow.title,
        workflow.start,
        new anchor.BN(workflow.id),
        workflow.checkpoints
      )
      .accounts({
        user: anchorProvider.wallet.publicKey,
        workflow: workflowPDA,
        workflowProgram: solanaWorkflow.programId,
      })
      .remainingAccounts(remainingAccounts)
      .rpc({ skipPreflight: true });

    console.log('Create workflow tx: ', tx);
  });

  it('Create mission', async () => {
    const [missionPDA, _] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('mission'),
        anchorProvider.wallet.publicKey.toBuffer(),
        Buffer.from(borsh.serialize('u64', 1)),
      ],
      solanaWorkflow.programId
    );

    const [voteData, __] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('vote_data'),
        missionPDA.toBytes(),
        Buffer.from(borsh.serialize('u64', 1)),
        Buffer.from([0]),
      ],
      solanaWorkflow.programId
    );

    let remainingAccounts = [];
    for (let i = 0; i < workflow.noVariable; i++) {
      const [variable, _] = PublicKey.findProgramAddressSync(
        [Buffer.from('variable'), missionPDA.toBuffer(), Buffer.from([i])],
        solanaWorkflow.programId
      );

      remainingAccounts.push({
        pubkey: variable,
        isWritable: true,
        isSigner: false,
      });
    }

    const tx = await solanaWorkflow.methods
      .createMission(
        new anchor.BN(1),
        new anchor.BN(1),
        'Test mission',
        'This is test mission',
        voteData,
        1,
        new anchor.BN(1)
      )
      .accounts({
        user: anchorProvider.wallet.publicKey,
        mission: missionPDA,
        voteData: voteData,
        workflowProgram: solanaWorkflow.programId,
      })
      .remainingAccounts(remainingAccounts)
      .rpc({ skipPreflight: true });

    console.log('Create misison tx: ', tx);
  });

  it('Vote DocInput', async () => {
    const [missionPDA, _] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('mission'),
        anchorProvider.wallet.publicKey.toBuffer(),
        Buffer.from(borsh.serialize('u64', 1)),
      ],
      solanaWorkflow.programId
    );

    const currentVoteData = (
      await solanaWorkflow.account.mission.fetch(missionPDA)
    ).currentVoteData;

    const checkpointId = (
      await solanaWorkflow.account.voteData.fetch(currentVoteData)
    ).checkpointId;

    const checkpointData = workflow.checkpoints.find(
      (cp: InputCheckpoint) => cp.id === checkpointId
    );

    const [workflowPDA, __] = PublicKey.findProgramAddressSync(
      [Buffer.from('workflow'), anchorProvider.wallet.publicKey.toBuffer()],
      solanaWorkflow.programId
    );

    const [checkpointPDA, ___] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(borsh.serialize('u16', checkpointId)),
        Buffer.from('checkpoint'),
        workflowPDA.toBuffer(),
      ],
      solanaWorkflow.programId
    );

    const [tmpVoteData, ____] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('tmp_vote_data'),
        missionPDA.toBuffer(),
        currentVoteData.toBuffer(),
      ],
      docInput.programId
    );

    let remainingAccounts = [];

    let coefs = [];
    for (let option of checkpointData.options) {
      let coef = 0;
      let isExist = false;

      const [nextCheckpointPDA, ___] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(borsh.serialize('u16', option.nextId)),
          Buffer.from('checkpoint'),
          workflowPDA.toBuffer(),
        ],
        solanaWorkflow.programId
      );

      remainingAccounts.push({
        pubkey: nextCheckpointPDA,
        isWritable: false,
        isSigner: false,
      });

      while (isExist === false || coef === 8) {
        const [nextVoteData, __] = PublicKey.findProgramAddressSync(
          [
            Buffer.from('vote_data'),
            missionPDA.toBuffer(),
            Buffer.from(borsh.serialize('u16', option.nextId)),
            Buffer.from([coef]),
          ],
          solanaWorkflow.programId
        );

        const url = 'http://localhost:8899';
        const data = {
          jsonrpc: '2.0',
          id: 1,
          method: 'getAccountInfo',
          params: [
            nextVoteData,
            {
              encoding: 'base58',
            },
          ],
        };

        try {
          const response = await axios.post(url, data, {
            headers: {
              'Content-Type': 'application/json',
            },
          });

          if (!response.data.result.value) {
            isExist = true;

            remainingAccounts.push({
              pubkey: nextVoteData,
              isWritable: true,
              isSigner: false,
            });

            coefs.push(coef);
          }
        } catch (error) {
          console.error('Error fetching data:', error);
        }

        coef++;
      }
    }

    const vote: InputVoteDocInput = {
      option: 0,
      submission: Buffer.from('This is for you'),
      variableId: 1,
    };

    const [variable, bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('variable'),
        missionPDA.toBuffer(),
        Buffer.from([vote.variableId]),
      ],
      solanaWorkflow.programId
    );

    remainingAccounts.push({
      pubkey: variable,
      isWritable: true,
      isSigner: false,
    });

    const tx = await docInput.methods
      .vote(vote)
      .accounts({
        user: anchorProvider.wallet.publicKey,
        mission: missionPDA,
        voteData: currentVoteData,
        checkpoint: checkpointPDA,
        dash: solanaWorkflow.programId,
        tmpVoteData: tmpVoteData,
      })
      .remainingAccounts(remainingAccounts)
      .rpc({ skipPreflight: true });

    console.log(tx);
  });

  it('Vote Single Choice 1', async () => {
    const [missionPDA, _] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('mission'),
        anchorProvider.wallet.publicKey.toBuffer(),
        Buffer.from(borsh.serialize('u64', 1)),
      ],
      solanaWorkflow.programId
    );

    const currentVoteData = (
      await solanaWorkflow.account.mission.fetch(missionPDA)
    ).currentVoteData;

    const checkpointId = (
      await solanaWorkflow.account.voteData.fetch(currentVoteData)
    ).checkpointId;

    const checkpointData = workflow.checkpoints.find(
      (cp: InputCheckpoint) => cp.id === checkpointId
    );

    const [workflowPDA, __] = PublicKey.findProgramAddressSync(
      [Buffer.from('workflow'), anchorProvider.wallet.publicKey.toBuffer()],
      solanaWorkflow.programId
    );

    const [checkpointPDA, ___] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(borsh.serialize('u16', checkpointId)),
        Buffer.from('checkpoint'),
        workflowPDA.toBuffer(),
      ],
      solanaWorkflow.programId
    );

    const [tmpVoteData, ____] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('tmp_vote_data'),
        missionPDA.toBuffer(),
        currentVoteData.toBuffer(),
      ],
      singleChoice.programId
    );

    let remainingAccounts = [];

    let coefs = [];
    for (let option of checkpointData.options) {
      let coef = 0;
      let isExist = false;

      const [nextCheckpointPDA, ___] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(borsh.serialize('u16', option.nextId)),
          Buffer.from('checkpoint'),
          workflowPDA.toBuffer(),
        ],
        solanaWorkflow.programId
      );

      remainingAccounts.push({
        pubkey: nextCheckpointPDA,
        isWritable: false,
        isSigner: false,
      });

      while (isExist === false || coef === 8) {
        const [nextVoteData, __] = PublicKey.findProgramAddressSync(
          [
            Buffer.from('vote_data'),
            missionPDA.toBuffer(),
            Buffer.from(borsh.serialize('u16', option.nextId)),
            Buffer.from([coef]),
          ],
          solanaWorkflow.programId
        );

        const url = 'http://localhost:8899';
        const data = {
          jsonrpc: '2.0',
          id: 1,
          method: 'getAccountInfo',
          params: [
            nextVoteData,
            {
              encoding: 'base58',
            },
          ],
        };

        try {
          const response = await axios.post(url, data, {
            headers: {
              'Content-Type': 'application/json',
            },
          });

          if (!response.data.result.value) {
            isExist = true;

            remainingAccounts.push({
              pubkey: nextVoteData,
              isWritable: true,
              isSigner: false,
            });

            coefs.push(coef);
          }
        } catch (error) {
          console.error('Error fetching data:', error);
        }

        coef++;
      }
    }

    const vote: InputVoteSingleChoice = {
      option: 0,
      who: anchorProvider.wallet.publicKey,
    };

    const tx = await singleChoice.methods
      .vote(vote)
      .accounts({
        user: anchorProvider.wallet.publicKey,
        mission: missionPDA,
        voteData: currentVoteData,
        checkpoint: checkpointPDA,
        dash: solanaWorkflow.programId,
        tmpVoteData: tmpVoteData,
      })
      .remainingAccounts(remainingAccounts)
      .rpc({ skipPreflight: true });

    console.log(tx);
  });

  it('Vote Single Choice 2', async () => {
    const [missionPDA, _] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('mission'),
        anchorProvider.wallet.publicKey.toBuffer(),
        Buffer.from(borsh.serialize('u64', 1)),
      ],
      solanaWorkflow.programId
    );

    const currentVoteData = (
      await solanaWorkflow.account.mission.fetch(missionPDA)
    ).currentVoteData;

    const checkpointId = (
      await solanaWorkflow.account.voteData.fetch(currentVoteData)
    ).checkpointId;

    const checkpointData = workflow.checkpoints.find(
      (cp: InputCheckpoint) => cp.id === checkpointId
    );

    const [workflowPDA, __] = PublicKey.findProgramAddressSync(
      [Buffer.from('workflow'), anchorProvider.wallet.publicKey.toBuffer()],
      solanaWorkflow.programId
    );

    const [checkpointPDA, ___] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(borsh.serialize('u16', checkpointId)),
        Buffer.from('checkpoint'),
        workflowPDA.toBuffer(),
      ],
      solanaWorkflow.programId
    );

    const [tmpVoteData, ____] = PublicKey.findProgramAddressSync(
      [
        Buffer.from('tmp_vote_data'),
        missionPDA.toBuffer(),
        currentVoteData.toBuffer(),
      ],
      singleChoice.programId
    );

    let remainingAccounts = [];

    let coefs = [];
    for (let option of checkpointData.options) {
      let coef = 0;
      let isExist = false;

      const [nextCheckpointPDA, ___] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(borsh.serialize('u16', option.nextId)),
          Buffer.from('checkpoint'),
          workflowPDA.toBuffer(),
        ],
        solanaWorkflow.programId
      );

      remainingAccounts.push({
        pubkey: nextCheckpointPDA,
        isWritable: false,
        isSigner: false,
      });

      while (isExist === false || coef === 8) {
        const [nextVoteData, __] = PublicKey.findProgramAddressSync(
          [
            Buffer.from('vote_data'),
            missionPDA.toBuffer(),
            Buffer.from(borsh.serialize('u16', option.nextId)),
            Buffer.from([coef]),
          ],
          solanaWorkflow.programId
        );

        const url = 'http://localhost:8899';
        const data = {
          jsonrpc: '2.0',
          id: 1,
          method: 'getAccountInfo',
          params: [
            nextVoteData,
            {
              encoding: 'base58',
            },
          ],
        };

        try {
          const response = await axios.post(url, data, {
            headers: {
              'Content-Type': 'application/json',
            },
          });

          if (!response.data.result.value) {
            isExist = true;

            remainingAccounts.push({
              pubkey: nextVoteData,
              isWritable: true,
              isSigner: false,
            });

            coefs.push(coef);
          }
        } catch (error) {
          console.error('Error fetching data:', error);
        }

        coef++;
      }
    }
    const voter = Keypair.generate();

    const SOLANA_CONNECTION = new Connection('http://127.0.0.1:8899');
    const WALLET_ADDRESS = voter.publicKey;
    const AIRDROP_AMOUNT = 1 * LAMPORTS_PER_SOL;

    const signature = await SOLANA_CONNECTION.requestAirdrop(
      new PublicKey(WALLET_ADDRESS),
      AIRDROP_AMOUNT
    );

    const { blockhash, lastValidBlockHeight } =
      await SOLANA_CONNECTION.getLatestBlockhash();
    await SOLANA_CONNECTION.confirmTransaction(
      {
        blockhash,
        lastValidBlockHeight,
        signature,
      },
      'finalized'
    );

    const vote: InputVoteSingleChoice = {
      option: 0,
      who: voter.publicKey,
    };

    console.log(`Airdrop to ${voter.publicKey}`);

    const tx = await singleChoice.methods
      .vote(vote)
      .accounts({
        user: voter.publicKey,
        mission: missionPDA,
        voteData: currentVoteData,
        checkpoint: checkpointPDA,
        dash: solanaWorkflow.programId,
        tmpVoteData: tmpVoteData,
      })
      .signers([voter])
      .remainingAccounts(remainingAccounts)
      .rpc({ skipPreflight: true });

    console.log(tx);
  });
});
