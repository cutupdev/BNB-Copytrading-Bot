// src/tradeExecutorOnFourmeme.ts
import { ethers, Wallet, Contract, JsonRpcProvider, WebSocketProvider } from 'ethers';
import { COPY_PERCENT, GAS_MULTIPLIER, MIN_GAS_PRICE_GWEI } from '../config';
import { sleep } from '../utils';

const fourMemeAbi = [
    'function buyToken(address token, uint256 amount, uint256 maxFunds) payable',
    'function buyTokenAMAP(address token, uint256 funds, uint256 minAmount) payable',
    'function sellToken(address token, uint256 amount, uint256 minFunds)',
];

export async function executeCopyTradeOnFourMeme(
    wallet: Wallet,
    fourMemeAddress: string,
    tokenAddress: string,
    amountOriginal: bigint,
    isBuy: boolean,
    txHash: string
) {
    try {
        if (!wallet.provider) {
            console.error('❌ executeCopyTradeOnFourMeme: missing provider');
            return;
        }

        const provider = wallet.provider as JsonRpcProvider | WebSocketProvider;
        const contract = new Contract(fourMemeAddress, fourMemeAbi, wallet);

        const percent = Number(COPY_PERCENT);
        if (Number.isNaN(percent) || percent <= 0) {
            console.error('❌ Invalid COPY_PERCENT', COPY_PERCENT);
            return;
        }

        // scaledAmount = amountOriginal * percent (3-decimal precision)
        let scaledAmount = (amountOriginal * BigInt(Math.round(percent * 1000))) / 1000n;
        if (scaledAmount <= 0n) {
            console.error('❌ Scaled amount is zero');
            return;
        }

        // Gas price
        const feeData = await provider.getFeeData();
        let gasPrice = feeData.gasPrice ?? ethers.parseUnits(String(MIN_GAS_PRICE_GWEI), 'gwei');
        const multiplier = Number(GAS_MULTIPLIER ?? 1);
        gasPrice = (gasPrice * BigInt(Math.round(multiplier * 100))) / 100n;

        // Estimate gasLimit (try specific method, fallback)
        let estimatedGas: bigint;
        try {
        } catch (err) {
        }

        const gasCost = estimatedGas * gasPrice;

        // Check wallet balance
        const balance = await provider.getBalance(wallet.address);

        console.log(`💰 Wallet balance: ${ethers.formatEther(balance)} BNB`);
        console.log(`⛽ Estimated gas: ${estimatedGas.toString()} @ ${ethers.formatUnits(gasPrice, 'gwei')} gwei`);
        console.log(`⚙️ Gas cost: ${ethers.formatEther(gasCost)} BNB`);

        if (isBuy) {
        } else {
            
        }

        // build overrides
        const txOverrides: any = {
            gasLimit: estimatedGas,
            gasPrice,
        };

        let tx;
        if (isBuy) {
            txOverrides.value = scaledAmount;
            console.log(`🟢 Copying BUY on ${tokenAddress} with value ${ethers.formatEther(scaledAmount)} ETH/BNB (scaled)`);
            tx = await contract.buyTokenAMAP(tokenAddress, scaledAmount, 0n, txOverrides);
        } else {
            console.log(`🔴 Copying SELL on ${tokenAddress} with amount ${scaledAmount.toString()} tokens (scaled)`);
            tx = await contract.sellToken(tokenAddress, scaledAmount, 0n, txOverrides);
        }

        console.log(`Target tx: 🔗 View on BSCScan: https://bscscan.com/tx/${txHash}`);
        console.log(`CopyTrading tx: 🔗 View on BSCScan: https://bscscan.com/tx/${tx.hash}`);
        console.log(`------🎯 Copy-Trading transaction on Four.meme DEX completed ------\n\n`);
    } catch (err: any) {
        console.error('❌ executeCopyTradeOnFourMeme error:', err.message ?? err);
        console.log(`------🎯 Copy-Trading transaction on Four.meme DEX failed ------\n\n`);
    }
}
