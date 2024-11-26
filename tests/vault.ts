import * as anchor from "@coral-xyz/anchor";
import { BN } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import { PublicKey, Keypair, Transaction, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import fs from 'fs';

// const keypairFilePath = '/home/canfly/.config/solana/cli/spng.json';
const keypairFilePath = '/home/canfly/.config/solana/cli/id.json';

function loadKeypairFromFile(filePath) {
  const secretKeyString = fs.readFileSync(filePath, { encoding: 'utf8' });
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

describe("vault", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const connection = provider.connection;
  const program = anchor.workspace.Vault as Program<Vault>;
  
  const authority = loadKeypairFromFile(keypairFilePath);
  console.log("Authority Public Key:", authority.publicKey.toBase58());
  const payer = Keypair.generate(); // 使用者 - 存 SOL 的人

  // 設置 PDA
  const vaultStatePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("vault_state"), authority.publicKey.toBytes()],
    program.programId
  )[0];

  const vaultPDA = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultStatePDA.toBytes()],
    program.programId
  )[0];

  const userStatePDA = PublicKey.findProgramAddressSync(
    [Buffer.from("user_state"), payer.publicKey.toBytes()],
    program.programId
  )[0];

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  xit("Transfer SOL for testing.", async () => {
    let tx = new Transaction();
    
    tx.add(
      SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: authority.publicKey,
          lamports: 0.03 * LAMPORTS_PER_SOL,
      })
    );

    tx.add(
      SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: payer.publicKey,
          lamports: 0.03 * LAMPORTS_PER_SOL,
      })
    );

    await provider.sendAndConfirm(tx).then(log);
  });  

  xit("Initialize Vault", async () => {
    await program.methods
      .initializeVault()
      .accounts({
        authority: authority.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc()
      .then(confirm)
      .then(log);
  });

  xit("Transfer SOL to vault.", async () => {
    let tx = new Transaction();
    
    tx.add(
      SystemProgram.transfer({
          fromPubkey: authority.publicKey,
          toPubkey: vaultPDA,
          lamports: 0.01 * LAMPORTS_PER_SOL,
      })
    );

    await provider.sendAndConfirm(tx,[authority]).then(log);
  });  

  xit("Stake", async () => {
    await program.methods
      .stake(new BN(1_000)) // 0.001 SOL
      .accounts({
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        userState: userStatePDA,
        user: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc()
      .then(confirm)
      .then(log);
  });

  xit("Unstake", async () => {
    await program.methods
      .unstake(new BN(1_000)) // 0.001 SOL
      .accounts({
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        userState: userStatePDA,
        user: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc()
      .then(confirm)
      .then(log);
  });

  xit("Close Vault", async () => {
    await program.methods
      .closeVault()
      .accounts({
        authority: authority.publicKey,
        vaultState: vaultStatePDA,
        vault: vaultPDA,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc()
      .then(confirm)
      .then(log);
  });
});

// describe("vault", () => {
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.Vault as Program<Vault>;

//   const provider = anchor.getProvider();

//   const connection = provider.connection;

//   const vaultPDA = PublicKey.findProgramAddressSync(
//     [provider.publicKey.toBuffer()],
//     program.programId
//   )[0];

//   const confirm = async (signature: string): Promise<string> => {
//     const block = await connection.getLatestBlockhash();
//     await connection.confirmTransaction({
//       signature,
//       ...block,
//     });
//     return signature;
//   };

//   const log = async (signature: string): Promise<string> => {
//     console.log(
//       `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
//     );
//     return signature;
//   };

//   it("Stake", async () => {
//     const tx = await program.methods.stake(new BN(1_000_000))
//     .accounts({
//       signer: provider.publicKey!,
//       vault: vaultPDA,
//     })
//     .rpc()
//     .then(confirm)
//     .then(log)
//   });

//   it("Unstake", async () => {
//     const tx = await program.methods.unstake(new BN(1_000_000))
//     .accounts({
//       signer: provider.publicKey!,
//       vault: vaultPDA,
//     })
//     .rpc()
//     .then(confirm)
//     .then(log)
//   });
// });