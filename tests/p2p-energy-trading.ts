import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Registry } from "../target/types/registry";
import { EnergyToken } from "../target/types/energy_token";
import { Oracle } from "../target/types/oracle";
import { Trading } from "../target/types/trading";
import { Governance } from "../target/types/governance";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("p2p-energy-trading", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const provider = anchor.getProvider();

  const registryProgram = anchor.workspace.Registry as Program<Registry>;
  const energyTokenProgram = anchor.workspace.EnergyToken as Program<EnergyToken>;
  const oracleProgram = anchor.workspace.Oracle as Program<Oracle>;
  const tradingProgram = anchor.workspace.Trading as Program<Trading>;
  const governanceProgram = anchor.workspace.Governance as Program<Governance>;

  // Generate keypairs for testing
  const authority = (provider.wallet as any).payer as Keypair;
  const apiGateway = Keypair.generate();

  it("Initializes all programs!", async () => {
    console.log("Starting P2P Energy Trading System Tests");

    try {
      // Airdrop SOL to authority for funding accounts
      await provider.connection.requestAirdrop(authority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
      await provider.connection.requestAirdrop(apiGateway.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);

      // Wait for airdrop confirmations
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Get PDAs for all programs
      const [registryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("registry")],
        registryProgram.programId
      );

      const [oracleDataPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("oracle_data")],
        oracleProgram.programId
      );

      // Test Registry Program
      const registryTx = await registryProgram.methods.initialize()
        .rpc();
      console.log("✅ Registry initialized:", registryTx);

      // Test Energy Token Program
      const energyTokenTx = await energyTokenProgram.methods.initialize()
        .rpc();
      console.log("✅ Energy Token initialized:", energyTokenTx);

      // Test Oracle Program
      const oracleTx = await oracleProgram.methods.initialize(apiGateway.publicKey)
        .rpc();
      console.log("✅ Oracle initialized:", oracleTx);

      // Test Trading Program
      const tradingTx = await tradingProgram.methods.initialize()
        .rpc();
      console.log("✅ Trading initialized:", tradingTx);

      // Test Governance Program
      const governanceTx = await governanceProgram.methods.initialize()
        .rpc();
      console.log("✅ Governance initialized:", governanceTx);

      console.log("All P2P Energy Trading programs initialized successfully!");
    } catch (error) {
      console.error("❌ Error during initialization:", error);
      throw error;
    }
  });
});
