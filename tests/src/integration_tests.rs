#[cfg(test)]
mod tests {

    mod cep47_helpers;
    mod marketplace_actions;

    use std::{thread, time::Duration};

    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR,
        PRODUCTION_RUN_GENESIS_REQUEST,
    };
    use casper_execution_engine::core::engine_state::ExecuteRequest;
    use casper_types::{
        account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs, U256, U512,
    };
    use cep47_helpers::{approve_cep_47, deploy_cep_47, mint_cep_47};
    use marketplace_actions::{
        build_create_listing_request, create_buy_nft_request, create_make_offer_request,
    };

    use self::marketplace_actions::{
        build_accept_offer_request, build_cancel_listing_request, build_cancel_offer_request, build_end_auction_request, build_start_auction_request, create_place_bid_request
    };

    // Contract Wasm File Paths (Constants)
    const MARKETPLACE_WASM: &str = "contract.wasm";
    const CEP47_WASM: &str = "cep47-token.wasm";
    const PAYMENT_WASM: &str = "payment-call.wasm";
    const OFFER_WASM: &str = "make-offer-call.wasm";
    const BID_WASM: &str = "bid-call.wasm";

    // Contract Storage Keys (Constants)
    const CONTRACT_KEY: &str = "mystra_marketplace777";

    // Contract Entry Points (Constants)
    const ENTRY_POINT_CREATE_LISTING: &str = "create_listing";
    const ENTRY_POINT_ACCEPT_OFFER: &str = "accept_offer";
    const ENTRY_POINT_CANCEL_OFFER: &str = "cancel_offer";

    /// Deploys a contract version to the InMemoryWasmTestBuilder
    fn deploy_marketplace(builder: &mut InMemoryWasmTestBuilder, wasm_code: &str) -> ContractHash {
        let request =
            ExecuteRequestBuilder::standard(*DEFAULT_ACCOUNT_ADDR, wasm_code, runtime_args! {})
                .build();
        builder.exec(request).expect_success().commit();
        get_contract_hash_from_account(builder, CONTRACT_KEY)
    }

    fn print_account_balance(builder: &mut InMemoryWasmTestBuilder, account_hash: AccountHash) {
        // Zapytanie do stanu o strukturę konta
        let account_result = builder
            .query(None, Key::Account(account_hash), &[])
            .expect("Should have account data");

        // Teraz przekształcamy wynik zapytania w konto
        let account = account_result.as_account().expect("Should be account");

        // Odczytujemy URef portfela z danych konta
        let main_purse = account.main_purse();

        // Odczytujemy balans z URef portfela
        let balance = builder.get_purse_balance(main_purse);

        println!("Balance for account {}: {}", account_hash, balance);
    }


    /// Retrieves the contract hash from the default account's storage by a given key
    fn get_contract_hash_from_account(
        builder: &mut InMemoryWasmTestBuilder,
        key: &str,
    ) -> ContractHash {
        builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(key)
            .expect("must have contract hash key")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash")
    }

    #[test]
    fn install_and_test() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder
            .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
            .commit();

        let ACCOUNT_ONE: AccountHash = *DEFAULT_ACCOUNT_ADDR;
        let ACCOUNT_TWO: AccountHash = DEFAULT_ACCOUNTS.get(1).unwrap().account_hash();

        let marketplace_hash = deploy_marketplace(&mut builder, MARKETPLACE_WASM);
        let nft_hash = deploy_cep_47(&mut builder);

        mint_cep_47(
            &mut builder,
            nft_hash,
            (ACCOUNT_ONE).into(),
            vec![U256::from(1)],
        );

        mint_cep_47(
            &mut builder,
            nft_hash,
            (ACCOUNT_ONE).into(),
            vec![U256::from(2)],
        );
        mint_cep_47(
            &mut builder,
            nft_hash,
            (ACCOUNT_ONE).into(),
            vec![U256::from(3)],
        );

        let marketplace_contract_package_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get("mystra_marketplace_package_name777")
            .expect("must have contract hash key")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        approve_cep_47(
            &mut builder,
            ACCOUNT_ONE,
            nft_hash,
            marketplace_contract_package_hash.into(),
            vec![U256::from(1)],
        );

        approve_cep_47(
            &mut builder,
            ACCOUNT_ONE,
            nft_hash,
            marketplace_contract_package_hash.into(),
            vec![U256::from(2)],
        );

        // Cant buy before listing
        let req = create_buy_nft_request(ACCOUNT_TWO, U256::from(1), marketplace_hash, nft_hash);
        builder.exec(req).expect_failure().commit();

        let req = build_create_listing_request(
            ACCOUNT_ONE,
            marketplace_hash,
            nft_hash,
            U256::from(1),
            U512::from(100u64),
            0u64,
        );
        builder.exec(req).expect_success().commit();

        let req = build_cancel_listing_request(ACCOUNT_ONE, marketplace_hash, nft_hash, U256::from(1));
        builder.exec(req).expect_success().commit();

        let req = create_buy_nft_request(ACCOUNT_TWO, U256::from(1), marketplace_hash, nft_hash);
        builder.exec(req).expect_failure().commit();

        let req = build_create_listing_request(
            ACCOUNT_ONE,
            marketplace_hash,
            nft_hash,
            U256::from(1),
            U512::from(100u64),
            0u64,
        );
        builder.exec(req).expect_success().commit();

        let req = build_create_listing_request(
            ACCOUNT_ONE,
            marketplace_hash,
            nft_hash,
            U256::from(1),
            U512::from(100u64),
            15u64,
        );
        builder.exec(req).expect_success().commit();

        let req = create_buy_nft_request(ACCOUNT_TWO, U256::from(1), marketplace_hash, nft_hash);
        builder.exec(req).expect_success().commit();


        // Offer 

        let req = create_make_offer_request(ACCOUNT_TWO, U256::from(2), marketplace_hash, nft_hash);
        builder.exec(req).expect_success().commit();

        let req = build_cancel_offer_request(ACCOUNT_TWO,  marketplace_hash, nft_hash,U256::from(2));
        builder.exec(req).expect_success().commit();

        let req = build_accept_offer_request(ACCOUNT_ONE,  marketplace_hash, nft_hash,U256::from(2));
        builder.exec(req).expect_failure().commit();

        let req = create_make_offer_request(ACCOUNT_TWO, U256::from(2), marketplace_hash, nft_hash);
        builder.exec(req).expect_success().commit();

        let req = build_accept_offer_request(ACCOUNT_ONE,  marketplace_hash, nft_hash,U256::from(2));
        builder.exec(req).expect_success().commit();

        // Auction


        approve_cep_47(
            &mut builder,
            ACCOUNT_ONE,
            nft_hash,
            marketplace_contract_package_hash.into(),
            vec![U256::from(3)],
        );


        let req = build_start_auction_request(ACCOUNT_ONE,  marketplace_hash, nft_hash,U256::from(3));
        builder.exec(req).expect_success().commit();

        let req = create_place_bid_request(ACCOUNT_TWO,U256::from(3), U512::from(2),  marketplace_hash, nft_hash, 5u64);
        builder.exec(req).expect_success().commit();

        let req = create_place_bid_request(ACCOUNT_ONE,U256::from(3),U512::from(2),  marketplace_hash, nft_hash, 5u64);
        builder.exec(req).expect_failure().commit();

        let req = create_place_bid_request(ACCOUNT_ONE,U256::from(3),U512::from(3),  marketplace_hash, nft_hash, 5u64);
        builder.exec(req).expect_success().commit();
        
        let req = create_place_bid_request(ACCOUNT_ONE,U256::from(3),U512::from(10),  marketplace_hash, nft_hash, 60000 * 100);
        builder.exec(req).expect_failure().commit();

        let req = create_place_bid_request(ACCOUNT_ONE,U256::from(3),U512::from(10),  marketplace_hash, nft_hash, 60000 * 100);
        builder.exec(req).expect_failure().commit();

        let req = build_end_auction_request(ACCOUNT_ONE,  marketplace_hash, nft_hash, U256::from(3),  60000 * 100);
        builder.exec(req).expect_success().commit();
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
