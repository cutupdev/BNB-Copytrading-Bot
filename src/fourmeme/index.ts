// src/fourmeme.ts — Four.meme copy-trader with topic filter

import { Wallet, Interface, Log } from 'ethers';
import { COPY_KEYS, TARGET_WALLETS, FOUR_MEME_ADDRESS } from '../config';
import { fetchTransactionWithRetry, sleep } from '../utils';
import { executeCopyTradeOnFourMeme } from './tradeExecutorOnFourmeme';
import { fourMemeAbi } from '../abi';
import { wsProvider, httpProvider } from '../provider';

export const fourmemeBot = async () => {

  const fourMemeIface = new Interface(fourMemeAbi);

  // Compute function selectors (ethers v6)
  const buyTopic = fourMemeIface.getEvent('TokenPurchase');
  if (!buyTopic) throw new Error("Buy function not found");
  const buyTopicHash = buyTopic.topicHash;
  const sellTopic = fourMemeIface.getEvent('TokenSale');
  if (!sellTopic) throw new Error("Sell function not found");
  const sellTopicHash = sellTopic.topicHash;

  const copyWallets = COPY_KEYS.map(k => new Wallet(k, wsProvider));

  console.log('🚀 Four.meme Copy-Trading socket is running...');

  // Subscribe to all logs from four.meme contract
  wsProvider.on(
    {
      address: FOUR_MEME_ADDRESS,
      topics: [[buyTopicHash, sellTopicHash]],
    },
    async (log: Log) => {
      try {
        switch (name) {
          case 'TokenPurchase':
            tokenAddress = args.token;
            // Use actual BNB spent by the target wallet
            amountIn = args.cost;
            isBuy = true;
            break;
          case 'TokenSale':
            tokenAddress = args.token;
            // Token amount sold by target wallet
            amountIn = args.amount;
            isBuy = false;
            break;
          default:
            return;
        }

        console.log(`💰 ${isBuy ? 'Buy' : 'Sell'} detected for ${tokenAddress}`);

        await Promise.all(
          copyWallets.map(async (wallet, idx) => {
            await sleep(idx * 50);
            await executeCopyTradeOnFourMeme(wallet, FOUR_MEME_ADDRESS, tokenAddress, amountIn, isBuy, tx.hash);
          })
        );
      } catch (err) {
        console.error('❌ Listener error:', err);
        console.log(`------🎯 Four.meme event listener failed ------`);
      }
    }
  );

}