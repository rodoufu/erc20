# erc20

<p align="center">
  <a href="https://travis-ci.com/rodoufu/erc20">
    <img src="https://travis-ci.com/rodoufu/erc20.svg?branch=master" alt="Current TravisCI build status.">
  </a>
  <a href="https://github.com/rodoufu/erc20/releases">
    <img src="https://badge.fury.io/gh/rodoufu%2Ferc20.svg" alt="Current version.">
  </a>
  <!--
  <a href='https://coveralls.io/github/rodoufu/erc20'>
    <img src='https://coveralls.io/repos/github/rodoufu/erc20/badge.svg' alt='Coverage Status' />
  </a>
  -->
  <a href="https://github.com/rodoufu/erc20">
      <img src="https://tokei.rs/b1/github/rodoufu/erc20?category=lines" alt="Current total lines.">
    </a>
  <a href="https://github.com/rodoufu/erc20/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License.">
  </a>
</p>


## Rust ERC20 parser

This project intends to parse the Ethereum transactions from web3 into an abstraction that makes generic dealing 
with Ethereum or ERC20 transfers.

Available at https://crates.io/crates/erc20

## Code examples

### Parsing a transfer

```rust
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
```

### Identifying an ERC20 contract address

```rust
let tusd_address = H160::from_str("0000000000085d4780B73119b644AE5ecd22b376").unwrap();
assert_eq!("0x0000000000085d4780b73119b644ae5ecd22b376".to_string(), format!("{:?}", tusd_address));

let contract_address: ContractAddress = tusd_address.into();
assert_eq!(ContractAddress::TUSD, contract_address);

// let tusd_from_contract: H160 = contract_address.into();
// assert_eq!(tusd_address, tusd_from_contract);
```

## References

- https://eips.ethereum.org/EIPS/eip-20
