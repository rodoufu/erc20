//! A simple implementation for parsing ERC20 transactions
#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

extern crate serde;
extern crate hex;

mod error;
pub mod util;
pub mod transfer;
pub mod erc20;
pub mod transaction;
#[cfg(test)]
mod transaction_test;

pub use self::error::ERC20Error;
