import {
  Keypair,
  Transaction,
  TransactionInstruction as TxIx,
} from "@solana/web3.js";

import { TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

import * as anchor from "@coral-xyz/anchor";

import { BanksClient, ProgramTestContext, startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";

import { CreateNftMetadataToken2022 } from "../target/types/create_nft_metadata_token_2022";

describe("create-nft-metadata-token-2022", () => {
  let context: ProgramTestContext;
  let client: BanksClient;
  let senderKeys: Keypair;
  let provider: BankrunProvider;
  let program: anchor.Program<CreateNftMetadataToken2022>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  beforeEach(async () => {
    // Configure the testing environment
    context = await startAnchor(
      "",
      [
        {
          name: "token_metadata_program",
          programId: TOKEN_METADATA_PROGRAM_ID,
        },
      ],
      []
    );
    client = context.banksClient;

    provider = new BankrunProvider(context);
    anchor.setProvider(provider);
    // DEV: The program must be fetched after the provider has been set, so that the data from inside a program's PDA can be accessed/used as a seed for another PDA
    program = anchor.workspace
      .CreateNftMetadataToken2022 as anchor.Program<CreateNftMetadataToken2022>;

    // Initialize the sender account
    senderKeys = provider.wallet.payer;
  });

  it("Creates an NFT with Metadata via the SPL Token program", async () => {
    let ix = await program.methods
      .createNftWithMetadata()
      .accounts({
        signer: senderKeys.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    // Build, sign and process the transaction
    await buildSignAndProcessTxFromIx(ix, senderKeys);
  });

  it("Creates an NFT with Metadata via the Token2022 program", async () => {
    let ix = await program.methods
      .createNftWithMetadata()
      .accounts({
        signer: senderKeys.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .instruction();

    // Build, sign and process the transaction
    await buildSignAndProcessTxFromIx(ix, senderKeys);
  });

  // HELPER FUNCTIONS AND DATA STRUCTS

  async function buildSignAndProcessTxFromIx(ix: TxIx, signerKeys: Keypair) {
    const tx = await initializeTxWithIx(ix);

    tx.sign(signerKeys);
    await client.processTransaction(tx);
  }

  async function initializeTxWithIx(ix: TxIx): Promise<Transaction> {
    const res = await client.getLatestBlockhash();
    if (!res) throw new Error("Couldn't get the latest blockhash");

    let tx = new Transaction();
    tx.recentBlockhash = res[0];
    tx.add(ix);
    return tx;
  }
});
