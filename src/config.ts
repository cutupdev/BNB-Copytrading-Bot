import dotenv from 'dotenv';
dotenv.config();

// Provider URLs
export const WS_URL = process.env.WS_URL ?? '';
export const HTTP_URL = process.env.BSC_RPC_URL ?? WS_URL.replace(/^wss:/, 'https:');

// Pancake Swap Router and Factory Addresses
export const PANCAKE_SWAP_ROUTER_ADDR = (process.env.PANCAKE_SWAP_ROUTER_ADDR ?? '').toLowerCase();
export const PANCAKE_SWAP_FACTORY_ADDR = (process.env.PANCAKE_SWAP_FACTORY_ADDR ?? '').toLowerCase();

// Four.meme Address
export const FOUR_MEME_ADDRESS = (process.env.FOUR_MEME_ADDRESS ?? '').toLowerCase();

// Copy Keys and Percentages
export const COPY_KEYS = (process.env.COPY_KEYS ?? '')
  .split(',')
  .map(k => k.trim())
  .filter(Boolean);
export const COPY_PERCENT = parseFloat(process.env.COPY_PERCENT ?? '1.0');

// Slippage and Gas Multiplier
export const SLIPPAGE_BPS = parseInt(process.env.SLIPPAGE_BPS ?? '300');
export const GAS_MULTIPLIER = parseFloat(process.env.GAS_MULTIPLIER ?? '1.0');

// Target Wallets
export const TARGET_WALLETS = (process.env.TARGET_WALLETS ?? '')
  .split(',')
  .map(a => a.trim().toLowerCase())
  .filter(Boolean);

// Minimum Gas Price in GWEI
export const MIN_GAS_PRICE_GWEI = parseFloat(process.env.MIN_GAS_PRICE_GWEI ?? '5');

if (!WS_URL) throw new Error('WS_URL is required');
if (COPY_KEYS.length === 0) throw new Error('COPY_KEYS is required');
if (!PANCAKE_SWAP_ROUTER_ADDR) throw new Error('PANCAKE_SWAP_ROUTER_ADDR is required');
if (!PANCAKE_SWAP_FACTORY_ADDR) throw new Error('PANCAKE_SWAP_FACTORY_ADDR is required');
