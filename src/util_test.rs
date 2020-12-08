use crate::util::{
	BytesToFixedNumber,
	FixedNumberToBytes,
};
use web3::types::{
	H160,
	U256,
};

#[test]
fn bytes_to_fixed_number() {
	let bytes_str = "a9059cbb00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
	let bytes_vec = hex::decode(bytes_str);
	assert!(bytes_vec.is_ok());
	let bytes_vec = bytes_vec.unwrap();
	let mut decoder: BytesToFixedNumber = bytes_vec.into();

	let method = decoder.next_vec(4);
	assert!(method.is_ok());
	let method = method.unwrap();
	assert_eq!(hex::decode("a9059cbb").unwrap(), method);

	let to_address = decoder.next_h160();
	assert!(to_address.is_ok());
	let to_address = to_address.unwrap();
	assert_eq!(H160::from_low_u64_be(1), to_address);

	let value = decoder.next_u256();
	assert!(value.is_ok());
	let value = value.unwrap();
	assert_eq!(U256::from(2), value);
}

#[test]
fn fixed_number_to_bytes() {
	let mut encoder: FixedNumberToBytes = Default::default();

	encoder.push_vec(&hex::decode("a9059cbb").unwrap());
	encoder.push_h160(&H160::from_low_u64_be(1));
	encoder.push_u256(&U256::from(2));

	let encoded_vec: Vec<u8> = encoder.into();

	let bytes_str = "a9059cbb00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";
	let bytes_vec = hex::decode(bytes_str);
	assert!(bytes_vec.is_ok());
	let bytes_vec = bytes_vec.unwrap();

	assert_eq!(bytes_vec, encoded_vec);
}
