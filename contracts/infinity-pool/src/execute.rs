use crate::{error::ContractError};
use crate::msg::ExecuteMsg;
use crate::state::{PoolType, BondingCurve, CONFIG, Pool, POOLS, Config};
use crate::helpers::{
    save_pool, get_next_pool_counter, get_pool_attributes, transfer_nft, only_owner, transfer_token,
};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Uint128, DepsMut, Env, MessageInfo, Addr, Event, coin};
use cw_utils::{maybe_addr, must_pay, nonpayable};
use sg_std::{Response};

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
            fee_bps,
        } => execute_create_pool(
            deps,
            info,
            api.addr_validate(&collection)?,
            maybe_addr(api, asset_recipient)?,
            pool_type,
            bonding_curve,
            spot_price,
            delta,
            fee_bps,
        ),
        ExecuteMsg::DepositTokens {
            pool_id,
        } => execute_deposit_tokens(
            deps,
            info,
            pool_id,
        ),
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
        } => execute_withdraw_all_tokens(
            deps,
            info,
            pool_id,
            maybe_addr(api, asset_recipient)?,
        ),
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
        } => execute_withdraw_all_nfts(
            deps,
            info,
            pool_id,
            maybe_addr(api, asset_recipient)?,
        ),
        _ => Ok(Response::default()),
    }
}

pub fn execute_create_pool(
    deps: DepsMut,
    info: MessageInfo,
    collection: Addr,
    asset_recipient: Option<Addr>,
    pool_type: PoolType,
    bonding_curve: BondingCurve,
    spot_price: Uint128,
    delta: Uint128,
    fee_bps: u16,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let response = Response::new();

    let pool_counter = get_next_pool_counter(deps.storage)?;
    let pool = Pool::new(
        pool_counter,
        collection,
        info.sender,
        asset_recipient,
        pool_type,
        bonding_curve,
        spot_price,
        delta,
        fee_bps,
    );
    save_pool(deps.storage, &pool)?;

    let mut event = Event::new("create_token_pool");
    let pool_attributes = get_pool_attributes(&pool);
    for attribute in pool_attributes {
        event = event.add_attribute(attribute.key, attribute.value);
    }

    Ok(response.add_event(event))
}

pub fn execute_deposit_tokens(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let received_amount = must_pay(&info, &config.denom)?;
    
    let mut pool = POOLS.load(deps.storage, pool_id)?;
    only_owner(&info, &pool)?;

    let response = Response::new();
    
    pool.deposit_tokens(received_amount)?;
    save_pool(deps.storage, &pool)?;

    let event = Event::new("deposit_tokens")
        .add_attribute("pool_id", pool_id.to_string())
        .add_attribute("tokens_received", received_amount.to_string())
        .add_attribute("total_tokens", pool.total_tokens.to_string());

    Ok(response.add_event(event))
}

pub fn execute_deposit_nfts(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    pool_id: u64,
    collection: Addr,
    nft_token_ids: Vec<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = POOLS.load(deps.storage, pool_id)?;
    only_owner(&info, &pool)?;
    if pool.collection != collection {
        return Err(ContractError::InvalidPool(
            format!("invalid collection ({}) for pool ({})", collection, pool.id)
        ));
    }

    let mut response = Response::new();

    for nft_token_id in &nft_token_ids {
        transfer_nft(&nft_token_id, &env.contract.address, &collection, &mut response)?;
    }
    
    pool.deposit_nfts(&nft_token_ids)?;
    save_pool(deps.storage, &pool)?;

    let all_nft_token_ids =
        pool.nft_token_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    let event = Event::new("deposit_nfts")
        .add_attribute("nft_token_ids", pool_id.to_string())
        .add_attribute("nfts_received", nft_token_ids.join(","))
        .add_attribute("total_tokens", all_nft_token_ids);

    Ok(response.add_event(event))
}

pub fn execute_withdraw_tokens(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    amount: Uint128,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = POOLS.load(deps.storage, pool_id)?;
    only_owner(&info, &pool)?;

    let mut response = Response::new();

    let config = CONFIG.load(deps.storage)?;
    let recipient = asset_recipient.unwrap_or(info.sender);
    transfer_token(
        coin(amount.u128(), &config.denom),
        recipient.to_string(),
        "withdrawal_by_owner",
        &mut response,
    );
    
    pool.withdraw_tokens(amount)?;
    save_pool(deps.storage, &pool)?;

    let event = Event::new("withdraw_tokens")
        .add_attribute("pool_id", pool_id.to_string())
        .add_attribute("tokens_withdrawn", amount.to_string())
        .add_attribute("total_tokens", pool.total_tokens.to_string());

    Ok(response.add_event(event))
}

pub fn execute_withdraw_all_tokens(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let pool = POOLS.load(deps.storage, pool_id)?;
    execute_withdraw_tokens(deps, info, pool_id, pool.total_tokens, asset_recipient)
}

pub fn execute_withdraw_nfts(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    nft_token_ids: Vec<String>,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut pool = POOLS.load(deps.storage, pool_id)?;
    only_owner(&info, &pool)?;

    let mut response = Response::new();

    let recipient = asset_recipient.unwrap_or(info.sender);
    for nft_token_id in &nft_token_ids {
        transfer_nft(&nft_token_id, &recipient, &pool.collection, &mut response)?;
    }
    
    pool.withdraw_nfts(&nft_token_ids)?;
    save_pool(deps.storage, &pool)?;

    let all_nft_token_ids =
        pool.nft_token_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    let event = Event::new("withdraw_nfts")
        .add_attribute("nft_token_ids", pool_id.to_string())
        .add_attribute("nfts_withdrawn", nft_token_ids.join(","))
        .add_attribute("nft_token_ids", all_nft_token_ids);

    Ok(response.add_event(event))
}

pub fn execute_withdraw_all_nfts(
    deps: DepsMut,
    info: MessageInfo,
    pool_id: u64,
    asset_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let pool = POOLS.load(deps.storage, pool_id)?;

    let withdrawal_batch_size: u8 = 10;
    let nft_token_ids = 
        pool.nft_token_ids.into_iter().take(withdrawal_batch_size as usize).collect();

    execute_withdraw_nfts(deps, info, pool_id, nft_token_ids, asset_recipient)
}