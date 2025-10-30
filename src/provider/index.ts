import { ethers } from 'ethers';
import { WS_URL, HTTP_URL } from '../config';

export const wsProvider = new ethers.WebSocketProvider(WS_URL);
export const httpProvider = new ethers.JsonRpcProvider(HTTP_URL);