# Quick Start Examples for Alternative Frameworks

## 1. Hyperledger Fabric Setup

### Network Configuration (docker-compose.yml)
```yaml
version: '2'

networks:
  smart-grid:

services:
  orderer.university.com:
    container_name: orderer.university.com
    image: hyperledger/fabric-orderer:2.4
    environment:
      - FABRIC_LOGGING_SPEC=INFO
      - ORDERER_GENERAL_LISTENADDRESS=0.0.0.0
      - ORDERER_GENERAL_BOOTSTRAPMETHOD=file
      - ORDERER_GENERAL_BOOTSTRAPFILE=/var/hyperledger/orderer/orderer.genesis.block
    networks:
      - smart-grid

  peer0.students.university.com:
    container_name: peer0.students.university.com
    image: hyperledger/fabric-peer:2.4
    environment:
      - CORE_PEER_ID=peer0.students.university.com
      - CORE_PEER_ADDRESS=peer0.students.university.com:7051
      - CORE_PEER_LOCALMSPID=StudentsMSP
    networks:
      - smart-grid
```

### Smart Contract (Go)
```go
package main

import (
    "encoding/json"
    "fmt"
    "github.com/hyperledger/fabric-contract-api-go/contractapi"
)

type SmartGridContract struct {
    contractapi.Contract
}

type EnergyMeter struct {
    MeterID     string  `json:"meterID"`
    OwnerID     string  `json:"ownerID"`
    Location    string  `json:"location"`
    EnergyGen   float64 `json:"energyGenerated"`
    EnergyConsumed float64 `json:"energyConsumed"`
}

func (sgc *SmartGridContract) RegisterMeter(ctx contractapi.TransactionContextInterface, 
    meterID string, ownerID string, location string) error {
    
    meter := EnergyMeter{
        MeterID:  meterID,
        OwnerID:  ownerID,
        Location: location,
        EnergyGen: 0,
        EnergyConsumed: 0,
    }
    
    meterJSON, err := json.Marshal(meter)
    if err != nil {
        return err
    }
    
    return ctx.GetStub().PutState(meterID, meterJSON)
}

func (sgc *SmartGridContract) UpdateEnergyData(ctx contractapi.TransactionContextInterface,
    meterID string, generated float64, consumed float64) error {
    
    meterJSON, err := ctx.GetStub().GetState(meterID)
    if err != nil {
        return fmt.Errorf("failed to read meter %s: %v", meterID, err)
    }
    
    var meter EnergyMeter
    err = json.Unmarshal(meterJSON, &meter)
    if err != nil {
        return err
    }
    
    meter.EnergyGen = generated
    meter.EnergyConsumed = consumed
    
    updatedMeterJSON, err := json.Marshal(meter)
    if err != nil {
        return err
    }
    
    return ctx.GetStub().PutState(meterID, updatedMeterJSON)
}
```

## 2. Ethereum POA Setup

### Genesis Configuration (genesis.json)
```json
{
  "config": {
    "chainId": 1337,
    "homesteadBlock": 0,
    "eip150Block": 0,
    "eip155Block": 0,
    "eip158Block": 0,
    "byzantiumBlock": 0,
    "constantinopleBlock": 0,
    "petersburgBlock": 0,
    "clique": {
      "period": 15,
      "epoch": 30000
    }
  },
  "difficulty": "0x1",
  "gasLimit": "0x8000000",
  "extradata": "0x0000000000000000000000000000000000000000000000000000000000000000[VALIDATOR_ADDRESSES]0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "alloc": {}
}
```

