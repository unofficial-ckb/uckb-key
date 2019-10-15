// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::address::{Address, AddressBuilder, CodeHashIndex};

pub enum PubKey {
    Secp256k1Blake160([u8; 20]),
}

impl ::std::fmt::Debug for PubKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Self::Secp256k1Blake160(_) => write!(f, "PubKey::Secp256k1Blake160({})", self),
        }
    }
}

impl ::std::fmt::Display for PubKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Self::Secp256k1Blake160(ref data) => {
                let s = faster_hex::hex_string(&data[..]).unwrap();
                write!(f, "{}", s)
            }
        }
    }
}

impl PubKey {
    pub fn from_secp256k1_blake160(data: [u8; 20]) -> Self {
        Self::Secp256k1Blake160(data)
    }

    pub fn address(&self) -> Address {
        match *self {
            Self::Secp256k1Blake160(ref data) => AddressBuilder::default()
                .code_hash_by_index(CodeHashIndex::Secp256k1Blake160)
                .args(vec![data.to_vec()])
                .build()
                .unwrap(),
        }
    }
}
