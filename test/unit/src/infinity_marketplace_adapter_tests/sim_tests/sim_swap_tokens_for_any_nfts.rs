use crate::helpers::marketplace_functions::set_ask;
use crate::helpers::nft_functions::{approve, mint_and_approve_many};
use crate::helpers::swap_functions::{setup_swap_test, SwapTestSetup};
use crate::setup::msg::MarketAccounts;
use crate::setup::setup_marketplace::MIN_EXPIRY;
use cosmwasm_std::Uint128;
use infinity_marketplace_adapter::msg::QueryMsg as InfinityAdapterQueryMsg;
use infinity_shared::interface::{
    NftPayment, Swap, SwapParams, SwapResponse, TokenPayment, TransactionType,
};
use sg_marketplace::state::SaleType;
use test_suite::common_setup::msg::MinterTemplateResponse;

#[test]
fn sim_token_for_any_nfts_marketplace_adapter() {
    let SwapTestSetup {
        vending_template:
            MinterTemplateResponse {
                mut router,
                accts:
                    MarketAccounts {
                        creator,
                        bidder,
                        owner,
                    },
                collection_response_vec,
                ..
            },
        marketplace,
        infinity_marketplace_adapter,
        ..
    } = setup_swap_test(5000).unwrap();

    let collection_resp = &collection_response_vec[0];
    let minter = collection_resp.minter.clone().unwrap();
    let collection = collection_resp.collection.clone().unwrap();
    let block_time = router.block_info().time;

    let owner_token_ids = mint_and_approve_many(
        &mut router,
        &creator,
        &owner,
        &minter,
        &collection,
        &marketplace,
        4,
    );

    /*
        Test Ask Matching:
            * Test that only Asks below or equal to the desired price are taken
            * Test that they are taken in the order of price ascending
            * Test that multiple orders can be filled

        Asks
            ask_0: token_id_0, 999 STARS
            ask_1: token_id_1, 1001 STARS
            ask_2: token_id_2, None
            ask_3: token_id_3, 1000 STARS

        Match Order
            token_id_0: ask_0
            token_id_3: ask_3
    */

    // Setup Asks
    let expires = block_time.plus_seconds(MIN_EXPIRY + 1);
    for (idx, token_id) in owner_token_ids.iter().enumerate() {
        let token_id: u32 = token_id.parse().unwrap();
        approve(&mut router, &owner, &collection, &marketplace, token_id);

        let ask_amount = match idx {
            0 => Some(999u128),
            1 => Some(1001u128),
            2 => None,
            3 => Some(1000u128),
            _ => unreachable!(),
        };
        if let Some(_ask_amount) = ask_amount {
            set_ask(
                &mut router,
                marketplace.clone(),
                owner.clone(),
                _ask_amount,
                collection.to_string(),
                token_id,
                expires,
                SaleType::FixedPrice,
                None,
                None,
                None,
            );
        }
    }

    let sim_msg = InfinityAdapterQueryMsg::SimSwapTokensForAnyNfts {
        sender: bidder.to_string(),
        collection: collection.to_string(),
        nft_orders: owner_token_ids
            .iter()
            .map(|_| Uint128::from(1000u128))
            .collect(),
        swap_params: SwapParams {
            deadline: expires,
            robust: true,
            asset_recipient: None,
            finder: None,
        },
    };
    let swap_response: SwapResponse = router
        .wrap()
        .query_wasm_smart(infinity_marketplace_adapter.clone(), &sim_msg)
        .unwrap();

    assert_eq!(swap_response.swaps.len(), 2);

    let swap_0 = swap_response.swaps[0].clone();
    assert_eq!(
        swap_0,
        Swap {
            source: marketplace.to_string(),
            transaction_type: TransactionType::UserSubmitsTokens,
            sale_price: Uint128::from(999u128),
            network_fee: Uint128::from(19u128),
            nft_payments: vec![NftPayment {
                label: "buyer".to_string(),
                token_id: owner_token_ids[0].to_string(),
                address: bidder.to_string()
            }],
            token_payments: vec![
                TokenPayment {
                    label: "royalty".to_string(),
                    amount: Uint128::from(99u128),
                    address: creator.to_string()
                },
                TokenPayment {
                    label: "seller".to_string(),
                    amount: Uint128::from(881u128),
                    address: owner.to_string()
                }
            ]
        }
    );

    let swap_1 = swap_response.swaps[1].clone();
    assert_eq!(
        swap_1,
        Swap {
            source: marketplace.to_string(),
            transaction_type: TransactionType::UserSubmitsTokens,
            sale_price: Uint128::from(1000u128),
            network_fee: Uint128::from(20u128),
            nft_payments: vec![NftPayment {
                label: "buyer".to_string(),
                token_id: owner_token_ids[3].to_string(),
                address: bidder.to_string()
            }],
            token_payments: vec![
                TokenPayment {
                    label: "royalty".to_string(),
                    amount: Uint128::from(100u128),
                    address: creator.to_string()
                },
                TokenPayment {
                    label: "seller".to_string(),
                    amount: Uint128::from(880u128),
                    address: owner.to_string()
                }
            ]
        }
    );
}
