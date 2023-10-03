#![no_std]
#![feature(alloc)]

extern crate alloc;

use alloc::vec::Vec;
use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    account::AccountHash, contracts::EntryPoint, runtime_args, ContractHash, EntryPointType, EntryPoints, Parameter,
};

#[derive(Default)]
struct NFTBadges {
    id: u64,
    name: String,
    description: String,
    owners: Vec<AccountHash>,
}

impl NFTBadges {
    fn new(id: u64, name: String, description: String, owner: AccountHash) -> Self {
        let mut owners = Vec::new();
        owners.push(owner);

        NFTBadges {
            id,
            name,
            description,
            owners,
        }
    }
}

#[no_mangle]
pub extern "C" fn mint_badge() {
    let id: u64 = runtime::get_named_arg("id");
    let name: String = runtime::get_named_arg("name");
    let description: String = runtime::get_named_arg("description");
    let caller = runtime::get_caller();
    
    let badge = NFTBadges::new(id, name, description, caller);
    
    let key = format!("badge_{}", id);
    storage::put(key, badge);
}

#[no_mangle]
pub extern "C" fn transfer_badge() {
    let id: u64 = runtime::get_named_arg("id");
    let new_owner: AccountHash = runtime::get_named_arg("new_owner");
    let caller = runtime::get_caller();
    
    let key = format!("badge_{}", id);
    let mut badge: NFTBadges = storage::read(key.clone()).unwrap_or_default();

    if !badge.owners.contains(&caller) {
        runtime::revert("Permission denied: You are not the owner of this badge.");
    }

    if !badge.owners.contains(&new_owner) {
        runtime::revert("Invalid new owner.");
    }

    badge.owners.retain(|&owner| owner != caller);
    badge.owners.push(new_owner);

    storage::put(key, badge);
}

fn entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "mint_badge",
        vec![
            Parameter::new("id", u64::default()),
            Parameter::new("name", String::default()),
            Parameter::new("description", String::default()),
        ],
        NFTBadges::default(),
        EntryPointType::Session,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "transfer_badge",
        vec![
            Parameter::new("id", u64::default()),
            Parameter::new("new_owner", AccountHash::default()),
        ],
        NFTBadges::default(),
        EntryPointType::Session,
    ));
    entry_points
}

#[no_mangle]
pub extern "C" fn contract() {
    let entry_points = entry_points();
    let (contract_hash, _contract_version) = storage::new_contract(entry_points);
    runtime::put_key("nft_badge", contract_hash.into());
}






























#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::string::String;

// Define the NFT Badge struct
#[derive(Default, Serialize, Deserialize)]
struct NFTBadge {
    name: String,
    description: String,
    owners: Vec<AccountHash>,
}

// Define the main entry point of the contract
#[no_mangle]
pub extern "C" fn call() {
    // Get the runtime context
    let context = contract::get_context();

    // Dispatch based on the called method
    match context.method_name.as_str() {
        "mint" => mint_badge(context),
        "transfer" => transfer_badge(context),
        _ => contract::revert(),
    }
}

// Function to mint a new NFT badge
fn mint_badge(context: contract::ContractContext) {
    // Ensure that the caller is the contract owner
    if !is_owner(&context) {
        contract::revert();
    }

    // Parse badge details from contract arguments
    let name = contract::get_arg::<String>("name").unwrap_or_default();
    let description = contract::get_arg::<String>("description").unwrap_or_default();

    // Create a new badge
    let mut badge = NFTBadge {
        name,
        description,
        owners: vec![context.caller],
    };

    // Store the badge in contract storage
    contract::storage_put::<NFTBadge>(&context, &context.caller, &badge);
}

// Function to transfer ownership of an NFT badge
fn transfer_badge(context: contract::ContractContext) {
    // Parse badge ID and new owner from contract arguments
    let badge_id = contract::get_arg::<AccountHash>("badge_id").unwrap_or_default();
    let new_owner = contract::get_arg::<AccountHash>("new_owner").unwrap_or_default();

    // Get the current badge owner
    let mut badge = contract::storage_get::<NFTBadge>(&context, &badge_id).unwrap_or_default();

    // Ensure that the caller is one of the badge owners
    if !badge.owners.contains(&context.caller) {
        contract::revert();
    }

    // Remove the caller from the badge owners and add the new owner
    badge.owners.retain(|owner| *owner != context.caller);
    badge.owners.push(new_owner);

    // Update the badge in contract storage
    contract::storage_put::<NFTBadge>(&context, &badge_id, &badge);
}

// Helper function to check if the caller is the contract owner
fn is_owner(context: &contract::ContractContext) -> bool {
    context.caller == context.account
}






