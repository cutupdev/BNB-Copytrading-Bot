// src/tradeExecutor.ts
import { Contract, Wallet, TransactionRequest, ethers, JsonRpcProvider, WebSocketProvider, Transaction, TransactionResponse } from 'ethers';
import { PANCAKE_SWAP_ROUTER_ADDR, COPY_PERCENT, SLIPPAGE_BPS, GAS_MULTIPLIER, MIN_GAS_PRICE_GWEI } from '../config';

export async function executeCopyTradeOnPancakeswap(
    wallet: Wallet,
    routerContract: Contract,
    routerTx: any,                 // may be parsed router call or a fallback object { name: 'swapExactETHForTokens' }
    path: string[],
    amountInOriginal: bigint,
    isEthIn = false,
    txHash: string
) {
    try {

        // Get expected amounts out (may throw if path invalid)
        let amountsOutRaw: any;
        try {
            amountsOutRaw = await routerContract.getAmountsOut(scaledAmount, path);
        } catch (err) {
            console.error('❌ getAmountsOut failed', err);
            return;
        }

        // amountsOutRaw items may be bigint-like
        const expectedOutRaw = amountsOutRaw[amountsOutRaw.length - 1];
        const expectedOut = BigInt(expectedOutRaw?.toString?.() ?? expectedOutRaw ?? '0');

        // compute amountOutMin using SLIPPAGE_BPS (basis points)
        const amountOutMin = expectedOut * BigInt(Math.max(0, 10000 - Number(SLIPPAGE_BPS))) / BigInt(10000);

        const deadline = Math.floor(Date.now() / 1000) + 60; // 60s TTL (same as you used)

        // Build txRequest using populateTransaction. routerTx.name must exist (fallback should provide it)
        const fnName = routerTx?.name ?? (isEthIn ? 'swapExactETHForTokens' : 'swapExactTokensForTokens');
        let txRequest: TransactionRequest;

        try {
        } catch (err) {
            console.error('❌ populateTransaction failed for', fnName, err);
            return;
        }

        // Fee/gas handling
        let feeData;
        try {
            feeData = await provider.getFeeData();
        } catch (err) {
            console.warn('⚠️ provider.getFeeData failed, falling back to default gas price', err);
            feeData = { gasPrice: null as any };
        }

        let gasPrice: bigint;

        try {
            // Directly query live gas price
            const rawGas = await provider.send('eth_gasPrice', []);
            gasPrice = BigInt(rawGas);
        } catch (err) {
            console.warn('⚠️ eth_gasPrice RPC failed, fallback to getFeeData()', err);
            const feeData = await provider.getFeeData();
            gasPrice = feeData?.gasPrice ?? ethers.parseUnits(String(MIN_GAS_PRICE_GWEI), 'gwei');
        }
        
        // Apply multiplier (e.g., 1.15)
        const multiplier = Number(GAS_MULTIPLIER ?? 1);
        const gasPriceScaled = (gasPrice * BigInt(Math.round(multiplier * 100))) / 100n;
        txRequest.gasPrice = gasPriceScaled;
        try {
            // const estimated = await wallet.estimateGas({ ...txRequest, from: wallet.address });
            const estimated = await provider.estimateGas({ ...txRequest });
            txRequest.gasLimit = estimated;

        } catch (err) {
            console.warn('⚠️ estimateGas failed, using fallback gasLimit 800000', err);
            txRequest.gasLimit = BigInt(800000);
        }

        // nonce
        try {
            const nonce = await provider.getTransactionCount(wallet.address, 'pending');
            txRequest.nonce = nonce;
        } catch (err) {
            console.warn('⚠️ getTransactionCount failed, leaving nonce undefined', err);
        }

        // Send transaction
        const sent = await wallet.sendTransaction(txRequest);
        console.log(`Target tx: 🔗 View on BSCScan: https://bscscan.com/tx/${txHash}`);
        console.log(`CopyTrading tx: 🔗 View on BSCScan: https://bscscan.com/tx/${sent.hash}`);
        console.log(`------🎯 Copy-Trading transaction on Pancakeswap completed ------\n\n`);
    } catch (err) {
        console.error('❌ Error executing copy trade on Pancakeswap:', err);
        console.log(`------🎯 Copy-Trading transaction on Pancakeswap failed ------\n\n`);
    }
}
