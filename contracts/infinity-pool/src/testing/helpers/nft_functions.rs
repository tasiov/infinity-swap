use cosmwasm_std::{coins, Addr, Empty};
use cw_multi_test::Executor;
use sg721::ExecuteMsg as Sg721ExecuteMsg;
use sg721_base::msg::CollectionInfoResponse;
use sg_multi_test::StargazeApp;
use sg_std::NATIVE_DENOM;

pub const MINT_PRICE: u128 = 100_000_000;

// Mints an NFT for a creator
pub fn mint(router: &mut StargazeApp, creator: &Addr, minter_addr: &Addr) -> u32 {
    let minter_msg = vending_minter::msg::ExecuteMsg::Mint {};
    let res = router.execute_contract(
        creator.clone(),
        minter_addr.clone(),
        &minter_msg,
        &coins(MINT_PRICE, NATIVE_DENOM),
    );
    assert!(res.is_ok());
    res.unwrap().events[1].attributes[4].value.parse().unwrap()
}

pub fn mint_to(
    router: &mut StargazeApp,
    creator: &Addr,
    owner: &Addr,
    minter_addr: &Addr,
) -> String {
    let mint_for_creator_msg = vending_minter::msg::ExecuteMsg::MintTo {
        recipient: owner.to_string(),
    };
    let res = router
        .execute_contract(
            creator.clone(),
            minter_addr.clone(),
            &mint_for_creator_msg,
            &[],
        )
        .unwrap();

    let event = res.events.iter().find(|&e| e.ty == "wasm").unwrap();

    let token_id = event
        .attributes
        .iter()
        .find(|&a| a.key == "token_id")
        .unwrap()
        .value
        .clone();

    token_id
}

pub fn approve(
    router: &mut StargazeApp,
    creator: &Addr,
    collection: &Addr,
    marketplace: &Addr,
    token_id: u32,
) {
    let approve_msg: Sg721ExecuteMsg<CollectionInfoResponse, Empty> = Sg721ExecuteMsg::Approve {
        spender: marketplace.to_string(),
        token_id: token_id.to_string(),
        expires: None,
    };
    let res = router.execute_contract(creator.clone(), collection.clone(), &approve_msg, &[]);
    assert!(res.is_ok());
}

pub fn approve_all(router: &mut StargazeApp, owner: &Addr, collection: &Addr, approve_addr: &Addr) {
    let approve_msg: Sg721ExecuteMsg<CollectionInfoResponse, Empty> = Sg721ExecuteMsg::ApproveAll {
        operator: approve_addr.to_string(),
        expires: None,
    };
    let res = router.execute_contract(owner.clone(), collection.clone(), &approve_msg, &[]);
    assert!(res.is_ok());
}

pub fn _transfer(
    router: &mut StargazeApp,
    creator: &Addr,
    recipient: &Addr,
    collection: &Addr,
    token_id: u32,
) {
    let transfer_msg: Sg721ExecuteMsg<Empty, Empty> = Sg721ExecuteMsg::TransferNft {
        recipient: recipient.to_string(),
        token_id: token_id.to_string(),
    };
    let res = router.execute_contract(creator.clone(), collection.clone(), &transfer_msg, &[]);
    assert!(res.is_ok());
}

pub fn _burn(router: &mut StargazeApp, creator: &Addr, collection: &Addr, token_id: u32) {
    let transfer_msg: Sg721ExecuteMsg<Empty, Empty> = Sg721ExecuteMsg::Burn {
        token_id: token_id.to_string(),
    };
    let res = router.execute_contract(creator.clone(), collection.clone(), &transfer_msg, &[]);
    assert!(res.is_ok());
}

pub fn mint_and_approve_many(
    router: &mut StargazeApp,
    creator: &Addr,
    owner: &Addr,
    minter_addr: &Addr,
    collection: &Addr,
    approve_addr: &Addr,
    num_tokens: u32,
) -> Vec<String> {
    let mut token_ids = Vec::new();
    for _ in 0..num_tokens {
        let token_id = mint_to(router, creator, owner, minter_addr);
        token_ids.push(token_id);
    }
    approve_all(router, owner, collection, approve_addr);
    token_ids
}
