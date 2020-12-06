use serde::{
	Deserialize,
	Serialize,
};
use std::convert::TryFrom;
use web3::types::{
	H160,
	H256,
	Index,
	Transaction,
	U64,
	U256,
};
use crate::transfer::{
	TransferType,
	Transfer,
};
use crate::erc20::ERC20Method;
use crate::error::ERC20Error;

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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAndTransferType {
	transaction: Transaction,
	transfer_type: TransferType,
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
		match self.transfer_type {
			TransferType::Ethereum => {
				if let Some(to) = self.transaction.to {
					to
				} else {
					panic!("Unexpected transaction for TransactionAndTransferType::to");
				}
			},
			TransferType::ERC20 => {
				let contract_invocation: TransactionContractInvocation = self.transaction.clone().into();
				match contract_invocation {
					TransactionContractInvocation::ERC20(method, transaction) => {
						match method {
							ERC20Method::Transfer => H160::zero(), // FIXME
							ERC20Method::TransferFrom => H160::zero(), // FIXME
							_ => {
								panic!("Unexpected transaction for TransactionAndTransferType::to invalid ERC20 method");
							}
						}
					}
					TransactionContractInvocation::Other(_) => {
						panic!("Unexpected transaction for TransactionAndTransferType::to other");
					}
				}
			}
		}
	}

	fn contract(&self) -> Option<H160> {
		match self.transfer_type {
			TransferType::Ethereum => None,
			TransferType::ERC20 => {
				match self.transaction.to {
					None => panic!("Unexpected transaction for TransactionAndTransferType::contract"),
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

