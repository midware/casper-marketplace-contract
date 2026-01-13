#![no_std]
#![no_main]
#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage, system,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, contracts::NamedKeys, runtime_args, ApiError, ContractHash, Key,
    RuntimeArgs, URef, U256, U512,
};
use entry_points::get_entry_points;
use utils::{
    contract_package_hash, get_acutin_dictionary, get_auction_data, get_listing_data, get_listing_dictionary, get_listing_key, get_offer_dictionary, get_offer_key, get_purse, get_token_owner, transfer_approved
};

mod entry_points;
mod error;
mod utils;

use error::Error;

const MILLISECONDS_IN_MINUTE: u64 = 60000;

const ARG_DURATION_MINUTES: &str = "duration_minutes";
const ARG_PRICE: &str = "price";
const ARG_TOKEN_ID: &str = "token_id";
const ARG_TOKEN_CONTRACT: &str = "contract_hash";
const ARG_BUY_PURSE: &str = "buy_purse";
const ARG_ROYALTIES_PERCENTAGE: &str = "royalties_percentage";

const CONTRACT_VERSION_KEY: &str = "version777";
const CONTRACT_KEY: &str = "mystra_marketplace777";

const CONTRACT_PACKAGE_NAME: &str = "mystra_marketplace_package_name777";
const CONTRACT_ACCESS_UREF: &str = "mystra_marketplace_access_uref777";

use casper_types_derive::{CLTyped, FromBytes, ToBytes};

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct ListingData {
    pub seller: Key,
    pub price: U512,
    pub expiration_time: Option<u64>,
}

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct AuctionData {
    pub seller: AccountHash,
    pub starting_price: U512,
    pub current_bid: U512,
    pub current_winner: AccountHash,
    pub end_time: u64,
}

#[derive(CLTyped, ToBytes, FromBytes)]
pub struct OfferData {
    pub price: U512,
    pub expiration_time: u64,
}

#[no_mangle]
pub extern "C" fn create_listing() -> () {
    let caller = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let price: U512 = runtime::get_named_arg(ARG_PRICE);
    let duration_in_minutes: u64 = runtime::get_named_arg(ARG_DURATION_MINUTES);

    let current_time: u64 = runtime::get_blocktime().into();

    if price == U512::zero() {
        runtime::revert(Error::PriceSetToZero)
    }

    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    let owner = get_token_owner(token_contract_hash, token_id).unwrap_or_revert();

    if owner != caller {
        runtime::revert(Error::PermissionDenied)
    }

    let approved = transfer_approved(token_contract_hash, token_id, caller);

    if approved == false {
        runtime::revert(Error::NeedsTransferApproval);
    }

    // aukcja musi nie istniec

    let key = get_listing_key(token_contract_hash, token_id);

    let expiration_time: Option<u64> = if duration_in_minutes > 0 {
        Some(current_time + duration_in_minutes * MILLISECONDS_IN_MINUTE)
    } else {
        None
    };

    let listing_data = ListingData {
        price: price,
        seller: owner,
        expiration_time: expiration_time,
    };

    storage::dictionary_put(get_listing_dictionary(), &key, listing_data)
}

#[no_mangle]
pub extern "C" fn cancel_listing() -> () {
    let caller = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    let owner: Key = get_token_owner(token_contract_hash, token_id).unwrap_or_revert();

    if owner != caller {
        runtime::revert(Error::PermissionDenied)
    }

    let key = get_listing_key(token_contract_hash, token_id);
    storage::dictionary_put(get_listing_dictionary(), &key, None::<ListingData>)
}

