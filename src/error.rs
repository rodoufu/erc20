//! Expected errors.

use serde::{
	Deserialize,
	Serialize,
};

/// Possible transaction errors.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ERC20Error {
	/// Returned when the transaction is not a Ethereum transfer neither an ERC20 transfer.
	NoTransferTransaction,
	/// Unexpected size for the input.
	UnexpectedSize,
	/// The end of the input was found before expected.
	UnexpectedEndOfData,
	/// Returned when the type or value used is not expected for the operation.
	UnexpectedType,
}
