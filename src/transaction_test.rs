use crate::{transfer::Transfer, transaction::TransactionAndTransferType, util::string_to_h256, ERC20Error};
use std::convert::TryInto;
use web3::types::{Bytes, Transaction, H160};

#[test]
fn parse_no_transfer_transaction() {
	let serialized_str = "a9059cbb0000000000000000000000006748f50f686bfbca6fe8ad62b22228b87f31ff2b00000000000000000000000000000000000000000000003635c9adc5dea00000";

	let transaction = Transaction {
		hash: string_to_h256("43a5d6d13b6a9dca381e3f4b4677a4b9e5d9f80d1a5b6cfa2b1404fab733bcee".to_string()).unwrap(),
		nonce: Default::default(),
		block_hash: None,
		block_number: None,
		transaction_index: None,
		from: Default::default(),
		to: None,
		value: Default::default(),
		gas_price: Default::default(),
		gas: Default::default(),
		input: Bytes(hex::decode(serialized_str).unwrap()),
		raw: None,
	};

	let resp: Result<TransactionAndTransferType, ERC20Error> = transaction.try_into();
	assert!(resp.is_err());
	assert_eq!(ERC20Error::NoTransferTransaction, resp.err().unwrap());
}

#[test]
fn parse_erc20() {
	let serialized_str = "a9059cbb0000000000000000000000006748f50f686bfbca6fe8ad62b22228b87f31ff2b00000000000000000000000000000000000000000000003635c9adc5dea00000";

	let transaction = Transaction {
		hash: string_to_h256("43a5d6d13b6a9dca381e3f4b4677a4b9e5d9f80d1a5b6cfa2b1404fab733bcee".to_string()).unwrap(),
		nonce: Default::default(),
		block_hash: None,
		block_number: None,
		transaction_index: None,
		from: H160::random(),
		to: Some(H160::random()),
		value: Default::default(),
		gas_price: Default::default(),
		gas: Default::default(),
		input: Bytes(hex::decode(serialized_str).unwrap()),
		raw: None,
	};

	let resp: Result<TransactionAndTransferType, ERC20Error> = transaction.clone().try_into();
	assert!(resp.is_ok());
	let resp = resp.unwrap();
	assert_eq!(transaction.from, resp.from());
}
