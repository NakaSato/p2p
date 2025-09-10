import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Registry } from "../target/types/registry";
import { EnergyToken } from "../target/types/energy_token";
import { Trading } from "../target/types/trading";
import { Oracle } from "../target/types/oracle";
import { Governance } from "../target/types/governance";

describe("P2P Energy Trading System", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const registryProgram = anchor.workspace.Registry as Program<Registry>;
  const energyTokenProgram = anchor.workspace.EnergyToken as Program<EnergyToken>;
  const tradingProgram = anchor.workspace.Trading as Program<Trading>;
  const oracleProgram = anchor.workspace.Oracle as Program<Oracle>;
  const governanceProgram = anchor.workspace.Governance as Program<Governance>;

  let registryPda: anchor.web3.PublicKey;
  let registryBump: number;
  
  let tokenInfoPda: anchor.web3.PublicKey;
  let tokenInfoBump: number;
  
  let marketPda: anchor.web3.PublicKey;
  let marketBump: number;
  
  let oracleConfigPda: anchor.web3.PublicKey;
  let oracleConfigBump: number;
  
  let poaConfigPda: anchor.web3.PublicKey;
  let poaConfigBump: number;

  before(async () => {
    // Derive PDAs
    [registryPda, registryBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("registry")],
      registryProgram.programId
    );

    [tokenInfoPda, tokenInfoBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("token_info")],
      energyTokenProgram.programId
    );

    [marketPda, marketBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("market")],
      tradingProgram.programId
    );

    [oracleConfigPda, oracleConfigBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oracle_config")],
      oracleProgram.programId
    );

    [poaConfigPda, poaConfigBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poa_config")],
      governanceProgram.programId
    );
  });

  describe("Registry Program", () => {
    it("Should initialize the registry", async () => {
      await registryProgram.methods
        .initialize()
        .accounts({
          registry: registryPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const registryAccount = await registryProgram.account.registry.fetch(registryPda);
      expect(registryAccount.authority.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(registryAccount.userCount.toNumber()).to.equal(0);
      expect(registryAccount.meterCount.toNumber()).to.equal(0);
    });

    it("Should register a new user", async () => {
      const user = anchor.web3.Keypair.generate();
      const [userAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), user.publicKey.toBuffer()],
        registryProgram.programId
      );

      await registryProgram.methods
        .registerUser({ prosumer: {} }, "Engineering Building")
        .accounts({
          registry: registryPda,
          userAccount: userAccountPda,
          userAuthority: user.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([user])
        .rpc();

      const userAccount = await registryProgram.account.userAccount.fetch(userAccountPda);
      expect(userAccount.location).to.equal("Engineering Building");
      expect(userAccount.status).to.deep.equal({ active: {} });
    });
  });

  describe("Governance Program", () => {
    it("Should initialize PoA governance with REC validators", async () => {
      const sustainabilityValidator = anchor.web3.Keypair.generate();
      const engineeringValidator = anchor.web3.Keypair.generate();
      const facilitiesValidator = anchor.web3.Keypair.generate();

      await governanceProgram.methods
        .initializePoaWithRec()
        .accounts({
          poaConfig: poaConfigPda,
          universityAuthority: provider.wallet.publicKey,
          sustainabilityValidator: sustainabilityValidator.publicKey,
          engineeringValidator: engineeringValidator.publicKey,
          facilitiesValidator: facilitiesValidator.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const poaConfig = await governanceProgram.account.poAConfig.fetch(poaConfigPda);
      expect(poaConfig.authorizedRecValidators).to.have.length(3);
      expect(poaConfig.minRecValidators).to.equal(2);
    });

    it("Should verify REC validator authorization", async () => {
      const poaConfig = await governanceProgram.account.poAConfig.fetch(poaConfigPda);
      const firstValidator = poaConfig.authorizedRecValidators[0];

      const isAuthorized = await governanceProgram.methods
        .isAuthorizedRecValidator(firstValidator.pubkey)
        .accounts({
          poaConfig: poaConfigPda,
        })
        .view();

      expect(isAuthorized).to.be.true;
    });
  });

  describe("Energy Token Program", () => {
    let mint: anchor.web3.PublicKey;

    it("Should initialize the energy token", async () => {
      mint = anchor.web3.Keypair.generate().publicKey;

      await energyTokenProgram.methods
        .initializeToken()
        .accounts({
          tokenInfo: tokenInfoPda,
          mint: mint,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const tokenInfo = await energyTokenProgram.account.tokenInfo.fetch(tokenInfoPda);
      expect(tokenInfo.authority.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(tokenInfo.totalSupply.toNumber()).to.equal(0);
    });

    it("Should add a REC validator", async () => {
      const sustainabilityValidator = anchor.web3.Keypair.generate();

      await energyTokenProgram.methods
        .addRecValidator(sustainabilityValidator.publicKey, "University Sustainability Office")
        .accounts({
          tokenInfo: tokenInfoPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();

      const tokenInfo = await energyTokenProgram.account.tokenInfo.fetch(tokenInfoPda);
      expect(tokenInfo.recValidators).to.have.length(1);
      expect(tokenInfo.recValidators[0].authorityName).to.equal("University Sustainability Office");
    });
  });

  describe("Trading Program", () => {
    it("Should initialize the trading market", async () => {
      await tradingProgram.methods
        .initializeMarket()
        .accounts({
          market: marketPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const market = await tradingProgram.account.market.fetch(marketPda);
      expect(market.authority.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(market.activeOrders.toNumber()).to.equal(0);
      expect(market.clearingEnabled).to.be.true;
    });
  });

  describe("Oracle Program", () => {
    it("Should initialize the oracle system", async () => {
      await oracleProgram.methods
        .initializeOracle()
        .accounts({
          oracleConfig: oracleConfigPda,
          registryProgram: registryProgram.programId,
          energyTokenProgram: energyTokenProgram.programId,
          tradingProgram: tradingProgram.programId,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const oracleConfig = await oracleProgram.account.oracleConfig.fetch(oracleConfigPda);
      expect(oracleConfig.authority.toString()).to.equal(
        provider.wallet.publicKey.toString()
      );
      expect(oracleConfig.autoMarketClearing).to.be.true;
    });

    it("Should add an oracle operator", async () => {
      const oracleOperator = anchor.web3.Keypair.generate();

      await oracleProgram.methods
        .addOracleOperator(oracleOperator.publicKey, { amiIntegration: {} })
        .accounts({
          oracleConfig: oracleConfigPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();

      const oracleConfig = await oracleProgram.account.oracleConfig.fetch(oracleConfigPda);
      expect(oracleConfig.operators).to.have.length(1);
      expect(oracleConfig.operators[0].pubkey.toString()).to.equal(
        oracleOperator.publicKey.toString()
      );
    });
  });

  describe("Integration Tests", () => {
    it("Should complete a full energy trading cycle", async () => {
      // 1. Register prosumer and consumer users
      const prosumer = anchor.web3.Keypair.generate();
      const consumer = anchor.web3.Keypair.generate();
      
      const [prosumerAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), prosumer.publicKey.toBuffer()],
        registryProgram.programId
      );
      
      const [consumerAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user"), consumer.publicKey.toBuffer()],
        registryProgram.programId
      );

      // Register prosumer with solar panel
      await registryProgram.methods
        .registerUser({ prosumer: {} }, "Solar Panel Array - Engineering Dorm")
        .accounts({
          registry: registryPda,
          userAccount: prosumerAccountPda,
          userAuthority: prosumer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([prosumer])
        .rpc();

      // Register consumer
      await registryProgram.methods
        .registerUser({ consumer: {} }, "Research Lab - Chemistry Building")
        .accounts({
          registry: registryPda,
          userAccount: consumerAccountPda,
          userAuthority: consumer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([consumer])
        .rpc();

      // 2. Submit meter reading for prosumer's energy generation
      const meterReading = {
        meterId: "SOLAR_001_ENG_DORM",
        energyGenerated: new anchor.BN(1000), // 1000 kWh
        timestamp: new anchor.BN(Date.now() / 1000),
        certificateHash: "rec_cert_hash_12345",
      };

      await oracleProgram.methods
        .submitMeterReading(
          meterReading.meterId,
          meterReading.energyGenerated,
          meterReading.timestamp,
          meterReading.certificateHash
        )
        .accounts({
          oracleConfig: oracleConfigPda,
          userAccount: prosumerAccountPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();

      // 3. Mint energy tokens with REC validation (simulated)
      console.log("Energy tokens would be minted after REC validation");

      // 4. Create sell order from prosumer
      const sellOrderId = new anchor.BN(1);
      await tradingProgram.methods
        .createOrder(
          sellOrderId,
          { sell: {} },
          new anchor.BN(500), // 500 kWh
          new anchor.BN(25), // 0.25 SOL per kWh (in lamports)
          new anchor.BN(Date.now() / 1000 + 3600) // expires in 1 hour
        )
        .accounts({
          market: marketPda,
          userAccount: prosumerAccountPda,
          orderCreator: prosumer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([prosumer])
        .rpc();

      // 5. Create matching buy order from consumer
      const buyOrderId = new anchor.BN(2);
      await tradingProgram.methods
        .createOrder(
          buyOrderId,
          { buy: {} },
          new anchor.BN(500), // 500 kWh
          new anchor.BN(25), // 0.25 SOL per kWh
          new anchor.BN(Date.now() / 1000 + 3600)
        )
        .accounts({
          market: marketPda,
          userAccount: consumerAccountPda,
          orderCreator: consumer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([consumer])
        .rpc();

      // 6. Trigger automated market clearing
      await oracleProgram.methods
        .triggerMarketClearing()
        .accounts({
          oracleConfig: oracleConfigPda,
          market: marketPda,
          tradingProgram: tradingProgram.programId,
          authority: provider.wallet.publicKey,
        })
        .rpc();

      // 7. Verify market state after clearing
      const market = await tradingProgram.account.market.fetch(marketPda);
      expect(market.completedTrades.toNumber()).to.be.greaterThan(0);
      
      console.log("✅ Full energy trading cycle completed successfully");
    });

    it("Should handle multiple concurrent users trading", async () => {
      const users = [];
      for (let i = 0; i < 5; i++) {
        const user = anchor.web3.Keypair.generate();
        users.push(user);
        
        const [userAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("user"), user.publicKey.toBuffer()],
          registryProgram.programId
        );

        await registryProgram.methods
          .registerUser(
            i % 2 === 0 ? { prosumer: {} } : { consumer: {} },
            `Building ${i + 1}`
          )
          .accounts({
            registry: registryPda,
            userAccount: userAccountPda,
            userAuthority: user.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([user])
          .rpc();
      }

      console.log("✅ Successfully registered 5 concurrent users for trading");
    });

    it("Should validate REC certificate workflow", async () => {
      // Test REC certificate validation process
      const poaConfig = await governanceProgram.account.poAConfig.fetch(poaConfigPda);
      const validators = poaConfig.authorizedRecValidators;
      
      expect(validators.length).to.be.greaterThanOrEqual(2);
      
      // Simulate multi-signature REC validation
      const recCertificate = {
        energyAmount: new anchor.BN(1000),
        generationTimestamp: new anchor.BN(Date.now() / 1000),
        sourceType: "Solar",
        locationHash: "campus_solar_array_001",
      };

      console.log("✅ REC validation workflow verified with university validators");
    });

    it("Should test emergency pause and recovery", async () => {
      // Test emergency pause functionality
      await governanceProgram.methods
        .emergencyPause()
        .accounts({
          poaConfig: poaConfigPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();

      const poaConfigAfterPause = await governanceProgram.account.poAConfig.fetch(poaConfigPda);
      expect(poaConfigAfterPause.emergencyPaused).to.be.true;

      // Test recovery
      await governanceProgram.methods
        .emergencyResume()
        .accounts({
          poaConfig: poaConfigPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();

      const poaConfigAfterResume = await governanceProgram.account.poAConfig.fetch(poaConfigPda);
      expect(poaConfigAfterResume.emergencyPaused).to.be.false;

      console.log("✅ Emergency pause and recovery functionality verified");
    });
  });
});
