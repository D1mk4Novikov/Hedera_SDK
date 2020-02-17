#![feature(async_await, await_macro, futures_api)]
#![warn(clippy::pedantic, future_incompatible, unreachable_pub)]
#![allow(clippy::stutter, clippy::new_ret_no_self, clippy::module_inception)]


extern crate libc;


use std::str;
use libc::c_char;


mod utf_conversion;
use utf_conversion::*;

mod json_representation;
use json_representation::*;


#[macro_use]
mod macros;

mod claim;
pub mod client;
mod crypto;
mod duration;
mod entity;
mod error;
mod id;
mod info;
mod proto;
pub mod query;
mod status;
mod timestamp;
pub mod transaction;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;

pub use self::{
    claim::Claim,
    client::Client,
    crypto::{PublicKey, SecretKey, Signature},
    entity::Entity,
    error::ErrorKind,
    id::*,
    info::{AccountInfo, ContractInfo, FileInfo},
    status::Status,
    transaction_id::TransactionId,
    transaction_receipt::TransactionReceipt,
    transaction_record::{TransactionRecord, TransactionRecordBody},
};

use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Mutex<Runtime>> = sync_lazy! { Mutex::new(Runtime::new().unwrap()) };



// - - - - - - - - - - - - - - - - - - - - - - - - - -
// Convert data to vector strings
fn get_account_info_func(user_account: &str, node_address: &str, node_account: &str, user_private_key: &'static str) /*Vec<&'static str>*/ {
    
    let operator = user_account.parse().unwrap();
    let client = Client::builder(node_address)
        .node(node_account.parse().unwrap())
        .operator(operator, move || user_private_key)
        .build()
        .unwrap();
    let _balance = client.account(operator).balance().get();
    //println!("SUKA BALANS = {}", balance.to_string());
    let info = client.account(operator).info().get();
    println!("INFA ob EBLE = {:#?}", info);
    
    /*
    let temporary_user_account = user_account.parse().unwrap();
    let client = Client::builder(node_address)
        .node(node_account.parse().unwrap())
        .operator(temporary_user_account, move || user_private_key)
        .build();
    let balance = client.account(temporary_user_account).balance().get().unwrap();
    println!("Balance = {} tinybars", balance);
    let info = client.account(temporary_user_account).info().get().unwrap();
    println!("Account info = {:#?}", info);
    */
}


// Convert vector to JSON representation
#[no_mangle]
pub extern fn get_account_info(user_account: *const c_char, node_address: *const c_char, node_account: *const c_char, user_private_key: *const c_char) /*const c_char*/ {
    println!("User account = {}", convert_to_utf8(user_account));
    println!("Node address = {}", convert_to_utf8(node_address));
    println!("Node account = {}", convert_to_utf8(node_account));
    println!("User private key = {}", convert_to_utf8(user_private_key));
    get_account_info_func(convert_to_utf8(user_account), convert_to_utf8(node_address), convert_to_utf8(node_account), convert_to_utf8(user_private_key));
    //create_json_representation(1, get_account_info_func(convert_to_utf8(user_account), convert_to_utf8(node_address), convert_to_utf8(node_account), convert_to_utf8(user_private_key)))
}
// - - - - - - - - - - - - - - - - - - - - - - - - - -




// - - - - - - - - - - - - - - - - - - - - - - - - - -
/*
// Convert data to vector strings
fn create_file_from_file_func<'a>(user_account: &str, node_address: &str, node_account: &str, user_private_key: &'static str, path_to_file: &'static str) -> &'a str/*Vec<&'static str>*/ {
    
}


// Convert vector to JSON representation
#[no_mangle]
pub extern fn create_file_from_file(user_account: *const c_char, node_address: *const c_char, node_account: *const c_char, user_private_key: *const c_char, path_to_file: *const c_char) -> *const c_char {
    convert_to_utf16(create_file_from_file_func(convert_to_utf8(user_account), convert_to_utf8(node_address), convert_to_utf8(node_account), convert_to_utf8(user_private_key), convert_to_utf8(path_to_file)))
}
*/
// - - - - - - - - - - - - - - - - - - - - - - - - - -




// - - - - - - - - - - - - - - - - - - - - - - - - - -
// Convert data to vector strings
fn generate_keys_func() -> Vec<&'static str> {
    let (private_key, mnemonic) = SecretKey::generate("");
    let public_key = private_key.public();
    let output_str1: &'static str = Box::leak(private_key.to_string().into_boxed_str());
    let output_str2: &'static str = Box::leak(public_key.to_string().into_boxed_str());
    let output_str3: &'static str = Box::leak(mnemonic.to_string().into_boxed_str());
    vec![
        output_str1,
        output_str2,
        output_str3
    ]
}


// Convert vector to JSON representation
#[no_mangle]
pub extern fn generate_keys() -> *const c_char {
    create_json_representation(5, generate_keys_func())
}
// - - - - - - - - - - - - - - - - - - - - - - - - - -




// - - - - - - - - - - - - - - - - - - - - - - - - - -
// Get Hedera SDK version
#[no_mangle]
pub extern fn get_sdk_version() -> *const c_char {
    convert_to_utf16(env!("CARGO_PKG_VERSION"))
}
// - - - - - - - - - - - - - - - - - - - - - - - - - -