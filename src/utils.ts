import { JsonRpcProvider, WebSocketProvider, TransactionResponse } from 'ethers';

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export async function fetchTransactionWithRetry(
    provider: JsonRpcProvider | WebSocketProvider,
    txHash: string,
    retries = 3,
    wait = 200
): Promise<TransactionResponse | null> {

    for (let i = 0; i <= retries; i++) {
        try {
            const tx = await provider.getTransaction(txHash);
            if (tx) {
                return tx;
            }
        } catch (error) {
            console.log("❌ failed to fetch transaction", i, "retries", error);
        }
        await sleep(wait);
    }
    console.log("❌ failed to fetch transaction after", retries, "retries");
    return null;
}

export function bpsToPercent(bps: number): number {
    return bps / 10000;
}
