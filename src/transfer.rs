use maplit::hashmap;
use serde::{
	Deserialize,
	Serialize,
};
use std::{
	convert::TryFrom,
	collections::HashMap,
};
use web3::types::{
	BlockId,
	BlockNumber,
	H160,
	H256,
	Index,
	Res,
	Transaction,
	TransactionId,
	U64,
	U256,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferType {
	Ethereum,
	ERC20,
}

pub trait Transfer {
	fn from(&self) -> H160;
	fn to(&self) -> H160;
	fn contract(&self) -> Option<H160>;
	fn value(&self) -> U256;
	fn tx_hash(&self) -> H256;
	fn block_hash(&self) -> Option<H256>;
	fn block_number(&self) -> Option<U64>;
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
