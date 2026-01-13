use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNTS, DEFAULT_ACCOUNT_ADDR,
};
use casper_execution_engine::core::engine_state::ExecuteRequest;
use casper_types::{account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs, U256, U512};

use super::{BID_WASM, ENTRY_POINT_ACCEPT_OFFER, ENTRY_POINT_CANCEL_OFFER, ENTRY_POINT_CREATE_LISTING, OFFER_WASM, PAYMENT_WASM};

pub fn build_create_listing_request(
    caller: AccountHash,
    marketplace_hash: ContractHash,
    cep47_hash: ContractHash,
    token_id: U256,
    price: U512,
    expiration_time: u64
) -> ExecuteRequest {

    ExecuteRequestBuilder::contract_call_by_hash(
        caller,
        marketplace_hash,
        ENTRY_POINT_CREATE_LISTING,
        runtime_args! {
            "token_id" => token_id,
            "contract_hash" => cep47_hash.to_formatted_string(),
            "price" => price,
            "duration_minutes" => expiration_time

        },
    )
    .build()
}

pub fn create_buy_nft_request(
    caller: AccountHash,
    token_id: U256,
    marketplace_hash: ContractHash,
    contract_hash: ContractHash,
) -> ExecuteRequest {
    ExecuteRequestBuilder::standard(
        caller,
        PAYMENT_WASM,
        runtime_args! {
            "token_id" => token_id,
            "buy_contract_hash" => contract_hash.to_formatted_string(),
            "marketplace_hash" => marketplace_hash.to_formatted_string(),
            "amount" => U512::from(1000000000000u64)
        },
    )
    .with_block_time(40).build()
}

pub fn create_make_offer_request(
    caller: AccountHash,
    token_id: U256,
    marketplace_hash: ContractHash,
    contract_hash: ContractHash,
) -> ExecuteRequest {
    ExecuteRequestBuilder::standard(
        caller,
        OFFER_WASM,
        runtime_args! {
            "token_id" => token_id,
            "offer_contract_hash" => contract_hash.to_formatted_string(),
            "marketplace_hash" => marketplace_hash.to_formatted_string(),
            "amount" => U512::from(100000000u64)
        },
    )
    .build()
}

pub fn build_accept_offer_request(
    caller: AccountHash,
    marketplace_hash: ContractHash,
    cep47_hash: ContractHash,
    token_id: U256,
) -> ExecuteRequest {

    ExecuteRequestBuilder::contract_call_by_hash(
        caller,
        marketplace_hash,
        ENTRY_POINT_ACCEPT_OFFER,
        runtime_args! {
            "token_id" => token_id,
            "contract_hash" => cep47_hash.to_formatted_string(),
            "offerer" => (DEFAULT_ACCOUNTS.get(1).unwrap().account_hash()).to_formatted_string()
        },
    )
    .build()
}

pub fn build_cancel_offer_request(
    caller: AccountHash,
    marketplace_hash: ContractHash,
    cep47_hash: ContractHash,
    token_id: U256,
) -> ExecuteRequest {

    ExecuteRequestBuilder::contract_call_by_hash(
        caller,
        marketplace_hash,
        ENTRY_POINT_CANCEL_OFFER,
        runtime_args! {
            "token_id" => token_id,
            "contract_hash" => cep47_hash.to_formatted_string(),
        },
    )
    .build()
}

pub fn build_cancel_listing_request(
    
    caller: AccountHash,
    marketplace_hash: ContractHash,
    cep47_hash: ContractHash,
    token_id: U256,
) -> ExecuteRequest {

    ExecuteRequestBuilder::contract_call_by_hash(
        caller,
        marketplace_hash,
        "cancel_listing",
        runtime_args! {
            "token_id" => token_id,
            "contract_hash" => cep47_hash.to_formatted_string(),
        },
    )
    .build()
}

pub fn build_start_auction_request(
    caller: AccountHash,
    marketplace_hash: ContractHash,
    cep47_hash: ContractHash,
    token_id: U256,

) -> ExecuteRequest {

    ExecuteRequestBuilder::contract_call_by_hash(
        caller,
        marketplace_hash,
        "start_auction",
        runtime_args! {
            "token_id" => token_id,
            "contract_hash" => cep47_hash.to_formatted_string(),
            "duration_minutes" => 20u64,
            "price" => U512::from(1)
        },
    )
    .build()
}




pub fn create_place_bid_request(
    caller: AccountHash,
    token_id: U256,
    bid: U512,
    marketplace_hash: ContractHash,
    contract_hash: ContractHash,
    blocktime: u64
) -> ExecuteRequest {
    ExecuteRequestBuilder::standard(
        caller,
        BID_WASM,
        runtime_args! {
            "token_id" => token_id,
            "bid_contract_hash" => contract_hash.to_formatted_string(),
            "marketplace_hash" => marketplace_hash.to_formatted_string(),
            "amount" => bid
        },
    )
    .with_block_time(blocktime).build()
}


pub fn build_end_auction_request(
    caller: AccountHash,
    marketplace_hash: ContractHash,
    cep47_hash: ContractHash,
    token_id: U256,
    blocktime: u64

) -> ExecuteRequest {

    ExecuteRequestBuilder::contract_call_by_hash(
        caller,
        marketplace_hash,
        "end_auction",
        runtime_args! {
            "token_id" => token_id,
            "contract_hash" => cep47_hash.to_formatted_string(),
        },
    )
    .with_block_time(blocktime).build()
}

