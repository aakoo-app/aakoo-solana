import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AakooSolana } from "../target/types/aakoo_solana";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL} from '@solana/web3.js'
describe("aakoo-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = anchor.getProvider().connection;

  const program = anchor.workspace.AakooSolana as Program<AakooSolana>;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(`Your txs: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
    return signature;
  };

  const user1 = {
    keypair: Keypair.generate(),
    secret: "secret-user1",
    mobile: "09123456789"
  }
  const seed = new anchor.BN(1);
  const [vault] = PublicKey.findProgramAddressSync(
    [Buffer.from("datavault"), user1.keypair.publicKey.toBuffer(), seed.toBuffer("le", 8)],
    program.programId
  )

  it("Airdrops", async () => {
    await Promise.all([
      await connection
        .requestAirdrop(user1.keypair.publicKey, 3 * LAMPORTS_PER_SOL)
        .then(confirm)
    ]);
  });

  it("Should setup auth", async () => {
    // Add your test here.
    await program.methods.setupAuth
      (seed, user1.mobile, user1.secret)
      .accounts({
        user: user1.keypair.publicKey,
        vault,
        systemProgram: SystemProgram.programId
      })
      .signers([user1.keypair])
      .rpc()
      .then(log);
  });
});
