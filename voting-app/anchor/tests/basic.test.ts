/* eslint-env mocha */

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Basic } from '../target/types/basic';

describe('voting', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Basic as Program<Basic>;

  it('initialize pool', async () => {
    const poolId = new anchor.BN(1);
    await program.methods
      .initializePool(poolId)
      .accounts({
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });
});
