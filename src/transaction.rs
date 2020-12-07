use crate::{
	erc20::ERC20Method,
	error::ERC20Error,
	transfer::{
		TransferType,
		Transfer,
	},
	util::BytesToFixedNumber,
};
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
			Some(_) => if !transaction.input.0.is_empty() {
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
				transaction,
				transfer_type: TransferType::Ethereum,
			}),
			ParsedTransaction::ContractInvocation(transaction) => {
				let contract_invocation: TransactionContractInvocation = transaction.into();
				match contract_invocation {
					TransactionContractInvocation::ERC20(method, transaction) => {
						match method {
							ERC20Method::Transfer => Ok(Self {
								transaction,
								transfer_type: TransferType::ERC20,
							}),
							ERC20Method::TransferFrom => Ok(Self {
								transaction,
								transfer_type: TransferType::ERC20,
							}),
							_ => Err(ERC20Error::NoTransferTransaction),
						}
					}
					TransactionContractInvocation::Other(_) => Err(ERC20Error::NoTransferTransaction),
				}
			}
			ParsedTransaction::ContractCreation(_) => Err(ERC20Error::NoTransferTransaction),
			ParsedTransaction::Other(_) => Err(ERC20Error::NoTransferTransaction),
		}
	}
}

impl TransactionAndTransferType {
	pub fn get_from_to_value(&self) -> Result<(H160, H160, U256), ERC20Error> {
		let from_v: H160;
		let to_v: H160;
		let value_v: U256;
		match self.transfer_type {
			TransferType::Ethereum => {
				from_v = self.transaction.from;
				if let Some(v) = self.transaction.to {
					to_v = v;
				} else {
					return Err(ERC20Error::NoTransferTransaction);
				}
				value_v = self.transaction.value;
			}
			TransferType::ERC20 => {
				let contract_invocation: TransactionContractInvocation =
					self.transaction.clone().into();
				match contract_invocation {
					TransactionContractInvocation::ERC20(method, transaction) => {
						match method {
							ERC20Method::Transfer => {
								let mut resp: BytesToFixedNumber =
									transaction.input.clone().into();
								let _ignore = resp.next_vec(4);
								from_v = transaction.from;
								to_v = resp.next_h160()?;
								value_v = resp.next_u256()?;
							}
							ERC20Method::TransferFrom => {
								let mut resp: BytesToFixedNumber =
									transaction.input.clone().into();
								let _ignore = resp.next_vec(4);
								from_v = resp.next_h160()?;
								to_v = resp.next_h160()?;
								value_v = resp.next_u256()?;
							}
							_ => {
								return Err(ERC20Error::NoTransferTransaction);
							}
						}
					}
					TransactionContractInvocation::Other(_) => {
						return Err(ERC20Error::NoTransferTransaction);
					}
				}
			}
		}
		Ok((from_v, to_v, value_v))
	}
}

impl Transfer for TransactionAndTransferType {
	fn from(&self) -> H160 {
		match self.get_from_to_value() {
			Ok((the_from, _, _)) => the_from,
			Err(err) => {
				panic!("Unexpected transaction for TransactionAndTransferType::from {:?}", err)
			}
		}
	}

	fn to(&self) -> H160 {
		match self.get_from_to_value() {
			Ok((_, the_to, _)) => the_to,
			Err(err) => {
				panic!("Unexpected transaction for TransactionAndTransferType::to {:?}", err)
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
		match self.get_from_to_value() {
			Ok((_, _, the_value)) => the_value,
			Err(err) => {
				panic!("Unexpected transaction for TransactionAndTransferType::value {:?}", err)
			}
		}
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
