///! Ethereum transfer abstraction.

use serde::{
	Deserialize,
	Serialize,
};
use web3::types::{
	BlockId,
	BlockNumber,
	H160,
	H256,
	Index,
	TransactionId,
	U64,
	U256,
};

/// Type of the asset transfer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferType {
	/// Indicates an Ether transfer.
	Ethereum,
	/// Indicates an ERC20 transfer.
	ERC20,
}

/// Asset transfer abstraction.
pub trait Transfer {
	/// Returns the sender of the transfer.
	fn from(&self) -> H160;
	/// Returns the recipient of the transfer.
	fn to(&self) -> H160;
	/// Returns the ERC20 contract address for ERC20 transfers.
	fn contract(&self) -> Option<H160>;
	/// Returns the value of the transfer.
	fn value(&self) -> U256;
	/// Returns the transaction hash for the transfer.
	fn tx_hash(&self) -> H256;
	/// Returns the block hash for the transfer, if available.
	fn block_hash(&self) -> Option<H256>;
	/// Returns the block number for the transfer, if available.
	fn block_number(&self) -> Option<U64>;
	/// Returns the transaction index for the transfer, if available.
	fn transaction_index(&self) -> Option<Index>;
}

impl dyn Transfer {
	pub fn kind(&self) -> TransferType {
		match self.contract() {
			None => TransferType::Ethereum,
			Some(_) => TransferType::ERC20,
		}
	}

	pub fn is_ethereum(&self) -> bool {
		self.kind() == TransferType::Ethereum
	}

	pub fn is_erc20(&self) -> bool { !self.is_ethereum() }

	#[allow(dead_code)]
	fn transaction_id(&self) -> TransactionId {
		if let Some(block_num) = self.block_number() {
			if let Some(t_idx) = self.transaction_index() {
				TransactionId::Block(BlockId::Number(BlockNumber::Number(block_num)), t_idx)
			} else {
				TransactionId::Hash(self.tx_hash())
			}
		} else {
			TransactionId::Hash(self.tx_hash())
		}
	}
}
