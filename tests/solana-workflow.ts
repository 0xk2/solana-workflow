import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolanaWorkflow } from '../target/types/solana_workflow';
import { SingleChoice } from '../target/types/single_choice';

import { PublicKey } from '@solana/web3.js';
import * as borsh from 'borsh';
import axios from 'axios';

type InputCheckpoint = anchor.IdlTypes<SolanaWorkflow>['InputCheckPoint'];
type InputVote = anchor.IdlTypes<SingleChoice>['InputVote'];

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
      voteMachineAddress: new PublicKey(''),
      options: [
        {
          title: 'Cancel',
          nextId: 3,
        },
        {
          title: 'OK',
          nextId: 2,
        },
      ],
    },
    {
      id: 2,
      title: '2nd check: Do you want to proceed?',
      voteMachineAddress: new PublicKey(''),
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
      voteMachineAddress: new PublicKey(''),
      options: [],
    },
    {
      id: 4,
      title: 'Horray, success!',
      voteMachineAddress: new PublicKey(''),
      options: [],
    },
  ],
};

// const dashSdk = {
//   create_workflow: async function (data) {
//     const workflow = {
//       title: data.title,
//       start: 1,
//     };
//     const checkpoints = data.checkpoints.map((checkpoint, index) => {
//       return {
//         id: checkpoint.id,
//         title: checkpoint.title,
//         options: checkpoint.options.map((option) => {
//           return {
//             title: option.title,
//             nextId: option.nextId,
//           };
//         }),
//       };
//     });
//   },
// };

describe('solana-workflow', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const solanaWorkflow = anchor.workspace
    .SolanaWorkflow as Program<SolanaWorkflow>;
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
      })
      .rpc({ skipPreflight: true });

    console.log('Create misison tx: ', tx);
  });

  it('Vote', async () => {
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

    let remainingAccounts = [];
    let coefs = [];
    for (let option of checkpointData.options) {
      let coef = 0;
      let isExist = false;
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

    console.log(coefs, remainingAccounts);

    const vote: InputVote = {
      option: 0,
    };

    const tx = await solanaWorkflow.methods
      .vote(vote, Buffer.from(coefs))
      .accounts({
        user: anchorProvider.wallet.publicKey,
        mission: missionPDA,
        checkpoint: checkpointPDA,
        workflowProgram: solanaWorkflow.programId,
      })
      .remainingAccounts(remainingAccounts)
      .rpc({
        skipPreflight: true,
        maxRetries: 10,
        preflightCommitment: 'confirmed',
      });

    console.log('Create vote tx: ', tx);
  });
});
