use crate::erc20::ContractAddress;
use std::str::FromStr;
use web3::types::H160;

#[test]
fn creating_address() {
	let tusd_address = H160::from_str("0000000000085d4780B73119b644AE5ecd22b376").unwrap();
	assert_eq!("0x0000000000085d4780b73119b644ae5ecd22b376".to_string(), format!("{:?}", tusd_address));

	let contract_address: ContractAddress = tusd_address.into();
	assert_eq!(ContractAddress::TUSD, contract_address);

	let tusd_from_contract: H160 = contract_address.into();
	assert_eq!(tusd_address, tusd_from_contract);
}

#[test]
fn usdc_address() {
	let usdc_address: H160 = ContractAddress::USDC.into();
	assert_eq!("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", format!("{:?}", usdc_address));
}
