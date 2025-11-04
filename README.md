# 🤖 BNB Copy Trading Bot

A production-ready copy trading bot that automatically mirrors trades from target wallets on **PancakeSwap** and **Four.meme** DEX on the Binance Smart Chain (BSC). Built with real-time event monitoring, intelligent gas management, and comprehensive error handling. Feel free to reach out of me when you have any question [Telegram: https://t.me/DevCutup, Whatsapp: https://wa.me/13137423660].


## ✨ Features

### 🎯 **Multi-DEX Support**
- **PancakeSwap**: Monitors and copies swaps across all trading pairs
- **Four.meme**: Supports token purchase and sale events
- Runs both bots simultaneously for comprehensive coverage

### ⚡ **Real-Time Event Monitoring**
- WebSocket-based event listeners for instant trade detection
- Efficient topic filtering to reduce unnecessary processing
- Automatic transaction fetching with retry logic

### 🛡️ **Safety & Reliability**
- **Slippage Protection**: Configurable slippage tolerance (default 3%)
- **Gas Management**: Intelligent gas price calculation with multiplier support
- **Balance Checks**: Automatic validation before executing trades
- **Error Handling**: Comprehensive try-catch blocks with detailed logging

### 💼 **Advanced Trading Features**
- **Multi-Wallet Support**: Execute copy trades across multiple wallets simultaneously
- **Percentage Scaling**: Copy trades at configurable percentage of original trade size
- **Smart Amount Calculation**: Automatic balance adjustment when funds are insufficient
- **Token Approval Management**: Automatic ERC20 token approvals when needed

### 📊 **Production-Ready**
- Structured logging with Winston
- Environment-based configuration
- TypeScript for type safety
- Comprehensive error handling and recovery


## How It Works

1. **Event Detection**: The bot subscribes to blockchain events via WebSocket providers
   - PancakeSwap: Monitors `Swap` events from all liquidity pair contracts
   - Four.meme: Monitors `TokenPurchase` and `TokenSale` events from the contract

2. **Trade Analysis**: When a trade is detected:
   - Validates the transaction sender against target wallet list
   - Decodes the transaction to extract trading parameters
   - Determines trade direction (buy/sell) and token paths

3. **Copy Execution**: For each configured copy wallet:
   - Scales the trade amount based on `COPY_PERCENT`
   - Calculates optimal gas price with multiplier
   - Checks wallet balance and token allowances
   - Executes the copy trade with slippage protection

4. **Transaction Management**:
   - Estimates gas requirements
   - Applies gas price multiplier for faster confirmation
   - Handles nonce management automatically
   - Provides BSCScan links for transaction tracking


## 📦 Installation

### Prerequisites

- Node.js >= 18.x
- npm or yarn
- BSC RPC endpoint (WebSocket preferred for real-time monitoring)
- Private keys for copy trading wallets

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/cutupdev/BNB-Copytrading-Bot.git
   cd BNB-Copytrading-Bot
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Configure environment variables**
   ```bash
   cp .env.example .env
   ```
   Edit `.env` with your configuration (see [Configuration](#-configuration) section)

4. **Build the project**
   ```bash
   npm run build
   ```


## Configuration Details

- **COPY_PERCENT**: Multiply trade size by this value (1.0 = 100%, 0.5 = 50%)
- **SLIPPAGE_BPS**: Maximum acceptable slippage (300 = 3%, 500 = 5%)
- **GAS_MULTIPLIER**: Multiply current gas price by this value for faster confirmation
- **TARGET_WALLETS**: Only trades from these wallets will be copied


## 📁 Project Structure

```
BNB-Copytrading-Bot-Typescript-Private/
├── src/
│   ├── abi/                    # Contract ABIs
│   │   └── index.ts
│   ├── config.ts              # Environment configuration
│   ├── provider/              # Blockchain providers
│   │   └── index.ts
│   ├── utils.ts               # Utility functions
│   ├── index.ts               # Main entry point
│   ├── pancakeSwap/           # PancakeSwap bot implementation
│   │   ├── index.ts           # Event listener
│   │   └── tradeExecutorOnPancakeswap.ts
│   └── fourmeme/              # Four.meme bot implementation
│       ├── index.ts           # Event listener
│       └── tradeExecutorOnFourmeme.ts
├── package.json
├── tsconfig.json
└── README.md
```


## Contact Information

- **X (Twitter)**: [@devcutup](https://twitter.com/devcutup)
- **Telegram**: [@DevCutup](https://t.me/DevCutup)
- **WhatsApp**: [Contact via WhatsApp](https://wa.me/13137423660)
