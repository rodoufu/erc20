#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

//! A simple implementation for parsing ERC20 transactions

extern crate serde;

mod error;
/// A set of useful methods and abstractions.
pub mod util;
#[cfg(test)]
mod util_tests;
/// Ethereum transfer abstraction.
pub mod transfer;
/// ERC20 specific information.
pub mod erc20;
#[cfg(test)]
mod erc20_tests;
/// web3 transaction specific operations.
pub mod transaction;
#[cfg(test)]
mod transaction_tests;

pub use self::error::ERC20Error;
