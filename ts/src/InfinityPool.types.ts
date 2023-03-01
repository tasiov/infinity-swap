/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type Addr = string;
export interface ConfigResponse {
  config: Config;
}
export interface Config {
  denom: string;
  developer?: Addr | null;
  marketplace_addr: Addr;
}
export type ExecuteMsg = {
  create_pool: {
    asset_recipient?: string | null;
    bonding_curve: BondingCurve;
    collection: string;
    delta: Uint128;
    finders_fee_bps: number;
    pool_type: PoolType;
    reinvest_nfts: boolean;
    reinvest_tokens: boolean;
    spot_price: Uint128;
    swap_fee_bps: number;
  };
} | {
  deposit_tokens: {
    pool_id: number;
  };
} | {
  deposit_nfts: {
    collection: string;
    nft_token_ids: string[];
    pool_id: number;
  };
} | {
  withdraw_tokens: {
    amount: Uint128;
    asset_recipient?: string | null;
    pool_id: number;
  };
} | {
  withdraw_all_tokens: {
    asset_recipient?: string | null;
    pool_id: number;
  };
} | {
  withdraw_nfts: {
    asset_recipient?: string | null;
    nft_token_ids: string[];
    pool_id: number;
  };
} | {
  withdraw_all_nfts: {
    asset_recipient?: string | null;
    pool_id: number;
  };
} | {
  update_pool_config: {
    asset_recipient?: string | null;
    delta?: Uint128 | null;
    finders_fee_bps?: number | null;
    pool_id: number;
    reinvest_nfts?: boolean | null;
    reinvest_tokens?: boolean | null;
    spot_price?: Uint128 | null;
    swap_fee_bps?: number | null;
  };
} | {
  set_active_pool: {
    is_active: boolean;
    pool_id: number;
  };
} | {
  remove_pool: {
    asset_recipient?: string | null;
    pool_id: number;
  };
} | {
  direct_swap_nfts_for_tokens: {
    finder?: string | null;
    nfts_to_swap: NftSwap[];
    pool_id: number;
    swap_params: SwapParams;
    token_recipient?: string | null;
  };
} | {
  swap_nfts_for_tokens: {
    collection: string;
    finder?: string | null;
    nfts_to_swap: NftSwap[];
    swap_params: SwapParams;
    token_recipient?: string | null;
  };
} | {
  direct_swap_tokens_for_specific_nfts: {
    finder?: string | null;
    nft_recipient?: string | null;
    nfts_to_swap_for: NftSwap[];
    pool_id: number;
    swap_params: SwapParams;
  };
} | {
  swap_tokens_for_specific_nfts: {
    collection: string;
    finder?: string | null;
    nft_recipient?: string | null;
    pool_nfts_to_swap_for: PoolNftSwap[];
    swap_params: SwapParams;
  };
} | {
  swap_tokens_for_any_nfts: {
    collection: string;
    finder?: string | null;
    max_expected_token_input: Uint128[];
    nft_recipient?: string | null;
    swap_params: SwapParams;
  };
};
export type BondingCurve = "linear" | "exponential" | "constant_product";
export type Uint128 = string;
export type PoolType = "token" | "nft" | "trade";
export type Timestamp = Uint64;
export type Uint64 = string;
export interface NftSwap {
  nft_token_id: string;
  token_amount: Uint128;
}
export interface SwapParams {
  deadline: Timestamp;
  robust: boolean;
}
export interface PoolNftSwap {
  nft_swaps: NftSwap[];
  pool_id: number;
}
export interface InstantiateMsg {
  denom: string;
  developer?: string | null;
  marketplace_addr: string;
}
export type Decimal = string;
export interface PoolInfo {
  asset_recipient?: Addr | null;
  bonding_curve: BondingCurve;
  collection: Addr;
  delta: Uint128;
  finders_fee_percent: Decimal;
  pool_type: PoolType;
  reinvest_nfts: boolean;
  reinvest_tokens: boolean;
  spot_price: Uint128;
  swap_fee_percent: Decimal;
}
export interface PoolQuoteResponse {
  pool_quotes: PoolQuote[];
}
export interface PoolQuote {
  collection: Addr;
  id: number;
  quote_price: Uint128;
}
export interface PoolsByIdResponse {
  pools: [number, Pool | null][];
}
export interface Pool {
  asset_recipient?: Addr | null;
  bonding_curve: BondingCurve;
  collection: Addr;
  delta: Uint128;
  finders_fee_percent: Decimal;
  id: number;
  is_active: boolean;
  nft_token_ids: string[];
  owner: Addr;
  pool_type: PoolType;
  reinvest_nfts: boolean;
  reinvest_tokens: boolean;
  spot_price: Uint128;
  swap_fee_percent: Decimal;
  total_tokens: Uint128;
}
export interface PoolsResponse {
  pools: Pool[];
}
export type QueryMsg = {
  config: {};
} | {
  pools: {
    query_options: QueryOptionsForUint64;
  };
} | {
  pools_by_id: {
    pool_ids: number[];
  };
} | {
  pools_by_owner: {
    owner: string;
    query_options: QueryOptionsForUint64;
  };
} | {
  pool_quotes_buy: {
    collection: string;
    query_options: QueryOptionsForTupleOfUint128AndUint64;
  };
} | {
  pool_quotes_sell: {
    collection: string;
    query_options: QueryOptionsForTupleOfUint128AndUint64;
  };
} | {
  sim_direct_swap_nfts_for_tokens: {
    finder?: string | null;
    nfts_to_swap: NftSwap[];
    pool_id: number;
    swap_params: SwapParams;
    token_recipient: string;
  };
} | {
  sim_swap_nfts_for_tokens: {
    collection: string;
    finder?: string | null;
    nfts_to_swap: NftSwap[];
    swap_params: SwapParams;
    token_recipient: string;
  };
} | {
  sim_direct_swap_tokensfor_specific_nfts: {
    finder?: string | null;
    nft_recipient: string;
    nfts_to_swap_for: NftSwap[];
    pool_id: number;
    swap_params: SwapParams;
  };
} | {
  sim_swap_tokens_for_specific_nfts: {
    collection: string;
    finder?: string | null;
    nft_recipient: string;
    pool_nfts_to_swap_for: PoolNftSwap[];
    swap_params: SwapParams;
  };
} | {
  sim_swap_tokens_for_any_nfts: {
    collection: string;
    finder?: string | null;
    max_expected_token_input: Uint128[];
    nft_recipient: string;
    swap_params: SwapParams;
  };
};
export interface QueryOptionsForUint64 {
  descending?: boolean | null;
  limit?: number | null;
  start_after?: number | null;
}
export interface QueryOptionsForTupleOfUint128AndUint64 {
  descending?: boolean | null;
  limit?: number | null;
  start_after?: [Uint128, number] | null;
}
export type TransactionType = "sell" | "buy";
export interface SwapResponse {
  swaps: Swap[];
}
export interface Swap {
  finder_payment?: TokenPayment | null;
  network_fee: Uint128;
  nft_payment?: NftPayment | null;
  pool_id: number;
  royalty_payment?: TokenPayment | null;
  seller_payment?: TokenPayment | null;
  spot_price: Uint128;
  transaction_type: TransactionType;
}
export interface TokenPayment {
  address: string;
  amount: Uint128;
}
export interface NftPayment {
  address: string;
  nft_token_id: string;
}