#[no_mangle]
pub extern "C" fn buy_listing() -> () {
    let buyer = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let buyer_purse: URef = runtime::get_named_arg(ARG_BUY_PURSE);
    let purse_balance: U512 = system::get_purse_balance(buyer_purse).unwrap();

    let key = get_listing_key(token_contract_hash, token_id);
    let listing_data: ListingData = get_listing_data(&key);

    if listing_data.price == U512::zero() {
        runtime::revert(Error::OfferDoesntExistOrCancelled)
    }

    if purse_balance < listing_data.price {
        runtime::revert(Error::BalanceInsufficient);
    }

    match listing_data.expiration_time {
        Some(val) => {
            let current_time: u64 = runtime::get_blocktime().into();

            if current_time > val {
                runtime::revert(Error::ListingExpired)
            }
        }
        None => {}
    }

    let owner = get_token_owner(token_contract_hash, token_id).unwrap();

    if owner != listing_data.seller {
        runtime::revert(Error::PermissionDenied)
    }

    system::transfer_from_purse_to_account(
        buyer_purse,
        owner.into_account().unwrap_or_revert(),
        listing_data.price,
        None,
    )
    .unwrap_or_revert();

    runtime::call_contract::<()>(
        token_contract_hash,
        "transfer_from",
        runtime_args! {
          "sender" => owner,
          "recipient" => buyer,
          "token_ids" => vec![token_id],
        },
    );

    storage::dictionary_put(get_listing_dictionary(), &key, None::<ListingData>);
}

#[no_mangle]
pub extern "C" fn make_offer() -> () {
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let offerer_purse: URef = runtime::get_named_arg(ARG_BUY_PURSE);
    let purse_balance: U512 = system::get_purse_balance(offerer_purse).unwrap();

    let key = get_offer_key(token_contract_hash, token_id, runtime::get_caller());

    let offers_purse: URef = get_purse("offers_purse");

    match storage::dictionary_get::<OfferData>(get_offer_dictionary(), &key) {
        Ok(d) => match d {
            Some(offer_data) => {
                system::transfer_from_purse_to_account(
                    offers_purse,
                    runtime::get_caller(),
                    offer_data.price,
                    None,
                )
                .unwrap_or_revert();
            }
            None => {},
        },
        Err(_error) => {}
        
    }

    system::transfer_from_purse_to_purse(offerer_purse, offers_purse, purse_balance, None)
        .unwrap_or_revert();

    let offer = OfferData {
        price: purse_balance,
        expiration_time: 0u64,
    };

    storage::dictionary_put(get_offer_dictionary(), &key, offer);
}

#[no_mangle]
pub extern "C" fn accept_offer() -> () {
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    let offerer_account_string: String = runtime::get_named_arg("offerer");
    let offerer_account_hash: AccountHash =
        AccountHash::from_formatted_str(&offerer_account_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    let key = get_offer_key(token_contract_hash, token_id, offerer_account_hash);

    let offers_purse = get_purse("offers_purse");

    let owner = get_token_owner(token_contract_hash, token_id).unwrap_or_revert();

    if owner != Key::Account(runtime::get_caller()) {
        runtime::revert(Error::PermissionDenied);
    }

    match storage::dictionary_get::<OfferData>(get_offer_dictionary(), &key).unwrap_or_revert_with(Error::OfferCancelledOrFinished) {
        Some(offer_data) => {
            system::transfer_from_purse_to_account(
                offers_purse,
                runtime::get_caller(),
                offer_data.price,
                None,
            )
            .unwrap_or_revert();
        }
        None => runtime::revert(Error::OfferDoesntExistOrCancelled),
    }

    runtime::call_contract::<()>(
        token_contract_hash,
        "transfer_from",
        runtime_args! {
          "sender" => Key::Account(runtime::get_caller()),
          "recipient" => Key::Account(offerer_account_hash),
          "token_ids" => vec![token_id],
        },
    );

    storage::dictionary_put(get_offer_dictionary(), &key, None::<OfferData>);
}

#[no_mangle]
pub extern "C" fn cancel_offer() -> () {
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    let key = get_offer_key(token_contract_hash, token_id, runtime::get_caller());

    let offers_purse = get_purse("offers_purse");

    let current_offer: OfferData = storage::dictionary_get(get_offer_dictionary(), &key)
        .unwrap_or_revert()
        .unwrap_or_revert_with(Error::OfferDoesntExistOrCancelled);

    system::transfer_from_purse_to_account(
        offers_purse,
        runtime::get_caller(),
        current_offer.price,
        None,
    )
    .unwrap_or_revert();

    storage::dictionary_put(get_offer_dictionary(), &key, None::<OfferData>);
}

#[no_mangle]
pub extern "C" fn start_auction() -> () {
    let caller = Key::Account(runtime::get_caller());
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let starting_price: U512 = runtime::get_named_arg(ARG_PRICE);
    let duration_in_minutes: u64 = runtime::get_named_arg(ARG_DURATION_MINUTES);

    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();

    let current_time: u64 = runtime::get_blocktime().into();

    let key = get_listing_key(token_contract_hash, token_id);

    let approved = transfer_approved(token_contract_hash, token_id, caller);

    if (approved == false) {
        runtime::revert(Error::NeedsTransferApproval);
    }

    let auction_data = AuctionData {
        current_bid: starting_price,
        starting_price: starting_price,
        seller: runtime::get_caller(),
        current_winner: runtime::get_caller(),
        end_time: current_time + (duration_in_minutes * MILLISECONDS_IN_MINUTE),
    };

    storage::dictionary_put(get_acutin_dictionary(), &key, auction_data)
}

#[no_mangle]
pub extern "C" fn place_bid() -> () {
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);
    let buyer_purse: URef = runtime::get_named_arg(ARG_BUY_PURSE);
    let purse_balance: U512 = system::get_purse_balance(buyer_purse).unwrap();

    let key = get_listing_key(token_contract_hash, token_id);
    let mut auction_data: AuctionData = get_auction_data(&key);
 
    if (purse_balance <= auction_data.current_bid) {
        revert(Error::BidTooLow)
    }

    let current_time: u64 = runtime::get_blocktime().into();

    if (current_time > auction_data.end_time) {
        runtime::revert(Error::AuctionEnded);
    }

    if (current_time - auction_data.end_time < 10 * MILLISECONDS_IN_MINUTE) {
        auction_data.end_time += 10 * MILLISECONDS_IN_MINUTE;
    }

    let auctions_purse: URef = get_purse("auctions_purse");

    if (auction_data.current_bid != auction_data.starting_price) {
        system::transfer_from_purse_to_account(
            auctions_purse,
            auction_data.seller,
            auction_data.current_bid,
            None,
        )
        .unwrap_or_revert();
    }

    system::transfer_from_purse_to_purse(buyer_purse, auctions_purse, purse_balance, None)
        .unwrap_or_revert();

    auction_data.current_bid = purse_balance;
    auction_data.current_winner = runtime::get_caller();

    storage::dictionary_put(get_acutin_dictionary(), &key, auction_data)
}

