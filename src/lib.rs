extern crate serde;
extern crate hex;

mod util;
mod transaction;
#[cfg(test)]
mod transaction_test;


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