### Smart Contract (Solidity)
```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SmartGridTrading {
    
    struct EnergyMeter {
        address owner;
        string location;
        uint256 energyGenerated;
        uint256 energyConsumed;
        bool isActive;
    }
    
    struct EnergyTrade {
        address seller;
        address buyer;
        uint256 amount;
        uint256 pricePerUnit;
        uint256 timestamp;
        bool completed;
    }
    
    mapping(string => EnergyMeter) public meters;
    mapping(uint256 => EnergyTrade) public trades;
    mapping(address => uint256) public energyBalance;
    
    uint256 public tradeCounter;
    
    event MeterRegistered(string meterID, address owner, string location);
    event EnergyDataUpdated(string meterID, uint256 generated, uint256 consumed);
    event TradeCreated(uint256 tradeID, address seller, uint256 amount, uint256 price);
    event TradeCompleted(uint256 tradeID, address buyer);
    
    function registerMeter(string memory meterID, string memory location) public {
        require(meters[meterID].owner == address(0), "Meter already registered");
        
        meters[meterID] = EnergyMeter({
            owner: msg.sender,
            location: location,
            energyGenerated: 0,
            energyConsumed: 0,
            isActive: true
        });
        
        emit MeterRegistered(meterID, msg.sender, location);
    }
    
    function updateEnergyData(string memory meterID, uint256 generated, uint256 consumed) public {
        require(meters[meterID].owner == msg.sender, "Only meter owner can update data");
        require(meters[meterID].isActive, "Meter is not active");
        
        meters[meterID].energyGenerated = generated;
        meters[meterID].energyConsumed = consumed;
        
        // Update energy balance
        if (generated > consumed) {
            energyBalance[msg.sender] += (generated - consumed);
        }
        
        emit EnergyDataUpdated(meterID, generated, consumed);
    }
    
    function createEnergyTrade(uint256 amount, uint256 pricePerUnit) public {
        require(energyBalance[msg.sender] >= amount, "Insufficient energy balance");
        
        trades[tradeCounter] = EnergyTrade({
            seller: msg.sender,
            buyer: address(0),
            amount: amount,
            pricePerUnit: pricePerUnit,
            timestamp: block.timestamp,
            completed: false
        });
        
        energyBalance[msg.sender] -= amount;
        
        emit TradeCreated(tradeCounter, msg.sender, amount, pricePerUnit);
        tradeCounter++;
    }
    
    function buyEnergy(uint256 tradeID) public payable {
        EnergyTrade storage trade = trades[tradeID];
        require(!trade.completed, "Trade already completed");
        require(trade.seller != msg.sender, "Cannot buy your own energy");
        
        uint256 totalCost = trade.amount * trade.pricePerUnit;
        require(msg.value >= totalCost, "Insufficient payment");
        
        trade.buyer = msg.sender;
        trade.completed = true;
        
        energyBalance[msg.sender] += trade.amount;
        payable(trade.seller).transfer(totalCost);
        
        // Refund excess payment
        if (msg.value > totalCost) {
            payable(msg.sender).transfer(msg.value - totalCost);
        }
        
        emit TradeCompleted(tradeID, msg.sender);
    }
}
```

## 3. Polygon Edge Setup

### Initialize Network
```bash
# Create validator keys
polygon-edge polybft-secrets --data-dir node1
polygon-edge polybft-secrets --data-dir node2
polygon-edge polybft-secrets --data-dir node3
polygon-edge polybft-secrets --data-dir node4

# Create genesis
polygon-edge genesis \
    --consensus polybft \
    --validators node1:node2:node3:node4 \
    --chain-id 1337 \
    --block-gas-limit 20000000

# Start nodes
polygon-edge server --data-dir ./node1 --chain genesis.json --port 10001 --grpc-address 0.0.0.0:9632
```

## 4. Quick Comparison Commands

### Build and Deploy
```bash
# Hyperledger Fabric
./network.sh up createChannel -c smartgrid
./network.sh deployCC -ccn energy -ccp ./chaincode/energy -ccl go

# Ethereum POA
geth --datadir ./node1 init genesis.json
geth --datadir ./node1 --networkid 1337 --http --http.corsdomain "*" --mine

# Polygon Edge
polygon-edge server --data-dir ./node1 --chain genesis.json

# Substrate (Current)
cargo build --release
./target/release/node-template --dev
```

Each framework has its trade-offs. Would you like me to dive deeper into any specific framework or help you set up a prototype with one of these alternatives?
