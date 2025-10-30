// Four.meme ABI
export const fourMemeAbi = [
    // Token creation
    'function createToken(bytes args, bytes signature) payable',

    // Buy functions (using distinct signatures to avoid overload conflicts)
    'function buyToken(address token, uint256 amount, uint256 maxFunds) payable',
    'function buyTokenAMAP(address token, uint256 funds, uint256 minAmount) payable',

    // Sell functions (simplified but unique)
    'function sellToken(address token, uint256 amount, uint256 minFunds)',

    // Events (optional but useful for monitoring)
    'event TokenCreate(address creator, address token, uint256 requestId, string name, string symbol, uint256 totalSupply, uint256 launchTime, uint256 launchFee)',
    'event TokenPurchase(address token, address account, uint256 price, uint256 amount, uint256 cost, uint256 fee, uint256 offers, uint256 funds)',
    'event TokenSale(address token, address account, uint256 price, uint256 amount, uint256 cost, uint256 fee, uint256 offers, uint256 funds)',
    'event TradeStop(address token)'
];

export const pancakeswapRouterAbi = [
    'function getAmountsOut(uint amountIn, address[] memory path) view returns (uint[] memory amounts)',
    'function swapExactTokensForTokens(uint amountIn,uint amountOutMin,address[] calldata path,address to,uint deadline) payable returns (uint[] memory amounts)',
    'function swapExactETHForTokens(uint amountOutMin,address[] calldata path,address to,uint deadline) payable returns (uint[] memory amounts)',
    'function swapExactTokensForETH(uint amountIn,uint amountOutMin,address[] calldata path,address to,uint deadline) payable returns (uint[] memory amounts)',
];

// Swap event ABI (PancakeSwap Pair contract)
export const pancakeSwapPairABI = [
    'event Swap(address indexed sender, uint256 amount0In, uint256 amount1In, uint256 amount0Out, uint256 amount1Out, address indexed to)',
    'function token0() view returns (address)',
    'function token1() view returns (address)'
];