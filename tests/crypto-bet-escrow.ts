import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoBetEscrow } from "../target/types/crypto_bet_escrow";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import {
  AggregatorAccount,
  AnchorWallet,
  SwitchboardProgram,
} from "@switchboard-xyz/solana.js";
import { Big } from "@switchboard-xyz/common";

const BTC_USD_FEED = new anchor.web3.PublicKey(
  "8SXvChNYFhRq4EZuZvnhjrB3jJRQCv4k3P4W6hesH3Ee"
);

describe("crypto-bet-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.CryptoBetEscrow as Program<CryptoBetEscrow>;
  const initializer = anchor.web3.Keypair.generate();

  it("It Initialize bet!", async () => {
    const winPrice = new anchor.BN(100000);
    const losePrice = new anchor.BN(12000);
    const escrowAmout = new anchor.BN(0.1 * LAMPORTS_PER_SOL);

    try {
      const airdropTx = await connection.requestAirdrop(
        initializer.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      const latestBlockHash = await connection.getLatestBlockhash();
      await connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: airdropTx,
      });
    } catch (err) {
      console.log("error requesting airdrop: ", err);
    }

    const [escrowAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("CRYPTO_ESCROW"), initializer.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .initializeBet(winPrice, losePrice, escrowAmout)
      .accounts({
        initializer: initializer.publicKey,
        escrowAccount: escrowAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([initializer])
      .rpc();

    try {
      const latestBlock = await connection.getLatestBlockhash();
      await connection.confirmTransaction({
        blockhash: latestBlock.blockhash,
        lastValidBlockHeight: latestBlock.lastValidBlockHeight,
        signature: tx,
      });
    } catch (err) {
      console.log("error initialize bet: ", err);
    }

    const escrowState = await program.account.escrowState.fetch(escrowAccount);
    console.log("escrow stateasdasd: ", escrowState);

    assert.ok(escrowState.winPrice.eq(winPrice));
    assert.ok(escrowState.losePrice.eq(losePrice));
    assert.ok(escrowState.escrowAmount.eq(escrowAmout));
    assert.ok(escrowState.initializer.equals(initializer.publicKey));
  });

  it("accept bet", async () => {
    const acceptor = anchor.web3.Keypair.generate();

    try {
      const airdropTx = await connection.requestAirdrop(
        acceptor.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      const latestBlock = await connection.getLatestBlockhash();
      await connection.confirmTransaction({
        blockhash: latestBlock.blockhash,
        lastValidBlockHeight: latestBlock.lastValidBlockHeight,
        signature: airdropTx,
      });
    } catch (err) {
      console.log("error requesting airdrop for acceptor: ", err);
    }

    const [escrowAccout] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("CRYPTO_ESCROW"), acceptor.publicKey.toBuffer()],
      program.programId
    );

    try {
      const tx = await program.methods
        .acceptBet()
        .accounts({
          acceptor: acceptor.publicKey,
          escrowAccount: escrowAccout,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([acceptor])
        .rpc();
      console.log("accept bet sig: ", tx);
    } catch (err) {
      console.log("Error accepting bet: ", err);
    }

    const escrowState = await program.account.escrowState.fetch(escrowAccout);

    assert.ok(escrowState.isActive);
  });

  it("Withdraw", async () => {
    const switchboardProgram = await SwitchboardProgram.load(
      new anchor.web3.Connection("https://api.devnet.solana.com"),
      initializer
    );
    const aggregatorAccount = new AggregatorAccount(
      switchboardProgram,
      BTC_USD_FEED
    );

    const [escrowAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("CRYPTO_ESCROW"), initializer.publicKey.toBuffer()],
      program.programId
    );

    const escrowState = await program.account.escrowState.fetch(escrowAccount);
    if (!escrowState.isActive) {
      throw new Error("Bet is not active");
    }

    const currentPrice: Big | null = await aggregatorAccount.fetchLatestValue();
    if (currentPrice === null) {
      throw new Error("No value");
    }

    const winPrice = currentPrice.add(40000).toNumber();

    let winnerPubkey: PublicKey;
    if (winPrice >= escrowState.winPrice.toNumber()) {
      winnerPubkey = escrowState.initializer;
    } else if (winPrice <= escrowState.losePrice.toNumber()) {
      winnerPubkey = escrowState.acceptor;
    } else {
      throw new Error("Outcome not determined");
    }

    assert.ok(winnerPubkey, "winner must be determined");

    const withdrawTx = await program.methods
      .withdraw()
      .accounts({
        winner: initializer.publicKey,
        escrowAccount: escrowAccount,
        feedAggregator: BTC_USD_FEED,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([initializer])
      .rpc();
    console.log("withdraw sig: ", withdrawTx);
  });
});
