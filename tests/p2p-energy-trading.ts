import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Registry } from "../target/types/registry";
import { EnergyToken } from "../target/types/energy_token";
import { Oracle } from "../target/types/oracle";
import { Trading } from "../target/types/trading";
import { Governance } from "../target/types/governance";

describe("p2p-energy-trading", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const registryProgram = anchor.workspace.Registry as Program<Registry>;
  const energyTokenProgram = anchor.workspace.EnergyToken as Program<EnergyToken>;
  const oracleProgram = anchor.workspace.Oracle as Program<Oracle>;
  const tradingProgram = anchor.workspace.Trading as Program<Trading>;
  const governanceProgram = anchor.workspace.Governance as Program<Governance>;

  it("Initializes all programs!", async () => {
    console.log("Starting P2P Energy Trading System Tests");

    try {
      // Test Registry Program
      const registryTx = await registryProgram.methods.initialize().rpc();
      console.log("✅ Registry initialized:", registryTx);

      // Test Energy Token Program
      const energyTokenTx = await energyTokenProgram.methods.initialize().rpc();
      console.log("✅ Energy Token initialized:", energyTokenTx);

      // Test Oracle Program
      const oracleTx = await oracleProgram.methods.initialize().rpc();
      console.log("✅ Oracle initialized:", oracleTx);

      // Test Trading Program
      const tradingTx = await tradingProgram.methods.initialize().rpc();
      console.log("✅ Trading initialized:", tradingTx);

      // Test Governance Program
      const governanceTx = await governanceProgram.methods.initialize().rpc();
      console.log("✅ Governance initialized:", governanceTx);

      console.log("All P2P Energy Trading programs initialized successfully!");
    } catch (error) {
      console.error("❌ Error during initialization:", error);
      throw error;
    }
  });
});
