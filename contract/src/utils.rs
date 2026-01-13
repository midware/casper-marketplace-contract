use alloc::{format, str, string::{String, ToString}, vec::{self, Vec}};
use casper_contract::{contract_api::{runtime, storage, system}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    account::AccountHash, runtime_args, system::CallStackElement, ContractHash, ContractPackageHash, Key, RuntimeArgs, URef, U256
};

use crate::{error::Error, AuctionData, ListingData, OfferData};
use hex::encode;

pub fn contract_package_hash() -> ContractPackageHash {
    let call_stacks = runtime::get_call_stack();
    let last_entry = call_stacks.last().unwrap_or_revert_with(4);

    let package_hash = match last_entry {
        CallStackElement::StoredContract { contract_package_hash, .. } => *contract_package_hash,
        _ => runtime::revert(5),
    };

    package_hash
}

pub fn transfer_approved(token_contract_hash: ContractHash, token_id: U256, owner: Key) -> bool {
    let approved = runtime::call_contract::<Option<Key>>(
        token_contract_hash,
        "get_approved",
        runtime_args! {
          "owner" => owner,
          "token_id" => token_id
        },
    )
    .unwrap_or_revert_with(Error::NeedsTransferApproval);
    let approved_hash = approved.into_hash().unwrap_or_revert();

    contract_package_hash().value() == approved_hash
}

pub fn get_token_owner(token_contract_hash: ContractHash, token_id: U256) -> Option<Key> {
    runtime::call_contract::<Option<Key>>(
        token_contract_hash,
        "owner_of",
        runtime_args! {
          "token_id" => token_id
        },
    )
}

pub fn get_dictionary_uref(key: &str) -> URef {
    match runtime::get_key(key) {
        Some(uref_key) => uref_key.into_uref().unwrap_or_revert(),
        None => storage::new_dictionary(key).unwrap_or_revert(),
    }
}

pub fn get_listing_key(token_contract_hash : ContractHash, token_id: U256) -> String {
    let key_string = format!("{}_{}", token_contract_hash.to_string(),token_id.to_string());
    let hashed = runtime::blake2b(key_string);
    hex::encode(hashed)
}

pub fn get_listing_dictionary() -> URef {
    get_dictionary_uref("listings")
}

pub fn get_offer_key(token_contract_hash : ContractHash, token_id: U256, bidder: AccountHash) -> String {
    let key_string = format!("{}_{bidder}_{}", token_contract_hash.to_string(),token_id.to_string());
    let hashed = runtime::blake2b(key_string);
    hex::encode(hashed)
}

pub fn get_offer_dictionary() -> URef {
    get_dictionary_uref("offers")
}

pub fn get_acutin_dictionary() -> URef {
    get_dictionary_uref("auctions")
}


pub fn get_purse(purse_name: &str) -> URef {
    let purse = if !runtime::has_key(&purse_name) {
        let purse = system::create_purse();
        runtime::put_key(&purse_name, purse.into());
        purse
    } else {
        let destination_purse_key = runtime::get_key(&purse_name).unwrap_or_revert_with(
            Error::OfferPurseRetrieval
        );
        match destination_purse_key.as_uref() {
            Some(uref) => *uref,
            None => runtime::revert(Error::OfferPurseRetrieval),
        }
    };
    return purse;
}


pub fn get_listing_data(key: &str) -> ListingData {
    let listing : ListingData =
        match storage::dictionary_get(get_listing_dictionary(), &key)  {
            Ok(item) => match item {
                None => runtime::revert(Error::ListingDoesntExist),
                Some(value) => value,
            },
            Err(_error) => runtime::revert(Error::ListingCancelledOrFinished)
        };

    listing
}

pub fn get_offer_data(key: &str) -> OfferData {
    let offer : OfferData =
        match storage::dictionary_get(get_offer_dictionary(), &key)  {
            Ok(item) => match item {
                None => runtime::revert(Error::OfferDoesntExist),
                Some(value) => value,
            },
            Err(_error) => runtime::revert(Error::OfferCancelledOrFinished)
        };

        offer
}


pub fn get_auction_data(key: &str) -> AuctionData {
    let auction : AuctionData =
        match storage::dictionary_get(get_acutin_dictionary(), &key)  {
            Ok(item) => match item {
                None => runtime::revert(Error::AuctionDoesntExist),
                Some(value) => value,
            },
            Err(_error) => runtime::revert(Error::AuctionCancelledOrFinished)
        };

        auction
}