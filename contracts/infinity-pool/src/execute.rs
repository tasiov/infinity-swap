use crate::error::ContractError;
use crate::helpers::{
    check_deadline, get_next_pool_counter, get_pool_attributes, load_collection_royalties,
    load_marketplace_params, only_owner, remove_pool, save_pool, save_pools, transfer_nft,
    transfer_token, validate_finder,
};
use crate::msg::{ExecuteMsg, NftSwap, PoolInfo, PoolNftSwap, SwapParams};
use crate::state::{pools, Pool, CONFIG};
use crate::swap_processor::{SwapProcessor, TransactionType};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coin, Addr, Decimal, DepsMut, Env, Event, MessageInfo, Uint128};
use cw_utils::{may_pay, maybe_addr, must_pay, nonpayable};
use sg1::fair_burn;
use sg_std::{Response, NATIVE_DENOM};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        ExecuteMsg::CreatePool {
            collection,
            asset_recipient,
            pool_type,
            bonding_curve,
            delta,
            spot_price,
            finders_fee_bps,
            swap_fee_bps,
            reinvest_tokens,
            reinvest_nfts,
        } => execute_create_pool(
            deps,
            info,
            PoolInfo {
                collection: api.addr_validate(&collection)?,
                asset_recipient: maybe_addr(api, asset_recipient)?,
                pool_type,
                bonding_curve,
                spot_price,
                delta,
                finders_fee_percent: Decimal::percent(finders_fee_bps),
                swap_fee_percent: Decimal::percent(swap_fee_bps),
                reinvest_tokens,
                reinvest_nfts,
            },
        ),
        ExecuteMsg::DepositTokens { pool_id } => execute_deposit_tokens(deps, info, pool_id),
        ExecuteMsg::DepositNfts {
            pool_id,
            collection,
            nft_token_ids,
        } => execute_deposit_nfts(
            deps,
            info,
            env,
            pool_id,
            api.addr_validate(&collection)?,
            nft_token_ids,
        ),
        ExecuteMsg::WithdrawTokens {
            pool_id,
            amount,
            asset_recipient,
        } => execute_withdraw_tokens(
            deps,
            info,
            pool_id,
            amount,
            maybe_addr(api, asset_recipient)?,
        ),
        ExecuteMsg::WithdrawAllTokens {
            pool_id,
            asset_recipient,
        } => execute_withdraw_all_tokens(deps, info, pool_id, maybe_addr(api, asset_recipient)?),
        ExecuteMsg::WithdrawNfts {
            pool_id,
            nft_token_ids,
            asset_recipient,
        } => execute_withdraw_nfts(
            deps,
            info,
            pool_id,
            nft_token_ids,
            maybe_addr(api, asset_recipient)?,
        ),
        ExecuteMsg::WithdrawAllNfts {
            pool_id,
            asset_recipient,
        } => execute_withdraw_all_nfts(deps, info, pool_id, maybe_addr(api, asset_recipient)?),
        ExecuteMsg::UpdatePoolConfig {
            pool_id,
            asset_recipient,
            delta,
            spot_price,
            finders_fee_bps,
            swap_fee_bps,
            reinvest_tokens,
            reinvest_nfts,
        } => execute_update_pool_config(
            deps,
            info,
            pool_id,
            maybe_addr(api, asset_recipient)?,
            delta,
            spot_price,
            finders_fee_bps,
            swap_fee_bps,
            reinvest_tokens,
            reinvest_nfts,
        ),
        ExecuteMsg::SetActivePool { pool_id, is_active } => {
            execute_set_active_pool(deps, info, pool_id, is_active)
        }
        ExecuteMsg::RemovePool {
            pool_id,
            asset_recipient,
        } => execute_remove_pool(deps, info, pool_id, maybe_addr(api, asset_recipient)?),
        ExecuteMsg::DirectSwapNftsForTokens {
            pool_id,
            nfts_to_swap,
            swap_params,
            token_recipient,
            finder,
        } => execute_direct_swap_nfts_for_tokens(
            deps,
            info,
            env,
            pool_id,
            nfts_to_swap,
            swap_params,
            maybe_addr(api, token_recipient)?,
            maybe_addr(api, finder)?,
        ),
        ExecuteMsg::SwapNftsForTokens {
            collection,
            nfts_to_swap,
            swap_params,
            token_recipient,
            finder,
        } => execute_swap_nfts_for_tokens(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            nfts_to_swap,
            swap_params,
            maybe_addr(api, token_recipient)?,
            maybe_addr(api, finder)?,
        ),
        ExecuteMsg::DirectSwapTokensForSpecificNfts {
            pool_id,
            nfts_to_swap_for,
            swap_params,
            nft_recipient,
            finder,
        } => execute_direct_swap_tokens_for_specific_nfts(
            deps,
            info,
            env,
            pool_id,
            nfts_to_swap_for,
            swap_params,
            maybe_addr(api, nft_recipient)?,
            maybe_addr(api, finder)?,
        ),
        ExecuteMsg::SwapTokensForSpecificNfts {
            collection,
            pool_nfts_to_swap_for,
            swap_params,
            nft_recipient,
            finder,
        } => execute_swap_tokens_for_specific_nfts(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            pool_nfts_to_swap_for,
            swap_params,
            maybe_addr(api, nft_recipient)?,
            maybe_addr(api, finder)?,
        ),
        ExecuteMsg::SwapTokensForAnyNfts {
            collection,
            max_expected_token_input,
            swap_params,
            nft_recipient,
            finder,
        } => execute_swap_tokens_for_any_nfts(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            max_expected_token_input,
            swap_params,
            maybe_addr(api, nft_recipient)?,
            maybe_addr(api, finder)?,
        ),
    }
}

