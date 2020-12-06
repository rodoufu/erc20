use crate::ERC20Error;
use hex::FromHexError;
use web3::types::{
	Bytes,
	H160,
	H256,
	U256,
};

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

pub struct BytesToFixedNumber {
	data: Vec<u8>,
	index: usize,
}

impl From<Vec<u8>> for BytesToFixedNumber {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		Self {
			data,
			index: 0,
		}
	}
}

impl From<Bytes> for BytesToFixedNumber {
	#[inline]
	fn from(data: Bytes) -> Self {
		data.0.into()
	}
}

impl BytesToFixedNumber {
	pub fn next_vec(&mut self, size: usize) -> Result<Vec<u8>, ERC20Error> {
		if self.index + size > self.data.len() {
			return Err(ERC20Error::UnexpectedEndOfData);
		}
		let mut resp = Vec::new();
		for i in 0..size {
			resp.push(self.data[self.index + i]);
		}
		self.index += size;
		Ok(resp)
	}

	pub fn skip(&mut self, size: usize) -> Result<(), ERC20Error> {
		if self.index + size > self.data.len() {
			return Err(ERC20Error::UnexpectedEndOfData);
		}
		self.index += size;
		Ok(())
	}

	pub fn next_h160(&mut self) -> Result<H160, ERC20Error> {
		self.skip((256 - 160) / 8)?;
		self.next_h160_not_padded()
	}

	pub fn next_h160_not_padded(&mut self) -> Result<H160, ERC20Error> {
		let vec_resp = self.next_vec(20)?;
		let mut the_vec: [u8; 20] = [0; 20];
		for i in 0..20 {
			the_vec[i] = vec_resp[i];
		}
		Ok(the_vec.into())
	}

	pub fn next_h256(&mut self) -> Result<H256, ERC20Error> {
		let vec_resp = self.next_vec(32)?;
		let mut the_vec: [u8; 32] = [0; 32];
		for i in 0..32 {
			the_vec[i] = vec_resp[i];
		}
		Ok(the_vec.into())
	}

	pub fn next_u256(&mut self) -> Result<U256, ERC20Error> {
		let vec_resp = self.next_vec(32)?;
		let mut the_vec: [u8; 32] = [0; 32];
		for i in 0..32 {
			the_vec[i] = vec_resp[i];
		}
		Ok(the_vec.into())
	}
}

pub struct FixedNumberToBytes {
	data: Vec<u8>,
}

impl FixedNumberToBytes {
	pub fn push_vec(&mut self, vec: &Vec<u8>) {
		for it in vec {
			self.data.push(*it);
		}
	}

	pub fn push_h160(&mut self, value: &H160) {
		for i in 0..((256 - 160) / 8) {
			self.data.push(0);
		}
		self.push_h160_not_padded(value);
	}

	pub fn push_h160_not_padded(&mut self, value: &H160) {
		for it in value.0.iter() {
			self.data.push(*it);
		}
	}

	pub fn push_h256(&mut self, value: &H256) {
		for it in value.0.iter() {
			self.data.push(*it);
		}
	}

	pub fn push_u256(&mut self, value: &U256) {
		for i in 0..(256 / 8) {
			self.data.push(value.byte(i));
		}
	}
}

impl From<Bytes> for FixedNumberToBytes {
	fn from(data: Bytes) -> Self {
		Self {
			data: data.0
		}
	}
}