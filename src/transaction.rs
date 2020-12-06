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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAndTransferType {
	transaction: Transaction,
	transfer_type: TransferType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ERC20Error {
	NoTransferTransaction,
}

impl TryFrom<Transaction> for TransactionAndTransferType {
	type Error = ERC20Error;

	fn try_from(value: Transaction) -> Result<Self, Self::Error> {
		let parsed_transaction: ParsedTransaction = value.into();
		match parsed_transaction {
			ParsedTransaction::EthereumTransfer(transaction) => Ok(Self {
				transaction, transfer_type: TransferType::Ethereum
			}),
			ParsedTransaction::ContractInvocation(transaction) => {
				let contract_invocation: TransactionContractInvocation = transaction.into();
				match contract_invocation {
					TransactionContractInvocation::ERC20(method, transaction) => {
						match method {
							ERC20Method::Transfer => Ok(Self{
								transaction,
								transfer_type: TransferType::ERC20,
							}),
							ERC20Method::TransferFrom => Ok(Self {
								transaction,
								transfer_type: TransferType::ERC20,
							}),
							_ => Err(ERC20Error::NoTransferTransaction),
						}
					},
					TransactionContractInvocation::Other(_) => Err(ERC20Error::NoTransferTransaction),
				}
			},
			ParsedTransaction::ContractCreation(_) => Err(ERC20Error::NoTransferTransaction),
			ParsedTransaction::Other(_) => Err(ERC20Error::NoTransferTransaction),
		}
	}
}

impl Transfer for TransactionAndTransferType {
	fn from(&self) -> H160 {
		self.transaction.from
	}

	fn to(&self) -> H160 {
		unimplemented!()
	}

	fn contract(&self) -> Option<H160> {
		match self.transfer_type {
			TransferType::Ethereum => None,
			TransferType::ERC20 => {
				match self.transaction.to {
					None => panic!("Unexpected transaction for TransactionAndTransferType"),
					Some(to) => Some(to),
				}
			}
		}
	}

	fn value(&self) -> U256 {
		unimplemented!()
	}

	fn tx_hash(&self) -> H256 {
		self.transaction.hash
	}

	fn block_hash(&self) -> Option<H256> {
		self.transaction.block_hash
	}

	fn block_number(&self) -> Option<U64> {
		self.transaction.block_number
	}

	fn transaction_index(&self) -> Option<Index> {
		self.transaction.transaction_index
	}
}
