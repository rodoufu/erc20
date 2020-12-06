use web3::types::{
	H160,
	H256,
	U256,
};
use hex::FromHexError;

macro_rules! string_to {
	($name:ident, $num_type:ty, $size:expr) => {
		pub fn $name(text: String) -> Result<$num_type, FromHexError> {
			let value_vec = hex::decode(text)?;
			if value_vec.len() != $size {
				return Err(FromHexError::InvalidStringLength);
			}
			let mut value_array: [u8; $size] = Default::default();
			for i in 0..$size {
				value_array[i] = value_vec[i];
			}

			Ok(<$num_type>::from(value_array))
		}
	};
}

string_to!(string_to_h160, H160, 20);
string_to!(string_to_h256, H256, 32);

string_to!(string_to_u256, U256, 32);
