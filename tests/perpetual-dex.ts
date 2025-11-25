import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PerpetualDex } from "../target/types/perpetual_dex";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("perpetual-dex", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PerpetualDex as Program<PerpetualDex>;
  
  const authority = Keypair.generate();
  const trader = Keypair.generate();
  const marketId = Keypair.generate();
  const baseMint = Keypair.generate();
  const quoteMint = Keypair.generate();

  it("Initializes the global configuration", async () => {
    const [globalConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from("perpetual_config")],
      program.programId
    );

    const feeRecipient = Keypair.generate().publicKey;

    const tx = await program.methods
      .initialize({
        feeRecipient,
        tradingFeeBps: 10, // 0.1%
        liquidationFeeBps: 200, // 2%
        maxLeverageBps: 10000, // 100x
        initialMarginBps: 1000, // 10%
        maintenanceMarginBps: 500, // 5%
        liquidationThresholdBps: 400, // 4%
      })
      .accounts({
        globalConfig,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    console.log("Initialize transaction signature", tx);

    const config = await program.account.globalConfig.fetch(globalConfig);
    expect(config.isInitialized).to.be.true;
    expect(config.maxLeverageBps).to.equal(10000);
  });

  it("Creates a new market", async () => {
    const [globalConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from("perpetual_config")],
      program.programId
    );

    const [market] = PublicKey.findProgramAddressSync(
      [Buffer.from("perpetual_market"), marketId.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .createMarket({
        baseMint: baseMint.publicKey,
        quoteMint: quoteMint.publicKey,
        name: "SOL-PERP",
        symbol: "SOL/USDC",
        initialPrice: new anchor.BN(100_000_000_000), // 100 USDC with 8 decimals
      })
      .accounts({
        globalConfig,
        marketId: marketId.publicKey,
        market,
        authority: authority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    console.log("Create market transaction signature", tx);

    const marketAccount = await program.account.market.fetch(market);
    expect(marketAccount.name).to.equal("SOL-PERP");
    expect(marketAccount.currentPrice.toNumber()).to.equal(100_000_000_000);
  });

  // Add more tests for other instructions...
});