#[no_mangle]
pub extern "C" fn end_auction() -> () {
    let token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let token_contract_hash: ContractHash =
        ContractHash::from_formatted_str(&token_contract_string).unwrap();
    let token_id: U256 = runtime::get_named_arg(ARG_TOKEN_ID);

    let key = get_listing_key(token_contract_hash, token_id);
    let mut auction_data: AuctionData = get_auction_data(&key);

    let current_time: u64 = runtime::get_blocktime().into();

    if (current_time < auction_data.end_time) {
        runtime::revert(Error::AuctionNotFinished);
    }

    // listing musi nie istniec

    //sprawdzic approved i ownership, jak nie zgadza sie to zwrocic winnerowi kase

    let auctions_purse = get_purse("auctions_purse");

    if (auction_data.current_bid != auction_data.starting_price) {
        system::transfer_from_purse_to_account(
            auctions_purse,
            auction_data.current_winner.into(),
            auction_data.current_bid,
            None,
        )
        .unwrap_or_revert();

        runtime::call_contract::<()>(
            token_contract_hash,
            "transfer_from",
            runtime_args! {
              "sender" => Key::Account(auction_data.seller),
              "recipient" => Key::Account(auction_data.current_winner),
              "token_ids" => vec![token_id],
            },
        );
    }

    storage::dictionary_put(get_listing_dictionary(), &key, None::<AuctionData>)
}

#[no_mangle]
pub extern "C" fn set_royalties() -> () {
    let _token_contract_string: String = runtime::get_named_arg(ARG_TOKEN_CONTRACT);
    let _token_id: U256 = runtime::get_named_arg(ARG_ROYALTIES_PERCENTAGE);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut counter_named_keys = NamedKeys::new();

    let (stored_contract_hash, contract_version) = storage::new_contract(
        get_entry_points(),
        Some(counter_named_keys),
        Some(CONTRACT_PACKAGE_NAME.to_string()),
        Some(CONTRACT_ACCESS_UREF.to_string()),
    );

    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}
