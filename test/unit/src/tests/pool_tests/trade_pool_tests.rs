use std::vec;

use crate::helpers::nft_functions::{approve, mint};
use crate::helpers::pool_functions::create_pool;
use crate::helpers::utils::assert_error;
use crate::setup::setup_infinity_swap::setup_infinity_swap;
use crate::setup::setup_marketplace::{setup_marketplace, LISTING_FEE};
use crate::setup::templates::standard_minter_template;
use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::Executor;
use infinity_swap::msg::ExecuteMsg;
use infinity_swap::state::BondingCurve;
use infinity_swap::ContractError;
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};
use test_suite::common_setup::setup_accounts_and_block::setup_block_time;

const ASSET_ACCOUNT: &str = "asset";

#[test]
fn create_trade_pool() {
    let vt = standard_minter_template(5000);
    let (mut router, creator, _bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let asset_account = Addr::unchecked(ASSET_ACCOUNT);

    let marketplace = setup_marketplace(&mut router, creator.clone()).unwrap();
    let infinity_swap = setup_infinity_swap(&mut router, creator.clone(), marketplace).unwrap();

    // Cannot create a Trade Pool with a fee > 9000;
    let msg = ExecuteMsg::CreateTradePool {
        collection: collection.to_string(),
        asset_recipient: Some(asset_account.to_string()),
        bonding_curve: BondingCurve::Linear,
        spot_price: Uint128::from(1000u64),
        delta: Uint128::from(100u64),
        finders_fee_bps: 0,
        swap_fee_bps: 5001u64,
        reinvest_nfts: false,
        reinvest_tokens: false,
    };
    let res = router.execute_contract(
        creator.clone(),
        infinity_swap.clone(),
        &msg,
        &coins(LISTING_FEE, NATIVE_DENOM),
    );
    assert_error(
        res,
        ContractError::InvalidPool(String::from("swap_fee_percent cannot be greater than 50%")),
    );

    // Can create a Linear Trade Pool w/ no fee
    let msg = ExecuteMsg::CreateTradePool {
        collection: collection.to_string(),
        asset_recipient: Some(asset_account.to_string()),
        bonding_curve: BondingCurve::Linear,
        spot_price: Uint128::from(1000u64),
        delta: Uint128::from(100u64),
        finders_fee_bps: 0,
        swap_fee_bps: 0,
        reinvest_nfts: false,
        reinvest_tokens: false,
    };
    let res = router.execute_contract(
        creator.clone(),
        infinity_swap.clone(),
        &msg,
        &coins(LISTING_FEE, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Can create an Exponential Trade Pool
    let msg = ExecuteMsg::CreateTradePool {
        collection: collection.to_string(),
        asset_recipient: Some(asset_account.to_string()),
        bonding_curve: BondingCurve::Exponential,
        spot_price: Uint128::from(1000u64),
        delta: Uint128::from(100u64),
        finders_fee_bps: 0,
        swap_fee_bps: 200u64,
        reinvest_nfts: false,
        reinvest_tokens: false,
    };
    let res = router.execute_contract(
        creator.clone(),
        infinity_swap.clone(),
        &msg,
        &coins(LISTING_FEE, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Can create an Constant Product Trade Pool
    let msg = ExecuteMsg::CreateTradePool {
        collection: collection.to_string(),
        asset_recipient: Some(asset_account.to_string()),
        bonding_curve: BondingCurve::ConstantProduct,
        spot_price: Uint128::from(0u64),
        delta: Uint128::from(0u64),
        finders_fee_bps: 0,
        swap_fee_bps: 200u64,
        reinvest_nfts: false,
        reinvest_tokens: false,
    };
    let res = router.execute_contract(
        creator,
        infinity_swap,
        &msg,
        &coins(LISTING_FEE, NATIVE_DENOM),
    );
    assert!(res.is_ok());
}

#[test]
fn deposit_assets_trade_pool() {
    let vt = standard_minter_template(5000);
    let (mut router, minter, creator, user1) = (
        vt.router,
        vt.collection_response_vec[0].minter.as_ref().unwrap(),
        vt.accts.creator,
        vt.accts.bidder,
    );

    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let asset_account = Addr::unchecked(ASSET_ACCOUNT);

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let marketplace = setup_marketplace(&mut router, creator.clone()).unwrap();
    let infinity_swap = setup_infinity_swap(&mut router, creator.clone(), marketplace).unwrap();

    let pool = create_pool(
        &mut router,
        infinity_swap.clone(),
        creator.clone(),
        ExecuteMsg::CreateTradePool {
            collection: collection.to_string(),
            asset_recipient: Some(asset_account.to_string()),
            bonding_curve: BondingCurve::Linear,
            spot_price: Uint128::from(1000u64),
            delta: Uint128::from(100u64),
            finders_fee_bps: 0,
            swap_fee_bps: 0,
            reinvest_tokens: false,
            reinvest_nfts: false,
        },
    )
    .unwrap();

    // Only owner of pool can deposit tokens
    let deposit_amount = 1000u128;
    let msg = ExecuteMsg::DepositTokens { pool_id: pool.id };
    let res = router.execute_contract(
        user1.clone(),
        infinity_swap.clone(),
        &msg,
        &coins(deposit_amount, NATIVE_DENOM),
    );
    assert_error(
        res,
        ContractError::Unauthorized("sender is not the owner of the pool".to_string()),
    );

    // Owner can deposit tokens
    let deposit_amount = 1000u128;
    let msg = ExecuteMsg::DepositTokens { pool_id: pool.id };
    let res = router.execute_contract(
        creator.clone(),
        infinity_swap.clone(),
        &msg,
        &coins(deposit_amount, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Only owner can deposit NFTs into nft pool
    let token_id_1 = mint(&mut router, &user1, minter);
    approve(&mut router, &user1, &collection, &infinity_swap, token_id_1);
    let token_id_2 = mint(&mut router, &user1, minter);
    approve(&mut router, &user1, &collection, &infinity_swap, token_id_2);
    let msg = ExecuteMsg::DepositNfts {
        pool_id: pool.id,
        collection: collection.to_string(),
        nft_token_ids: vec![token_id_1.to_string(), token_id_2.to_string()],
    };
    let res = router.execute_contract(user1.clone(), infinity_swap.clone(), &msg, &[]);
    assert_error(
        res,
        ContractError::Unauthorized("sender is not the owner of the pool".to_string()),
    );

    // Owner can deposit NFTs into nft pool
    let token_id_1 = mint(&mut router, &creator, minter);
    approve(
        &mut router,
        &creator,
        &collection,
        &infinity_swap,
        token_id_1,
    );
    let token_id_2 = mint(&mut router, &creator, minter);
    approve(
        &mut router,
        &creator,
        &collection,
        &infinity_swap,
        token_id_2,
    );
    let msg = ExecuteMsg::DepositNfts {
        pool_id: pool.id,
        collection: collection.to_string(),
        nft_token_ids: vec![token_id_1.to_string(), token_id_2.to_string()],
    };
    let res = router.execute_contract(creator, infinity_swap, &msg, &[]);
    assert!(res.is_ok());
}
