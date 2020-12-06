use maplit::hashmap;
use serde::{
	Deserialize,
	Serialize,
};
use std::collections::HashMap;
use web3::types::H160;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ContractAddress {
	/// Basic Attention Token
	BAT(H160),
	/// Binance Token
	BNB(H160),
	/// Binance USD
	BUSD(H160),
	/// ChainLink
	LINK(H160),
	/// Tether USD
	TUSD(H160),
	/// USD Coin
	USDC(H160),
	/// True USD
	USDT(H160),
	/// Wrapped BTC
	WBTC(H160),
	/// Compound Dai
	cDAI(H160),
	/// Crypto.com Coin
	CRO(H160),
	/// Digital Asset Exchange
	OKB(H160),
	/// Bitfinex LEO Token
	LEO(H160),
	/// Wrapped Filecoin
	WFIL(H160),
	/// VeChain
	VEN(H160),
	/// Dai Stablecoin
	DAI(H160),
	/// Uniswap
	UNI(H160),
	/// Unidentified
	Unidentified(H160),
}
