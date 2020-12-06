use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ERC20Error {
	NoTransferTransaction,
	UnexpectedSize,
}