/// Execute a CreatePool message
pub fn execute_create_pool(
    deps: DepsMut,
    info: MessageInfo,
    pool_info: PoolInfo,
) -> Result<Response, ContractError> {
    let pool_counter = get_next_pool_counter(deps.storage)?;
    let pool = Pool::new(
        pool_counter,
        pool_info.collection,
        info.sender.clone(),
        pool_info.asset_recipient,
        pool_info.pool_type,
        pool_info.bonding_curve,
        pool_info.spot_price,
        pool_info.delta,
        pool_info.finders_fee_percent,
        pool_info.swap_fee_percent,
        pool_info.reinvest_tokens,
        pool_info.reinvest_nfts,
    );

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    // Burn the listing fee set on the marketplace contract
    let listing_fee = may_pay(&info, NATIVE_DENOM)?;
    if listing_fee != marketplace_params.params.listing_fee {
        return Err(ContractError::InvalidListingFee(listing_fee));
    }
    let mut response = Response::new();
    if listing_fee > Uint128::zero() {
        fair_burn(listing_fee.u128(), None, &mut response);
    }

    let mut event = Event::new("create_token_pool");
    let pool_attributes = get_pool_attributes(&pool);
    for attribute in pool_attributes {
        event = event.add_attribute(attribute.key, attribute.value);
    }

    Ok(response.add_event(event))
}

/// Execute a DepositTokens message
pub fn execute_deposit_tokens(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let received_amount = must_pay(&info, &config.denom)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can deposit and withdraw assets
    only_owner(&info, &pool)?;

    // Track the total amount of tokens that have been deposited into the pool
    pool.deposit_tokens(received_amount)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    let response = Response::new();
    let event = Event::new("deposit_tokens")
        .add_attribute("pool_id", pool_id.to_string())
        .add_attribute("tokens_received", received_amount.to_string())
        .add_attribute("total_tokens", pool.total_tokens.to_string());

    Ok(response.add_event(event))
}

/// Execute a DepositNfts message
pub fn execute_deposit_nfts(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    pool_id: u64,
    collection: Addr,
    nft_token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can deposit and withdraw assets
    only_owner(&info, &pool)?;
    if pool.collection != collection {
        return Err(ContractError::InvalidInput(format!(
            "invalid collection ({}) for pool ({})",
            collection, pool.id
        )));
    }

    // Push the NFT transfer messages
    let mut response = Response::new();
    for nft_token_id in &nft_token_ids {
        transfer_nft(
            nft_token_id,
            env.contract.address.as_ref(),
            collection.as_ref(),
            &mut response,
        )?;
    }
    // Track the NFTs that have been deposited into the pool
    pool.deposit_nfts(&nft_token_ids)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    let all_nft_token_ids = pool
        .nft_token_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let event = Event::new("deposit_nfts")
        .add_attribute("nfts_received", nft_token_ids.join(","))
        .add_attribute("nft_token_ids", all_nft_token_ids);

    Ok(response.add_event(event))
}

