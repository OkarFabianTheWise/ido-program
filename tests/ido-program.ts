import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IdoProgram } from "../target/types/ido_program";
import { assert, expect } from "chai";

describe("ido-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IdoProgram as Program<IdoProgram>;

  let saleState: anchor.web3.PublicKey;
  let whitelist: anchor.web3.PublicKey;
  let vestingState: anchor.web3.PublicKey;
  let tokenMint: anchor.web3.PublicKey;
  let buyer: anchor.web3.Keypair;

  before(async () => {
    const mintKeypair = anchor.web3.Keypair.generate();
    tokenMint = mintKeypair.publicKey;

    await airdrop(provider.connection, provider.wallet.publicKey, 2000000000);

    buyer = anchor.web3.Keypair.generate();
    await airdrop(provider.connection, buyer.publicKey, 2000000000);

    // Derive PDAs
    [saleState] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("sale_state"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    [whitelist] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("whitelist"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes the sale", async () => {
    const saleParams = {
      startTime: new anchor.BN(Math.floor(Date.now() / 1000) + 10),
      endTime: new anchor.BN(Math.floor(Date.now() / 1000) + 60),
      tokenPrice: new anchor.BN(1_000_000),
      totalTokens: new anchor.BN(1_000),
    };

    const tx = await program.methods
      .initializeSale(saleParams)
      .accounts({
        admin: provider.wallet.publicKey,
        saleState,
        whitelist,
        tokenMint,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize Sale Transaction:", tx);

    const state = await program.account.saleState.fetch(saleState);
    expect(state.tokenMint.toString()).to.equal(tokenMint.toString());
    expect(state.totalTokens.toNumber()).to.equal(1000);
  });

  it("Adds and verifies whitelist entry", async () => {
    const tx = await program.methods
      .addToWhitelist(buyer.publicKey)
      .accounts({
        admin: provider.wallet.publicKey,
        whitelist,
      })
      .rpc();

    console.log("Add To Whitelist Transaction:", tx);

    const whitelistState = await program.account.whitelist.fetch(whitelist);
    const isWhitelisted = whitelistState.users.some(([key]) => key === buyer.publicKey.toString());
    expect(isWhitelisted).to.be.true;
  });

  it("Prevents purchase before sale starts", async () => {
    const amountToBuy = new anchor.BN(10);

    try {
      await program.methods
        .purchaseTokens(amountToBuy)
        .accounts({
          buyer: buyer.publicKey,
          saleState,
          whitelist,
          saleVault: provider.wallet.publicKey,
          vestingState: anchor.web3.Keypair.generate().publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([buyer])
        .rpc();
      assert.fail("Expected SaleNotActive error.");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("SaleNotActive");
    }
  });

  it("Allows purchase during active sale", async () => {
    await new Promise((resolve) => setTimeout(resolve, 11_000)); // Wait 11 seconds

    const amountToBuy = new anchor.BN(10);

    const tx = await program.methods
      .purchaseTokens(amountToBuy)
      .accounts({
        buyer: buyer.publicKey,
        saleState,
        whitelist,
        saleVault: provider.wallet.publicKey,
        vestingState: anchor.web3.Keypair.generate().publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([buyer])
      .rpc();

    console.log("Purchase Transaction:", tx);

    const state = await program.account.saleState.fetch(saleState);
    expect(state.soldTokens.toNumber()).to.equal(10);
  });

  it("Prevents purchase after sale ends", async () => {
    await new Promise((resolve) => setTimeout(resolve, 51_000)); // Wait 51 seconds

    const amountToBuy = new anchor.BN(10);

    try {
      await program.methods
        .purchaseTokens(amountToBuy)
        .accounts({
          buyer: buyer.publicKey,
          saleState,
          whitelist,
          saleVault: provider.wallet.publicKey,
          vestingState: anchor.web3.Keypair.generate().publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([buyer])
        .rpc();
      assert.fail("Expected SaleNotActive error.");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("SaleNotActive");
    }
  });

  it("Claims vested tokens", async () => {
    const tx = await program.methods
      .claim()
      .accounts({
        vestingState,
        vaultTokenAccount: provider.wallet.publicKey,
        beneficiaryTokenAccount: buyer.publicKey,
        vaultAuthority: provider.wallet.publicKey,
        tokenProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Claim Transaction:", tx);

    const vesting = await program.account.vestingState.fetch(vestingState);
    expect(vesting.releasedAmount.toNumber()).to.be.greaterThan(0);
  });
});


async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}