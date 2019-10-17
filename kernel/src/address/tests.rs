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
fn test_short() {
    let pk = u8_slice!("0x13e41d6f9292555916f17b4882a5477c01270142");
    let expected = "ckb1qyqp8eqad7ffy42ezmchkjyz54rhcqf8q9pqrn323p";
    let addr = super::AddressBuilder::default()
        .args(vec![pk.to_vec()])
        .build()
        .unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
    let addr = super::Address::from_str(&actual).unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
}

#[test]
fn test_full() {
    let code_hash = u8_slice!("0x48a2ce278d84e1102b67d01ac8a23b31a81cc54e922e3db3ec94d2ec4356c67c");
    let arg0 = u8_slice!("0xdde7801c073dfb3464c7b1f05b806bb2bbb84e99");
    let arg1 = u8_slice!("0x00c1ddf9c135061b7635ca51e735fc2b03cee339");
    let expected = "ckb1\
                    qfy29n383kzwzyptvlgp4j9z8vc6s8x9f6fzu0dnaj2d9mzr2mr8c9xau7qpcpe\
                    alv6xf3a37pdcq6ajhwuyaxg5qrqam7wpx5rpka34efg7wd0u9vpuaceeu5fsh5";
    let addr = super::AddressBuilder::default()
        .code_hash_by_data(super::CodeHashType::Data, (&code_hash[..]).to_owned())
        .args(vec![arg0.to_vec(), arg1.to_vec()])
        .build()
        .unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
    let addr = super::Address::from_str(&actual).unwrap();
    let actual = addr.to_string();
    assert_eq!(expected, actual);
}