/// Execute a WithdrawNfts message
pub fn execute_withdraw_tokens(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    amount: Uint128,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can deposit and withdraw assets
    only_owner(&info, &pool)?;

    let mut response = Response::new();

    let config = CONFIG.load(deps.storage)?;
    // Withdraw tokens to the asset recipient if specified, otherwise to the sender
    let recipient = asset_recipient.unwrap_or(info.sender);
    transfer_token(
        coin(amount.u128(), config.denom),
        recipient.as_ref(),
        &mut response,
    )?;
    // Track total amount owned by the pool
    pool.withdraw_tokens(amount)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    let event = Event::new("withdraw_tokens")
        .add_attribute("pool_id", pool_id.to_string())
        .add_attribute("tokens_withdrawn", amount.to_string())
        .add_attribute("total_tokens", pool.total_tokens.to_string());

    Ok(response.add_event(event))
}

/// Execute a WithdrawAllNfts message, a convenvience method for withdrawing all tokens
pub fn execute_withdraw_all_tokens(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let pool = pools().load(deps.storage, pool_id)?;
    execute_withdraw_tokens(deps, info, pool_id, pool.total_tokens, asset_recipient)
}

/// Execute a WithdrawNfts message
pub fn execute_withdraw_nfts(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    nft_token_ids: Vec<String>,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can deposit and withdraw assets
    only_owner(&info, &pool)?;

    let mut response = Response::new();

    // Withdraw NFTs to the asset recipient if specified, otherwise to the sender
    let recipient = asset_recipient.unwrap_or(info.sender);
    for nft_token_id in &nft_token_ids {
        transfer_nft(
            nft_token_id,
            recipient.as_ref(),
            pool.collection.as_ref(),
            &mut response,
        )?;
    }
    // Track the NFTs that have been withdrawn from the pool
    pool.withdraw_nfts(&nft_token_ids)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    let all_nft_token_ids = pool
        .nft_token_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let event = Event::new("withdraw_nfts")
        .add_attribute("nft_token_ids", pool_id.to_string())
        .add_attribute("nfts_withdrawn", nft_token_ids.join(","))
        .add_attribute("nft_token_ids", all_nft_token_ids);

    Ok(response.add_event(event))
}

/// Execute a WithdrawAllNfts message, a convenvience method for withdrawing all NFTs
pub fn execute_withdraw_all_nfts(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let pool = pools().load(deps.storage, pool_id)?;

    let withdrawal_batch_size: u8 = 10;
    let nft_token_ids = pool
        .nft_token_ids
        .into_iter()
        .take(withdrawal_batch_size as usize)
        .collect();

    execute_withdraw_nfts(deps, info, pool_id, nft_token_ids, asset_recipient)
}

/// Execute an UpdatePoolConfig message
/// Option paramaters that are not specified will not be updated
pub fn execute_update_pool_config(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    asset_recipient: Option<Addr>,
    delta: Option<Uint128>,
    spot_price: Option<Uint128>,
    finders_fee_bps: Option<u64>,
    swap_fee_bps: Option<u64>,
    reinvest_tokens: Option<bool>,
    reinvest_nfts: Option<bool>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can update the pool config
    only_owner(&info, &pool)?;

    if let Some(_asset_recipient) = asset_recipient {
        pool.asset_recipient = Some(_asset_recipient);
    }
    if let Some(_spot_price) = spot_price {
        pool.spot_price = _spot_price;
    }
    if let Some(_delta) = delta {
        pool.delta = _delta;
    }
    if let Some(_swap_fee_bps) = swap_fee_bps {
        pool.swap_fee_percent = Decimal::percent(_swap_fee_bps);
    }
    if let Some(_finders_fee_bps) = finders_fee_bps {
        pool.finders_fee_percent = Decimal::percent(_finders_fee_bps);
    }
    if let Some(_reinvest_tokens) = reinvest_tokens {
        pool.reinvest_tokens = _reinvest_tokens;
    }
    if let Some(_reinvest_nfts) = reinvest_nfts {
        pool.reinvest_nfts = _reinvest_nfts;
    }

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    let response = Response::new();
    let mut event = Event::new("update_pool_config");
    let pool_attributes = get_pool_attributes(&pool);
    for attribute in pool_attributes {
        event = event.add_attribute(attribute.key, attribute.value);
    }

    Ok(response.add_event(event))
}

