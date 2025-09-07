#!/usr/bin/env node
/**
 * Oracle Simulator for P2P Energy Trading Platform
 * Enhanced with contract interaction capabilities
 */

const winston = require('winston');
const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');
require('dotenv').config();

// Configure logging
const logger = winston.createLogger({
    level: 'info',
    format: winston.format.combine(
        winston.format.timestamp(),
        winston.format.json()
    ),
    transports: [
        new winston.transports.Console(),
    ],
});

class OracleSimulator {
    constructor() {
        this.wsUrl = process.env.SUBSTRATE_WS_URL || 'ws://localhost:9944';
        this.kafkaServers = process.env.KAFKA_BOOTSTRAP_SERVERS || 'localhost:9092';
        this.oracleAccountSeed = process.env.ORACLE_ACCOUNT_SEED || '//Alice';
        this.processingInterval = parseInt(process.env.PROCESSING_INTERVAL) || 60; // seconds
        this.contractAddressesPath = process.env.CONTRACT_ADDRESSES_PATH || '/tmp/contract_addresses';
        
        this.contractAddresses = null;
        this.setupComplete = false;
        this.intervalId = null;
    }
    
    async initialize() {
        logger.info('Initializing Oracle Simulator...');
        
        try {
            // Wait for contracts to be deployed
            await this.waitForContracts();
            
            // Load contract addresses
            await this.loadContractAddresses();
            
            // Setup demo data
            await this.setupDemoData();
            
            logger.info('Oracle Simulator initialized successfully');
        } catch (error) {
            logger.error('Failed to initialize Oracle Simulator:', error);
            throw error;
        }
    }
    
    async waitForContracts() {
        logger.info('Waiting for contract deployment to complete...');
        
        const deploymentSummaryPath = path.join(this.contractAddressesPath, 'deployment_summary.json');
        
        while (!fs.existsSync(deploymentSummaryPath)) {
            logger.info('Waiting for contracts to be deployed...');
            await new Promise(resolve => setTimeout(resolve, 5000));
        }
        
        logger.info('✅ Contracts are deployed and ready!');
    }
    
    async loadContractAddresses() {
        try {
            const deploymentSummaryPath = path.join(this.contractAddressesPath, 'deployment_summary.json');
            const deploymentData = JSON.parse(fs.readFileSync(deploymentSummaryPath, 'utf8'));
            
            this.contractAddresses = deploymentData.deployment_summary.contracts;
            
            logger.info('Contract addresses loaded:', {
                registry: this.contractAddresses.registry.address,
                grid_token: this.contractAddresses.grid_token.address,
                trading: this.contractAddresses.trading.address,
                oracle_client: this.contractAddresses.oracle_client.address
            });
            
        } catch (error) {
            logger.error('Failed to load contract addresses:', error);
            throw error;
        }
    }
    
    async setupDemoData() {
        if (this.setupComplete) {
            logger.info('Demo data already setup, skipping...');
            return;
        }
        
        logger.info('Setting up demo data...');
        
        try {
            const result = await this.executeShellScript('setup');
            logger.info('Demo data setup completed');
            this.setupComplete = true;
        } catch (error) {
            logger.error('Failed to setup demo data:', error);
            // Continue without demo data
        }
    }
    
    async executeShellScript(command) {
        return new Promise((resolve, reject) => {
            const script = spawn('./interact_contracts.sh', [command], {
                stdio: ['inherit', 'pipe', 'pipe']
            });
            
            let output = '';
            let errorOutput = '';
            
            script.stdout.on('data', (data) => {
                output += data.toString();
            });
            
            script.stderr.on('data', (data) => {
                errorOutput += data.toString();
            });
            
            script.on('close', (code) => {
                if (code === 0) {
                    resolve(output);
                } else {
                    reject(new Error(`Script failed with code ${code}: ${errorOutput}`));
                }
            });
        });
    }
    
    async executeMarketClearing() {
        try {
            logger.info('Executing market clearing...');
            
            // Execute market clearing
            const clearResult = await this.executeShellScript('clear-market');
            logger.info('Market clearing executed successfully');
            
            return true;
        } catch (error) {
            logger.error('Failed to execute market clearing:', error);
            return false;
        }
    }
    
    async checkSystemStatus() {
        try {
            logger.info('Checking system status...');
            
            const statusResult = await this.executeShellScript('status');
            logger.info('System status checked successfully');
            
            return true;
        } catch (error) {
            logger.error('Failed to check system status:', error);
            return false;
        }
    }
    
    async run() {
        try {
            await this.initialize();
            
            logger.info(`🚀 Oracle Simulator is running with ${this.processingInterval}s intervals`);
            
            // Perform initial market clearing
            await this.executeMarketClearing();
            
            // Set up periodic market clearing
            this.intervalId = setInterval(async () => {
                try {
                    logger.info('⏰ Periodic market clearing started');
                    await this.executeMarketClearing();
                    await this.checkSystemStatus();
                } catch (error) {
                    logger.error('Error in periodic market clearing:', error);
                }
            }, this.processingInterval * 1000);
            
            // Keep the process running
            process.on('SIGINT', () => {
                logger.info('Received SIGINT, gracefully shutting down...');
                if (this.intervalId) {
                    clearInterval(this.intervalId);
                }
                process.exit(0);
            });
            
        } catch (error) {
            logger.error('Failed to start Oracle Simulator:', error);
            process.exit(1);
        }
    }
}

// Start the Oracle Simulator
if (require.main === module) {
    const simulator = new OracleSimulator();
    simulator.run().catch(error => {
        logger.error('Fatal error:', error);
        process.exit(1);
    });
}

module.exports = OracleSimulator;
