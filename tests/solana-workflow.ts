import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SolanaWorkflow } from '../target/types/solana_workflow';

const workflow = {
  title: 'My first workflow',
  start: 1,
  checkpoints: [
    {
      id: 1,
      title: '1st check: Do you want to proceed?',
      options: [
        {
          title: 'Cancel',
          next: 3,
        },
        {
          title: 'OK',
          next: 2,
        },
      ],
    },
    {
      id: 2,
      title: '2nd check: Do you want to proceed?',
      options: [
        {
          title: 'Cancel',
          next: 3,
        },
        {
          title: 'OK',
          next: 4,
        },
      ],
    },
    {
      id: 3,
      title: 'You have cancelled the workflow.',
    },
    {
      id: 4,
      title: 'Horray, success!',
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
            next: option.next,
          };
        }),
      };
    });
  },
};

describe('solana-workflow', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaWorkflow as Program<SolanaWorkflow>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log('Your transaction signature', tx);
  });
});
