import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));

const to = new PublicKey("EF5ZDRqSGfbTpiXQ8R7Mj4xdFehpSeVFPiYa9Fpnwg4E");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const balance = await connection.getBalance(from.publicKey);

    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      })
    );

    tx.recentBlockhash = (
      await connection.getLatestBlockhash("confirmed")
    ).blockhash;

    tx.feePayer = from.publicKey;

    const fee: number = (
      await connection.getFeeForMessage(tx.compileMessage(), "confirmed")
    ).value as number;

    tx.instructions.pop();

    tx.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    const sig = await sendAndConfirmTransaction(connection, tx, [from]);

    console.log("success :", sig);
  } catch (error) {
    console.error("Oops something went wrong : ", error);
  }
})();
