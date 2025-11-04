// src/index.ts
import { Wallet, Contract, Interface, Log } from 'ethers';
import { PANCAKE_SWAP_ROUTER_ADDR, COPY_KEYS, TARGET_WALLETS } from '../config';
import { fetchTransactionWithRetry, sleep } from '../utils';
import { executeCopyTradeOnPancakeswap } from './tradeExecutorOnPancakeswap';
import { pancakeSwapPairABI, pancakeswapRouterAbi } from '../abi';
import { wsProvider, httpProvider } from '../provider';

export const pancakeSwapBot = async () => {

    // Router contract (for executing trades)
    const routerContract = new Contract(PANCAKE_SWAP_ROUTER_ADDR, pancakeswapRouterAbi, httpProvider);
    const routerIface = new Interface(pancakeswapRouterAbi);

    const pairIface = new Interface(pancakeSwapPairABI);

    // Compute swapTopic
    const swapFragment = pairIface.getEvent('Swap');
    if (!swapFragment) throw new Error("Swap event not found");
    const swapTopic = swapFragment.topicHash;

    // Copy wallets
    const copyWallets = COPY_KEYS.map(k => new Wallet(k, wsProvider));

    console.log('🚀 Pancakeswap Copy-trading socket is running...');

    // Subscribe to Swap logs across all pairs
    wsProvider.on({ topics: [swapTopic] }, async (log: Log) => {
        try {

        } catch (err) {
            console.error("❌ Listener error:", err);
        }
    });

}