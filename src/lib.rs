#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

//! A simple implementation for parsing ERC20 transactions

extern crate serde;
extern crate hex;

mod error;
/// A set of useful methods and abstractions.
pub mod util;
/// Ethereum transfer abstraction.
pub mod transfer;
/// ERC20 specific information.
pub mod erc20;
/// web3 transaction specific operations.
pub mod transaction;
#[cfg(test)]
mod transaction_test;

pub use self::error::ERC20Error;
