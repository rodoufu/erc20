///! ERC20 specific information.

use crate::ERC20Error;
use maplit::hashmap;
use serde::{
	Deserialize,
	Serialize,
};
use std::{
	collections::HashMap,
	convert::{
		TryFrom,
		TryInto,
	},
	str::FromStr,
};
use web3::types::H160;

/// ERC20 method operation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ERC20Method {
	/// Returns the amount which `spender` is still allowed to withdraw from `owner`.
	Allowance,
	/// Allows `spender` to withdraw from your account multiple times, up to the `value` amount. If this function is called again it overwrites the current allowance with `value`.
	Approve,
	/// Returns the account balance of another account with address `owner`.
	BalanceOf,
	/// Returns the total token supply.
	TotalSupply,
	/// Transfers `value` amount of tokens to address `to`, and MUST fire the Transfer event. The function SHOULD throw if the message caller’s account balance does not have enough tokens to spend.
	Transfer,
	/// Transfers `value` amount of tokens from address `from` to address `to`, and MUST fire the Transfer event.
	TransferFrom,
	/// In case it is not identified an ERC20 operation.
	Unidentified,
}

impl TryFrom<ERC20Method> for [u8; 4] {
	type Error = ERC20Error;

	fn try_from(value: ERC20Method) -> Result<Self, Self::Error> {
		match value {
			ERC20Method::Allowance => Ok([0xdd, 0x62, 0xed, 0x3e]),
			ERC20Method::Approve => Ok([0x09, 0x5e, 0xa7, 0xb3]),
			ERC20Method::BalanceOf => Ok([0x70, 0xa0, 0x82, 0x31]),
			ERC20Method::TotalSupply => Ok([0x18, 0x16, 0x0d, 0xdd]),
			ERC20Method::Transfer => Ok([0xa9, 0x05, 0x9c, 0xbb]),
			ERC20Method::TransferFrom => Ok([0x23, 0xb8, 0x72, 0xdd]),
			ERC20Method::Unidentified => Err(ERC20Error::UnexpectedType),
		}
	}
}

impl TryFrom<ERC20Method> for Vec<u8> {
	type Error = ERC20Error;

	fn try_from(value: ERC20Method) -> Result<Self, Self::Error> {
		let resp: [u8; 4] = value.try_into()?;
		Ok(resp.to_vec())
	}
}

impl From<Vec<u8>> for ERC20Method {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		if data.is_empty() || data.len() < 4 {
			Self::Unidentified
		} else {
			let method_encoding: HashMap<Self, [u8; 4]> = hashmap! {
				Self::Allowance => Self::Allowance.try_into().unwrap(),
				Self::Approve => Self::Approve.try_into().unwrap(),
				Self::BalanceOf => Self::BalanceOf.try_into().unwrap(),
				Self::TotalSupply => Self::TotalSupply.try_into().unwrap(),
				Self::Transfer => Self::Transfer.try_into().unwrap(),
				Self::TransferFrom => Self::TransferFrom.try_into().unwrap(),
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

/// Known ERC20 contract addresses.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ContractAddress {
	/// Basic Attention Token
	BAT,
	/// Binance Token
	BNB,
	/// Binance USD
	BUSD,
	/// ChainLink
	LINK,
	/// Tether USD
	TUSD,
	/// USD Coin
	USDC,
	/// True USD
	USDT,
	/// Wrapped BTC
	WBTC,
	#[allow(non_camel_case_types)]
	/// Compound Dai
	cDAI,
	/// Crypto.com Coin
	CRO,
	/// Digital Asset Exchange
	OKB,
	/// Bitfinex LEO Token
	LEO,
	/// Wrapped Filecoin
	WFIL,
	/// VeChain
	VEN,
	/// Dai Stablecoin
	DAI,
	/// Uniswap
	UNI,
	/// Unidentified
	Unidentified(H160),
}

impl ContractAddress {
	fn contract_and_address() -> HashMap<ContractAddress, H160> {
		hashmap! {
			ContractAddress::TUSD => H160::from_str("0000000000085d4780B73119b644AE5ecd22b376").unwrap(),
			ContractAddress::LINK => H160::from_str("514910771af9ca656af840dff83e8264ecf986ca").unwrap(),
			ContractAddress::BNB => H160::from_str("B8c77482e45F1F44dE1745F52C74426C631bDD52").unwrap(),
			ContractAddress::USDC => H160::from_str("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48").unwrap(),
			ContractAddress::WBTC => H160::from_str("2260fac5e5542a773aa44fbcfedf7c193bc2c599").unwrap(),
			ContractAddress::cDAI => H160::from_str("5d3a536E4D6DbD6114cc1Ead35777bAB948E3643").unwrap(),
			ContractAddress::OKB => H160::from_str("75231f58b43240c9718dd58b4967c5114342a86c").unwrap(),
			ContractAddress::CRO => H160::from_str("a0b73e1ff0b80914ab6fe0444e65848c4c34450b").unwrap(),
			ContractAddress::WFIL => H160::from_str("6e1A19F235bE7ED8E3369eF73b196C07257494DE").unwrap(),
			ContractAddress::BAT => H160::from_str("0d8775f648430679a709e98d2b0cb6250d2887ef").unwrap(),
			ContractAddress::BUSD => H160::from_str("4fabb145d64652a948d72533023f6e7a623c7c53").unwrap(),
			ContractAddress::USDT => H160::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap(),
			ContractAddress::LEO => H160::from_str("2af5d2ad76741191d15dfe7bf6ac92d4bd912ca3").unwrap(),
			ContractAddress::VEN => H160::from_str("d850942ef8811f2a866692a623011bde52a462c1").unwrap(),
			ContractAddress::DAI => H160::from_str("6b175474e89094c44da98b954eedeac495271d0f").unwrap(),
			ContractAddress::UNI => H160::from_str("1f9840a85d5af5bf1d1762f925bdaddc4201f984").unwrap(),
		}
	}
}

impl From<H160> for ContractAddress {
	fn from(address: H160) -> Self {
		for (contract, address_v) in Self::contract_and_address() {
			if address_v == address {
				return contract;
			}
		}
		ContractAddress::Unidentified(address)
	}
}

impl From<ContractAddress> for H160 {
	fn from(contract_address: ContractAddress) -> Self {
		for (contract, address) in ContractAddress::contract_and_address() {
			if contract_address == contract {
				return address;
			}
		}
		match contract_address {
			ContractAddress::Unidentified(address) => address,
			_ => panic!("Unexpected contract {:?}", contract_address),
		}
	}
}

#[test]
fn creating_address() {
	let tusd_address = H160::from_str("0000000000085d4780B73119b644AE5ecd22b376").unwrap();
	assert_eq!("0x0000000000085d4780b73119b644ae5ecd22b376".to_string(), format!("{:?}", tusd_address));

	let contract_address: ContractAddress = tusd_address.into();
	assert_eq!(ContractAddress::TUSD, contract_address);

	let tusd_from_contract: H160 = contract_address.into();
	assert_eq!(tusd_address, tusd_from_contract);
}
