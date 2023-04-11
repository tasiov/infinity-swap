/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Addr, ConfigResponse, Config, ExecuteMsg, BondingCurve, Uint128, Timestamp, Uint64, NftSwap, SwapParams, PoolNftSwap, InstantiateMsg, NftTokenIdsResponse, PoolQuoteResponse, PoolQuote, Decimal, PoolType, PoolsByIdResponse, Pool, PoolsResponse, QueryMsg, QueryOptionsForUint64, QueryOptionsForString, QueryOptionsForTupleOfUint128AndUint64, TransactionType, SwapResponse, Swap, TokenPayment, NftPayment } from "./InfinitySwap.types";
export interface InfinitySwapReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<ConfigResponse>;
  pools: ({
    queryOptions
  }: {
    queryOptions: QueryOptions_for_uint64;
  }) => Promise<PoolsResponse>;
  poolsById: ({
    poolIds
  }: {
    poolIds: number[];
  }) => Promise<PoolsByIdResponse>;
  poolsByOwner: ({
    owner,
    queryOptions
  }: {
    owner: string;
    queryOptions: QueryOptions_for_uint64;
  }) => Promise<PoolsByOwnerResponse>;
  poolNftTokenIds: ({
    poolId,
    queryOptions
  }: {
    poolId: number;
    queryOptions: QueryOptionsForString;
  }) => Promise<PoolNftTokenIdsResponse>;
  quotesBuyFromPool: ({
    collection,
    queryOptions
  }: {
    collection: string;
    queryOptions: QueryOptionsForTupleOfUint128_and_uint64;
  }) => Promise<QuotesBuyFromPoolResponse>;
  quotesSellToPool: ({
    collection,
    queryOptions
  }: {
    collection: string;
    queryOptions: QueryOptionsForTupleOfUint128_and_uint64;
  }) => Promise<QuotesSellToPoolResponse>;
  simDirectSwapNftsForTokens: ({
    nftsToSwap,
    poolId,
    sender,
    swapParams
  }: {
    nftsToSwap: NftSwap[];
    poolId: number;
    sender: string;
    swapParams: SwapParams;
  }) => Promise<SimDirectSwapNftsForTokensResponse>;
  simSwapNftsForTokens: ({
    collection,
    nftsToSwap,
    sender,
    swapParams
  }: {
    collection: string;
    nftsToSwap: NftSwap[];
    sender: string;
    swapParams: SwapParams;
  }) => Promise<SimSwapNftsForTokensResponse>;
  simDirectSwapTokensForSpecificNfts: ({
    nftsToSwapFor,
    poolId,
    sender,
    swapParams
  }: {
    nftsToSwapFor: NftSwap[];
    poolId: number;
    sender: string;
    swapParams: SwapParams;
  }) => Promise<SimDirectSwapTokensForSpecificNftsResponse>;
  simSwapTokensForSpecificNfts: ({
    collection,
    poolNftsToSwapFor,
    sender,
    swapParams
  }: {
    collection: string;
    poolNftsToSwapFor: PoolNftSwap[];
    sender: string;
    swapParams: SwapParams;
  }) => Promise<SimSwapTokensForSpecificNftsResponse>;
  simSwapTokensForAnyNfts: ({
    collection,
    maxExpectedTokenInput,
    sender,
    swapParams
  }: {
    collection: string;
    maxExpectedTokenInput: Uint128[];
    sender: string;
    swapParams: SwapParams;
  }) => Promise<SimSwapTokensForAnyNftsResponse>;
}
export class InfinitySwapQueryClient implements InfinitySwapReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.pools = this.pools.bind(this);
    this.poolsById = this.poolsById.bind(this);
    this.poolsByOwner = this.poolsByOwner.bind(this);
    this.poolNftTokenIds = this.poolNftTokenIds.bind(this);
    this.quotesBuyFromPool = this.quotesBuyFromPool.bind(this);
    this.quotesSellToPool = this.quotesSellToPool.bind(this);
    this.simDirectSwapNftsForTokens = this.simDirectSwapNftsForTokens.bind(this);
    this.simSwapNftsForTokens = this.simSwapNftsForTokens.bind(this);
    this.simDirectSwapTokensForSpecificNfts = this.simDirectSwapTokensForSpecificNfts.bind(this);
    this.simSwapTokensForSpecificNfts = this.simSwapTokensForSpecificNfts.bind(this);
    this.simSwapTokensForAnyNfts = this.simSwapTokensForAnyNfts.bind(this);
  }

  config = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  pools = async ({
    queryOptions
  }: {
    queryOptions: QueryOptions_for_uint64;
  }): Promise<PoolsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      pools: {
        query_options: queryOptions
      }
    });
  };
  poolsById = async ({
    poolIds
  }: {
    poolIds: number[];
  }): Promise<PoolsByIdResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      pools_by_id: {
        pool_ids: poolIds
      }
    });
  };
  poolsByOwner = async ({
    owner,
    queryOptions
  }: {
    owner: string;
    queryOptions: QueryOptions_for_uint64;
  }): Promise<PoolsByOwnerResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      pools_by_owner: {
        owner,
        query_options: queryOptions
      }
    });
  };
  poolNftTokenIds = async ({
    poolId,
    queryOptions
  }: {
    poolId: number;
    queryOptions: QueryOptionsForString;
  }): Promise<PoolNftTokenIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      pool_nft_token_ids: {
        pool_id: poolId,
        query_options: queryOptions
      }
    });
  };
  quotesBuyFromPool = async ({
    collection,
    queryOptions
  }: {
    collection: string;
    queryOptions: QueryOptionsForTupleOfUint128_and_uint64;
  }): Promise<QuotesBuyFromPoolResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      quotes_buy_from_pool: {
        collection,
        query_options: queryOptions
      }
    });
  };
  quotesSellToPool = async ({
    collection,
    queryOptions
  }: {
    collection: string;
    queryOptions: QueryOptionsForTupleOfUint128_and_uint64;
  }): Promise<QuotesSellToPoolResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      quotes_sell_to_pool: {
        collection,
        query_options: queryOptions
      }
    });
  };
  simDirectSwapNftsForTokens = async ({
    nftsToSwap,
    poolId,
    sender,
    swapParams
  }: {
    nftsToSwap: NftSwap[];
    poolId: number;
    sender: string;
    swapParams: SwapParams;
  }): Promise<SimDirectSwapNftsForTokensResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      sim_direct_swap_nfts_for_tokens: {
        nfts_to_swap: nftsToSwap,
        pool_id: poolId,
        sender,
        swap_params: swapParams
      }
    });
  };
  simSwapNftsForTokens = async ({
    collection,
    nftsToSwap,
    sender,
    swapParams
  }: {
    collection: string;
    nftsToSwap: NftSwap[];
    sender: string;
    swapParams: SwapParams;
  }): Promise<SimSwapNftsForTokensResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      sim_swap_nfts_for_tokens: {
        collection,
        nfts_to_swap: nftsToSwap,
        sender,
        swap_params: swapParams
      }
    });
  };
  simDirectSwapTokensForSpecificNfts = async ({
    nftsToSwapFor,
    poolId,
    sender,
    swapParams
  }: {
    nftsToSwapFor: NftSwap[];
    poolId: number;
    sender: string;
    swapParams: SwapParams;
  }): Promise<SimDirectSwapTokensForSpecificNftsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      sim_direct_swap_tokens_for_specific_nfts: {
        nfts_to_swap_for: nftsToSwapFor,
        pool_id: poolId,
        sender,
        swap_params: swapParams
      }
    });
  };
  simSwapTokensForSpecificNfts = async ({
    collection,
    poolNftsToSwapFor,
    sender,
    swapParams
  }: {
    collection: string;
    poolNftsToSwapFor: PoolNftSwap[];
    sender: string;
    swapParams: SwapParams;
  }): Promise<SimSwapTokensForSpecificNftsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      sim_swap_tokens_for_specific_nfts: {
        collection,
        pool_nfts_to_swap_for: poolNftsToSwapFor,
        sender,
        swap_params: swapParams
      }
    });
  };
  simSwapTokensForAnyNfts = async ({
    collection,
    maxExpectedTokenInput,
    sender,
    swapParams
  }: {
    collection: string;
    maxExpectedTokenInput: Uint128[];
    sender: string;
    swapParams: SwapParams;
  }): Promise<SimSwapTokensForAnyNftsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      sim_swap_tokens_for_any_nfts: {
        collection,
        max_expected_token_input: maxExpectedTokenInput,
        sender,
        swap_params: swapParams
      }
    });
  };
}
export interface InfinitySwapInterface extends InfinitySwapReadOnlyInterface {
  contractAddress: string;
  sender: string;
  createTokenPool: ({
    assetRecipient,
    bondingCurve,
    collection,
    delta,
    findersFeeBps,
    spotPrice
  }: {
    assetRecipient?: string;
    bondingCurve: BondingCurve;
    collection: string;
    delta: Uint128;
    findersFeeBps: number;
    spotPrice: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  createNftPool: ({
    assetRecipient,
    bondingCurve,
    collection,
    delta,
    findersFeeBps,
    spotPrice
  }: {
    assetRecipient?: string;
    bondingCurve: BondingCurve;
    collection: string;
    delta: Uint128;
    findersFeeBps: number;
    spotPrice: Uint128;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  createTradePool: ({
    assetRecipient,
    bondingCurve,
    collection,
    delta,
    findersFeeBps,
    reinvestNfts,
    reinvestTokens,
    spotPrice,
    swapFeeBps
  }: {
    assetRecipient?: string;
    bondingCurve: BondingCurve;
    collection: string;
    delta: Uint128;
    findersFeeBps: number;
    reinvestNfts: boolean;
    reinvestTokens: boolean;
    spotPrice: Uint128;
    swapFeeBps: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  depositTokens: ({
    poolId
  }: {
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  depositNfts: ({
    collection,
    nftTokenIds,
    poolId
  }: {
    collection: string;
    nftTokenIds: string[];
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawTokens: ({
    amount,
    assetRecipient,
    poolId
  }: {
    amount: Uint128;
    assetRecipient?: string;
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawAllTokens: ({
    assetRecipient,
    poolId
  }: {
    assetRecipient?: string;
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawNfts: ({
    assetRecipient,
    nftTokenIds,
    poolId
  }: {
    assetRecipient?: string;
    nftTokenIds: string[];
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  withdrawAllNfts: ({
    assetRecipient,
    poolId
  }: {
    assetRecipient?: string;
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  updatePoolConfig: ({
    assetRecipient,
    delta,
    findersFeeBps,
    poolId,
    reinvestNfts,
    reinvestTokens,
    spotPrice,
    swapFeeBps
  }: {
    assetRecipient?: string;
    delta?: Uint128;
    findersFeeBps?: number;
    poolId: number;
    reinvestNfts?: boolean;
    reinvestTokens?: boolean;
    spotPrice?: Uint128;
    swapFeeBps?: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  setActivePool: ({
    isActive,
    poolId
  }: {
    isActive: boolean;
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removePool: ({
    assetRecipient,
    poolId
  }: {
    assetRecipient?: string;
    poolId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  directSwapNftsForTokens: ({
    nftsToSwap,
    poolId,
    swapParams
  }: {
    nftsToSwap: NftSwap[];
    poolId: number;
    swapParams: SwapParams;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  swapNftsForTokens: ({
    collection,
    nftsToSwap,
    swapParams
  }: {
    collection: string;
    nftsToSwap: NftSwap[];
    swapParams: SwapParams;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  directSwapTokensForSpecificNfts: ({
    nftsToSwapFor,
    poolId,
    swapParams
  }: {
    nftsToSwapFor: NftSwap[];
    poolId: number;
    swapParams: SwapParams;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  swapTokensForSpecificNfts: ({
    collection,
    poolNftsToSwapFor,
    swapParams
  }: {
    collection: string;
    poolNftsToSwapFor: PoolNftSwap[];
    swapParams: SwapParams;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  swapTokensForAnyNfts: ({
    collection,
    maxExpectedTokenInput,
    swapParams
  }: {
    collection: string;
    maxExpectedTokenInput: Uint128[];
    swapParams: SwapParams;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class InfinitySwapClient extends InfinitySwapQueryClient implements InfinitySwapInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.createTokenPool = this.createTokenPool.bind(this);
    this.createNftPool = this.createNftPool.bind(this);
    this.createTradePool = this.createTradePool.bind(this);
    this.depositTokens = this.depositTokens.bind(this);
    this.depositNfts = this.depositNfts.bind(this);
    this.withdrawTokens = this.withdrawTokens.bind(this);
    this.withdrawAllTokens = this.withdrawAllTokens.bind(this);
    this.withdrawNfts = this.withdrawNfts.bind(this);
    this.withdrawAllNfts = this.withdrawAllNfts.bind(this);
    this.updatePoolConfig = this.updatePoolConfig.bind(this);
    this.setActivePool = this.setActivePool.bind(this);
    this.removePool = this.removePool.bind(this);
    this.directSwapNftsForTokens = this.directSwapNftsForTokens.bind(this);
    this.swapNftsForTokens = this.swapNftsForTokens.bind(this);
    this.directSwapTokensForSpecificNfts = this.directSwapTokensForSpecificNfts.bind(this);
    this.swapTokensForSpecificNfts = this.swapTokensForSpecificNfts.bind(this);
    this.swapTokensForAnyNfts = this.swapTokensForAnyNfts.bind(this);
  }

  createTokenPool = async ({
    assetRecipient,
    bondingCurve,
    collection,
    delta,
    findersFeeBps,
    spotPrice
  }: {
    assetRecipient?: string;
    bondingCurve: BondingCurve;
    collection: string;
    delta: Uint128;
    findersFeeBps: number;
    spotPrice: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_token_pool: {
        asset_recipient: assetRecipient,
        bonding_curve: bondingCurve,
        collection,
        delta,
        finders_fee_bps: findersFeeBps,
        spot_price: spotPrice
      }
    }, fee, memo, funds);
  };
  createNftPool = async ({
    assetRecipient,
    bondingCurve,
    collection,
    delta,
    findersFeeBps,
    spotPrice
  }: {
    assetRecipient?: string;
    bondingCurve: BondingCurve;
    collection: string;
    delta: Uint128;
    findersFeeBps: number;
    spotPrice: Uint128;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_nft_pool: {
        asset_recipient: assetRecipient,
        bonding_curve: bondingCurve,
        collection,
        delta,
        finders_fee_bps: findersFeeBps,
        spot_price: spotPrice
      }
    }, fee, memo, funds);
  };
  createTradePool = async ({
    assetRecipient,
    bondingCurve,
    collection,
    delta,
    findersFeeBps,
    reinvestNfts,
    reinvestTokens,
    spotPrice,
    swapFeeBps
  }: {
    assetRecipient?: string;
    bondingCurve: BondingCurve;
    collection: string;
    delta: Uint128;
    findersFeeBps: number;
    reinvestNfts: boolean;
    reinvestTokens: boolean;
    spotPrice: Uint128;
    swapFeeBps: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_trade_pool: {
        asset_recipient: assetRecipient,
        bonding_curve: bondingCurve,
        collection,
        delta,
        finders_fee_bps: findersFeeBps,
        reinvest_nfts: reinvestNfts,
        reinvest_tokens: reinvestTokens,
        spot_price: spotPrice,
        swap_fee_bps: swapFeeBps
      }
    }, fee, memo, funds);
  };
  depositTokens = async ({
    poolId
  }: {
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      deposit_tokens: {
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  depositNfts = async ({
    collection,
    nftTokenIds,
    poolId
  }: {
    collection: string;
    nftTokenIds: string[];
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      deposit_nfts: {
        collection,
        nft_token_ids: nftTokenIds,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  withdrawTokens = async ({
    amount,
    assetRecipient,
    poolId
  }: {
    amount: Uint128;
    assetRecipient?: string;
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_tokens: {
        amount,
        asset_recipient: assetRecipient,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  withdrawAllTokens = async ({
    assetRecipient,
    poolId
  }: {
    assetRecipient?: string;
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_all_tokens: {
        asset_recipient: assetRecipient,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  withdrawNfts = async ({
    assetRecipient,
    nftTokenIds,
    poolId
  }: {
    assetRecipient?: string;
    nftTokenIds: string[];
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_nfts: {
        asset_recipient: assetRecipient,
        nft_token_ids: nftTokenIds,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  withdrawAllNfts = async ({
    assetRecipient,
    poolId
  }: {
    assetRecipient?: string;
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      withdraw_all_nfts: {
        asset_recipient: assetRecipient,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  updatePoolConfig = async ({
    assetRecipient,
    delta,
    findersFeeBps,
    poolId,
    reinvestNfts,
    reinvestTokens,
    spotPrice,
    swapFeeBps
  }: {
    assetRecipient?: string;
    delta?: Uint128;
    findersFeeBps?: number;
    poolId: number;
    reinvestNfts?: boolean;
    reinvestTokens?: boolean;
    spotPrice?: Uint128;
    swapFeeBps?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_pool_config: {
        asset_recipient: assetRecipient,
        delta,
        finders_fee_bps: findersFeeBps,
        pool_id: poolId,
        reinvest_nfts: reinvestNfts,
        reinvest_tokens: reinvestTokens,
        spot_price: spotPrice,
        swap_fee_bps: swapFeeBps
      }
    }, fee, memo, funds);
  };
  setActivePool = async ({
    isActive,
    poolId
  }: {
    isActive: boolean;
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      set_active_pool: {
        is_active: isActive,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  removePool = async ({
    assetRecipient,
    poolId
  }: {
    assetRecipient?: string;
    poolId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_pool: {
        asset_recipient: assetRecipient,
        pool_id: poolId
      }
    }, fee, memo, funds);
  };
  directSwapNftsForTokens = async ({
    nftsToSwap,
    poolId,
    swapParams
  }: {
    nftsToSwap: NftSwap[];
    poolId: number;
    swapParams: SwapParams;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      direct_swap_nfts_for_tokens: {
        nfts_to_swap: nftsToSwap,
        pool_id: poolId,
        swap_params: swapParams
      }
    }, fee, memo, funds);
  };
  swapNftsForTokens = async ({
    collection,
    nftsToSwap,
    swapParams
  }: {
    collection: string;
    nftsToSwap: NftSwap[];
    swapParams: SwapParams;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      swap_nfts_for_tokens: {
        collection,
        nfts_to_swap: nftsToSwap,
        swap_params: swapParams
      }
    }, fee, memo, funds);
  };
  directSwapTokensForSpecificNfts = async ({
    nftsToSwapFor,
    poolId,
    swapParams
  }: {
    nftsToSwapFor: NftSwap[];
    poolId: number;
    swapParams: SwapParams;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      direct_swap_tokens_for_specific_nfts: {
        nfts_to_swap_for: nftsToSwapFor,
        pool_id: poolId,
        swap_params: swapParams
      }
    }, fee, memo, funds);
  };
  swapTokensForSpecificNfts = async ({
    collection,
    poolNftsToSwapFor,
    swapParams
  }: {
    collection: string;
    poolNftsToSwapFor: PoolNftSwap[];
    swapParams: SwapParams;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      swap_tokens_for_specific_nfts: {
        collection,
        pool_nfts_to_swap_for: poolNftsToSwapFor,
        swap_params: swapParams
      }
    }, fee, memo, funds);
  };
  swapTokensForAnyNfts = async ({
    collection,
    maxExpectedTokenInput,
    swapParams
  }: {
    collection: string;
    maxExpectedTokenInput: Uint128[];
    swapParams: SwapParams;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      swap_tokens_for_any_nfts: {
        collection,
        max_expected_token_input: maxExpectedTokenInput,
        swap_params: swapParams
      }
    }, fee, memo, funds);
  };
}