/// Execute a SetActivePool message
pub fn execute_set_active_pool(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    is_active: bool,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can update the pool config
    only_owner(&info, &pool)?;

    pool.set_active(is_active)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    save_pool(deps.storage, &pool, &marketplace_params)?;

    let response = Response::new();
    let event = Event::new("toggle_pool")
        .add_attribute("pool_id", pool_id.to_string())
        .add_attribute("is_active", pool.is_active.to_string());

    Ok(response.add_event(event))
}

/// Execute a RemovePool message
pub fn execute_remove_pool(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    // Only the owner of the pool can remove the pool
    only_owner(&info, &pool)?;

    // Pools that hold NFTs cannot be removed
    if !pool.nft_token_ids.is_empty() {
        let all_nft_token_ids = pool
            .nft_token_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");
        return Err(ContractError::UnableToRemovePool(format!(
            "pool {} still has NFTs: {}",
            pool_id, all_nft_token_ids
        )));
    }

    let config = CONFIG.load(deps.storage)?;
    let mut response = Response::new();

    // If the pool has tokens, transfer them to the asset recipient
    if pool.total_tokens > Uint128::zero() {
        let recipient = asset_recipient.unwrap_or(info.sender);
        transfer_token(
            coin(pool.total_tokens.u128(), config.denom),
            recipient.as_ref(),
            &mut response,
        )?;
    }

    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;
    remove_pool(deps.storage, &mut pool, &marketplace_params)?;

    let event = Event::new("remove_pool").add_attribute("pool_id", pool_id.to_string());
    Ok(response.add_event(event))
}

/// Execute a DirectSwapNftsForTokens message
pub fn execute_direct_swap_nfts_for_tokens(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    pool_id: u64,
    nfts_to_swap: Vec<NftSwap>,
    swap_params: SwapParams,
    asset_recipient: Option<Addr>,
    finder: Option<Addr>,
) -> Result<Response, ContractError> {
    check_deadline(&env.block, swap_params.deadline)?;
    validate_finder(&finder, &info.sender, &asset_recipient)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;

    let mut pool = pools().load(deps.storage, pool_id)?;
    let seller_recipient = asset_recipient.unwrap_or(info.sender);
    let collection_royalties = load_collection_royalties(deps.as_ref(), &pool.collection)?;

    let mut response = Response::new();
    {
        let mut processor = SwapProcessor::new(
            TransactionType::Sell,
            pool.collection.clone(),
            seller_recipient,
            marketplace_params.params.trading_fee_percent,
            collection_royalties,
            finder,
            config.developer,
        );
        processor.direct_swap_nfts_for_tokens(pool, nfts_to_swap, swap_params)?;
        processor.commit_messages(&mut response)?;
        pool = processor
            .pool_set
            .into_iter()
            .filter(|p| p.needs_saving)
            .map(|p| p.pool)
            .next()
            .unwrap();
    }
    save_pool(deps.storage, &pool, &marketplace_params)?;

    Ok(response)
}

/// Execute a SwapNftsForTokens message
pub fn execute_swap_nfts_for_tokens(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    nfts_to_swap: Vec<NftSwap>,
    swap_params: SwapParams,
    asset_recipient: Option<Addr>,
    finder: Option<Addr>,
) -> Result<Response, ContractError> {
    check_deadline(&env.block, swap_params.deadline)?;
    validate_finder(&finder, &info.sender, &asset_recipient)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;

    let seller_recipient = asset_recipient.unwrap_or(info.sender);
    let collection_royalties = load_collection_royalties(deps.as_ref(), &collection)?;

    let pools_to_save: Vec<Pool>;
    let mut response = Response::new();
    {
        let mut processor = SwapProcessor::new(
            TransactionType::Sell,
            collection,
            seller_recipient,
            marketplace_params.params.trading_fee_percent,
            collection_royalties,
            finder,
            config.developer,
        );
        processor.swap_nfts_for_tokens(deps.storage, nfts_to_swap, swap_params)?;
        processor.commit_messages(&mut response)?;
        pools_to_save = processor
            .pool_set
            .into_iter()
            .filter(|p| p.needs_saving)
            .map(|p| p.pool)
            .collect();
    }
    save_pools(deps.storage, pools_to_save, &marketplace_params)?;

    Ok(response)
}

