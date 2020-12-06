use maplit::hashmap;
use serde::{
	Deserialize,
	Serialize,
};
use std::collections::HashMap;
use web3::types::{
	H160,
	H256,
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
	fn block_hash(&self) -> H256;
	fn block_number(&self) -> U64;
	fn transaction_id(&self) -> TransactionId;
}

impl dyn Transfer {
	pub fn kind(&self) -> TransferType {
		if self.contract().is_none() {
			TransferType::Ethereum
		} else {
			TransferType::ERC20
		}
	}
	pub fn is_ethereum(&self) -> bool { self.kind() == TransferType::Ethereum }
	pub fn is_erc20(&self) -> bool { !self.is_ethereum() }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ParsedTransaction {
	EthereumTransfer(Transaction),
	ContractInvocation(TransactionContractInvocation),
	ContractCreation(Transaction),
	Other(Transaction),
}

impl From<Transaction> for ParsedTransaction {
	#[inline]
	fn from(transaction: Transaction) -> Self {
		match transaction.to {
			None => if transaction.input.0.is_empty() {
				Self::Other(transaction)
			} else {
				Self::ContractCreation(transaction)
			},
			Some(_) => if transaction.input.0.is_empty() {
				Self::ContractInvocation(transaction.into())
			} else {
				Self::Other(transaction)
			},
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TransactionContractInvocation {
	ERC20(ERC20Method, Transaction),
	Other(Transaction),
}

impl From<Transaction> for TransactionContractInvocation {
	fn from(transaction: Transaction) -> Self {
		let method = transaction.clone().input.0.into();
		match method {
			ERC20Method::Unidentified => Self::Other(transaction),
			_ => Self::ERC20(method, transaction),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ERC20Method {
	Allowance,
	Approve,
	BalanceOf,
	TotalSupply,
	Transfer,
	TransferFrom,
	Unidentified,
}

impl From<Vec<u8>> for ERC20Method {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		if data.is_empty() || data.len() < 4 {
			Self::Unidentified
		} else {
			let method_encoding: HashMap<Self, Vec<u8>> = hashmap! {
				Self::Allowance => vec![0xdd, 0x62, 0xed, 0x3e],
				Self::Approve => vec![0x09, 0x5e, 0xa7, 0xb3],
				Self::BalanceOf => vec![0x70, 0xa0, 0x82, 0x31],
				Self::TotalSupply => vec![0x18, 0x16, 0x0d, 0xdd],
				Self::Transfer => vec![0xa9, 0x05, 0x9c, 0xbb],
				Self::TransferFrom => vec![0x23, 0xb8, 0x72, 0xdd],
			};
			for (key, value) in method_encoding {
				if data.starts_with(&value) {
					return key;
				}
			}
			Self::Unidentified
		}
	}
}
