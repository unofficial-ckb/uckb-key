// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use slices::u8_slice;
use std::str::FromStr;

#[test]
fn test_short_for_secp256k1_blake160() {
    let test = |pk: &[u8], expected: &str| {
        let addr = super::AddressBuilder::default()
            .network(super::Network::Main)
            .code_hash_by_index(super::CodeHashIndex::Secp256k1Blake160)
            .args_simple(pk.to_vec())
            .build()
            .unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
        let addr = super::Address::from_str(&actual).unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
    };
    let pk = u8_slice!("0x13e41d6f9292555916f17b4882a5477c01270142");
    let expected = "ckb1qyqp8eqad7ffy42ezmchkjyz54rhcqf8q9pqrn323p";
    test(pk, expected);
    let pk = u8_slice!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64");
    let expected = "ckb1qyqt8xaupvm8837nv3gtc9x0ekkj64vud3jqfwyw5v";
    test(pk, expected);
}

#[test]
fn test_short_for_secp256k1_multisig() {
    let pkh1 = u8_slice!("0xbd07d9f32bce34d27152a6a0391d324f79aab854");
    let pkh2 = u8_slice!("0x094ee28566dff02a012a66505822a2fd67d668fb");
    let pkh3 = u8_slice!("0x4643c241e59e81b7876527ebff23dfb24cf16482");
    let expected = "ckb1qyq5lv479ewscx3ms620sv34pgeuz6zagaaqklhtgg";
    let addr = super::AddressBuilder::default()
        .network(super::Network::Main)
        .code_hash_by_index(super::CodeHashIndex::Secp256k1MultiSig)
        .args_multisig(0, 1, 2, vec![*pkh1, *pkh2, *pkh3], None)
        .build()
        .unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
    let addr = super::Address::from_str(&actual).unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_full_for_simple() {
    let code_hash = u8_slice!("0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8");
    let args = u8_slice!("0xb39bbc0b3673c7d36450bc14cfcdad2d559c6c64");
    let expected = "ckb1\
                    qjda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xw3vumhs9nvu7\
                    86dj9p0q5elx66t24n3kxgj53qks";
    let addr = super::AddressBuilder::default()
        .network(super::Network::Main)
        .code_hash_by_data(super::CodeHashType::Type, *code_hash)
        .args_simple(args.to_vec())
        .build()
        .unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
    let addr = super::Address::from_str(&actual).unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_full_for_multisig() {
    let code_hash = u8_slice!("0x5c5069eb0857efc65e1bca0c07df34c31663b3622fd3876c876320fc9634e2a8");
    let args = u8_slice!("0x4146af6d67742cca87a9b0d1d3eb070e7a544e1c");
    let since = u8_slice!("0x5605008403080720");
    let expected = "ckb1\
                    q3w9q60tppt7l3j7r09qcp7lxnp3vcanvgha8pmvsa3jplykxn32s8ulyn4d0l8\
                    ztr8fzy4hzjmnn0uufyknj4s9qzzqxzq8yqd5ry6k";
    let addr = super::AddressBuilder::default()
        .network(super::Network::Main)
        .code_hash_by_data(super::CodeHashType::Type, *code_hash)
        .args_multisig(0, 0, 1, vec![*args], Some(*since))
        .build()
        .unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
    let addr = super::Address::from_str(&actual).unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
}