/// Execute a DirectSwapTokensForSpecificNfts message
pub fn execute_direct_swap_tokens_for_specific_nfts(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    pool_id: u64,
    nfts_to_swap_for: Vec<NftSwap>,
    swap_params: SwapParams,
    asset_recipient: Option<Addr>,
    finder: Option<Addr>,
) -> Result<Response, ContractError> {
    check_deadline(&env.block, swap_params.deadline)?;
    validate_finder(&finder, &info.sender, &asset_recipient)?;

    let pool = pools().load(deps.storage, pool_id)?;
    execute_swap_tokens_for_specific_nfts(
        deps,
        info,
        env,
        pool.collection,
        vec![PoolNftSwap {
            pool_id,
            nft_swaps: nfts_to_swap_for,
        }],
        swap_params,
        asset_recipient,
        finder,
    )
}

/// Execute a SwapTokensForSpecificNfts message
pub fn execute_swap_tokens_for_specific_nfts(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    nfts_to_swap_for: Vec<PoolNftSwap>,
    swap_params: SwapParams,
    asset_recipient: Option<Addr>,
    finder: Option<Addr>,
) -> Result<Response, ContractError> {
    check_deadline(&env.block, swap_params.deadline)?;
    validate_finder(&finder, &info.sender, &asset_recipient)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;

    let seller_recipient = asset_recipient.unwrap_or(info.sender);
    let collection_royalties = load_collection_royalties(deps.as_ref(), &collection)?;

    let pools_to_save: Vec<Pool>;
    let mut response = Response::new();
    {
        let mut processor = SwapProcessor::new(
            TransactionType::Buy,
            collection,
            seller_recipient,
            marketplace_params.params.trading_fee_percent,
            collection_royalties,
            finder,
            config.developer,
        );
        processor.swap_tokens_for_specific_nfts(deps.storage, nfts_to_swap_for, swap_params)?;
        processor.commit_messages(&mut response)?;
        pools_to_save = processor
            .pool_set
            .into_iter()
            .filter(|p| p.needs_saving)
            .map(|p| p.pool)
            .collect();
    }
    save_pools(deps.storage, pools_to_save, &marketplace_params)?;

    Ok(response)
}

/// Execute a SwapTokensForAnyNfts message
pub fn execute_swap_tokens_for_any_nfts(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    max_expected_token_input: Vec<Uint128>,
    swap_params: SwapParams,
    asset_recipient: Option<Addr>,
    finder: Option<Addr>,
) -> Result<Response, ContractError> {
    check_deadline(&env.block, swap_params.deadline)?;
    validate_finder(&finder, &info.sender, &asset_recipient)?;

    let config = CONFIG.load(deps.storage)?;
    let marketplace_params = load_marketplace_params(deps.as_ref(), &config.marketplace_addr)?;

    let seller_recipient = asset_recipient.unwrap_or(info.sender);
    let collection_royalties = load_collection_royalties(deps.as_ref(), &collection)?;

    let pools_to_save: Vec<Pool>;
    let mut response = Response::new();
    {
        let mut processor = SwapProcessor::new(
            TransactionType::Buy,
            collection,
            seller_recipient,
            marketplace_params.params.trading_fee_percent,
            collection_royalties,
            finder,
            config.developer,
        );
        processor.swap_tokens_for_any_nfts(deps.storage, max_expected_token_input, swap_params)?;
        processor.commit_messages(&mut response)?;
        pools_to_save = processor
            .pool_set
            .into_iter()
            .filter(|p| p.needs_saving)
            .map(|p| p.pool)
            .collect();
    }
    save_pools(deps.storage, pools_to_save, &marketplace_params)?;

    Ok(response)
}
