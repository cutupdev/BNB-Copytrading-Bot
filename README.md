# Perpetual DEX Smart Contract

A comprehensive Solana smart contract for perpetual futures trading with orderbook, margin trading, and leverage support. Similar to Axiom.trade functionality, this contract enables decentralized perpetual trading on Solana. Feel free to reach out of me when you have any question [Telegram: https://t.me/DevCutup, Whatsapp: https://wa.me/13137423660].

## Overview

This smart contract provides a complete perpetual DEX solution on Solana, featuring orderbook-based trading, margin trading with leverage, position management, and automated liquidation. The contract supports both long and short positions with configurable leverage up to 100x.

### Key Features

- **Orderbook Trading**: Limit orders, market orders, stop-loss, and take-profit orders
- **Perpetual Futures**: No expiration dates, trade indefinitely
- **Margin Trading**: Deposit collateral and trade with leverage
- **High Leverage**: Support for up to 100x leverage (configurable)
- **Position Management**: Open, close, add/remove margin from positions
- **Automated Liquidation**: Under-collateralized positions are automatically liquidated
- **PnL Tracking**: Real-time profit and loss calculation
- **Funding Rates**: Perpetual funding mechanism (structure in place)
- **Open Interest Tracking**: Monitor total long and short positions

## Architecture

The contract follows a modular structure similar to production Solana programs:

```
programs/perpetual-dex/src/
├── lib.rs                 # Main program entry point
├── consts.rs              # Constants and enums
├── errors.rs              # Custom error definitions
├── events/                # Event definitions
│   └── events.rs
├── instructions/          # Instruction handlers
│   ├── initialize.rs
│   ├── create_market.rs
│   ├── place_order.rs
│   ├── cancel_order.rs
│   ├── open_position.rs
│   ├── close_position.rs
│   ├── add_margin.rs
│   ├── remove_margin.rs
│   ├── liquidate_position.rs
│   ├── deposit_collateral.rs
│   ├── withdraw_collateral.rs
│   └── update_price.rs
├── states/                # Account state structures
│   ├── global_config.rs
│   ├── market.rs
│   ├── user.rs
│   ├── position.rs
│   └── order.rs
└── utils/                 # Utility functions
    ├── calculations.rs
    ├── margin.rs
    └── liquidation.rs
```

## Smart Contract Instructions

### 1. `initialize`

Initializes the global configuration for the perpetual DEX platform. Must be called once before any markets can be created.

**Parameters:**
- `fee_recipient`: Address that receives trading and liquidation fees
- `trading_fee_bps`: Trading fee in basis points (10000 = 100%)
- `liquidation_fee_bps`: Liquidation fee in basis points
- `max_leverage_bps`: Maximum leverage in basis points (10000 = 100x)
- `initial_margin_bps`: Initial margin requirement in basis points
- `maintenance_margin_bps`: Maintenance margin requirement in basis points
- `liquidation_threshold_bps`: Liquidation threshold in basis points

**Accounts:**
- `global_config`: Global configuration PDA
- `authority`: Authority that can update configuration
- `system_program`: Solana system program

### 2. `create_market`

Creates a new perpetual market for trading.

**Parameters:**
- `base_mint`: Base asset mint (e.g., SOL)
- `quote_mint`: Quote asset mint (e.g., USDC)
- `name`: Market name
- `symbol`: Market symbol
- `initial_price`: Initial price in quote units

**Accounts:**
- `global_config`: Global configuration PDA
- `market_id`: Unique market identifier
- `market`: Market account PDA
- `authority`: Authority creating the market
- `system_program`: Solana system program

**Events:**
- `MarketCreated`: Emitted when a market is created

### 3. `place_order`

Places an order in the orderbook.

**Parameters:**
- `side`: Order side (Buy or Sell)
- `order_type`: Order type (Market, Limit, StopLoss, TakeProfit)
- `size`: Order size in base units
- `price`: Limit price (for limit orders)
- `stop_price`: Stop price (for stop orders, optional)
- `leverage`: Leverage in basis points (e.g., 1000 = 10x)

**Accounts:**
- `global_config`: Global configuration PDA
- `market`: Market account PDA
- `user`: User account PDA
- `order_id`: Unique order identifier
- `order`: Order account PDA
- `trader`: Trader (signer)
- `system_program`: Solana system program

**Events:**
- `OrderPlaced`: Emitted when an order is placed

### 4. `cancel_order`

Cancels an existing order.

**Parameters:**
- `order_id`: Order ID to cancel

**Accounts:**
- `user`: User account PDA
- `order`: Order account PDA
- `trader`: Trader (signer)

**Events:**
- `OrderCancelled`: Emitted when an order is cancelled

### 5. `open_position`

Opens a new perpetual position (long or short).

**Parameters:**
- `side`: Position side (Long or Short)
- `size`: Position size in base units
- `leverage`: Leverage in basis points
- `margin`: Collateral margin amount

**Accounts:**
- `global_config`: Global configuration PDA
- `market`: Market account PDA
- `user`: User account PDA
- `position_id`: Unique position identifier
- `position`: Position account PDA
- `trader`: Trader (signer)
- `system_program`: Solana system program

**Events:**
- `PositionOpened`: Emitted when a position is opened

### 6. `close_position`

Closes an existing perpetual position.

**Parameters:**
- `position_id`: Position ID to close

**Accounts:**
- `global_config`: Global configuration PDA
- `market`: Market account PDA
- `user`: User account PDA
- `position`: Position account PDA
- `trader`: Trader (signer)

**Events:**
- `PositionClosed`: Emitted when a position is closed

### 7. `add_margin`

Adds margin to an existing position to reduce liquidation risk.

**Parameters:**
- `position_id`: Position ID
- `amount`: Amount of margin to add

**Accounts:**
- `user`: User account PDA
- `position`: Position account PDA
- `trader`: Trader (signer)

**Events:**
- `MarginAdded`: Emitted when margin is added

### 8. `remove_margin`

Removes margin from a position (must maintain maintenance margin).

**Parameters:**
- `position_id`: Position ID
- `amount`: Amount of margin to remove

**Accounts:**
- `user`: User account PDA
- `position`: Position account PDA
- `trader`: Trader (signer)

**Events:**
- `MarginRemoved`: Emitted when margin is removed

### 9. `liquidate_position`

Liquidates an under-collateralized position.

**Parameters:**
- `position_id`: Position ID to liquidate

**Accounts:**
- `global_config`: Global configuration PDA
- `market`: Market account PDA
- `user`: User account PDA
- `position`: Position account PDA
- `liquidator`: Liquidator (signer)

**Events:**
- `PositionLiquidated`: Emitted when a position is liquidated

### 10. `deposit_collateral`

Deposits collateral to user account for trading.

**Parameters:**
- `amount`: Amount of collateral to deposit

**Accounts:**
- `user_account`: User account PDA
- `user`: User (signer)
- `system_program`: Solana system program

**Events:**
- `CollateralDeposited`: Emitted when collateral is deposited

### 11. `withdraw_collateral`

Withdraws collateral from user account.

**Parameters:**
- `amount`: Amount of collateral to withdraw

**Accounts:**
- `user_account`: User account PDA
- `user`: User (signer)

**Events:**
- `CollateralWithdrawn`: Emitted when collateral is withdrawn

### 12. `update_price`

Updates the market price (oracle).

**Parameters:**
- `price`: New market price

**Accounts:**
- `market`: Market account PDA
- `oracle`: Oracle or authorized price updater (signer)

**Events:**
- `PriceUpdated`: Emitted when price is updated

## Margin and Leverage

### Margin Requirements

- **Initial Margin**: Minimum margin required to open a position (default: 10%)
- **Maintenance Margin**: Minimum margin to maintain a position (default: 5%)
- **Liquidation Threshold**: Price level at which position is liquidated (default: 4%)

### Leverage

- **Minimum Leverage**: 1x (100 basis points)
- **Maximum Leverage**: 100x (10000 basis points, configurable)
- **Leverage Calculation**: `position_value = margin * leverage`

## Position Types

### Long Position
- Profit when price increases
- Loss when price decreases
- Liquidated when price falls below liquidation price

### Short Position
- Profit when price decreases
- Loss when price increases
- Liquidated when price rises above liquidation price

## Order Types

- **Market Order**: Executes immediately at best available price
- **Limit Order**: Executes at specified price or better
- **Stop Loss**: Triggers when price reaches stop price
- **Take Profit**: Triggers when price reaches target price

## Requirements

- **Anchor Framework**: Version 0.30.1
- **Solana CLI**: Latest version
- **Rust**: Latest stable version
- **Node.js**: 18+ (for tests)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/cutupdev/Solana-Perpetual-Dex-Smart-Contract.git
cd Solana-Perpetual-Dex-Smart-Contract
```

2. Install dependencies:
```bash
anchor build
```

3. Run tests:
```bash
anchor test
```

## Configuration

The contract can be configured through the `initialize` instruction:

- **Trading Fee**: Default configurable (e.g., 0.1% = 10 basis points)
- **Liquidation Fee**: Default configurable (e.g., 2% = 200 basis points)
- **Max Leverage**: Default 100x (10000 basis points)
- **Initial Margin**: Default 10% (1000 basis points)
- **Maintenance Margin**: Default 5% (500 basis points)

## Security Considerations

1. **Access Control**: Only authorized accounts can perform sensitive operations
2. **Margin Validation**: All margin requirements are validated before operations
3. **Liquidation Checks**: Positions are checked for liquidation eligibility
4. **Price Validation**: All prices are validated for correctness
5. **Slippage Protection**: Orders can specify maximum slippage
6. **Pause Mechanism**: Markets can be paused for emergency situations

## Contact Information

- **X (Twitter)**: [@devcutup](https://twitter.com/devcutup)
- **Telegram**: [@DevCutup](https://t.me/DevCutup)
- **WhatsApp**: [Contact via WhatsApp](https://wa.me/13137423660)
