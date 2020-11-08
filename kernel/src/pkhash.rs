// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    address::{Address, AddressBuilder, CodeHashIndex, Network},
    utilities,
};

pub enum PubKeyHash {
    Secp256k1Blake160([u8; 20]),
}

impl ::std::fmt::Debug for PubKeyHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Self::Secp256k1Blake160(_) => write!(f, "PubKeyHash::Secp256k1Blake160({})", self),
        }
    }
}

impl ::std::fmt::Display for PubKeyHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Self::Secp256k1Blake160(ref data) => write!(f, "{}", utilities::hex_string(data)),
        }
    }
}

impl PubKeyHash {
    pub fn from_secp256k1_blake160(data: [u8; 20]) -> Self {
        Self::Secp256k1Blake160(data)
    }

    pub fn address(&self, is_mainnet: bool) -> Address {
        let network = if is_mainnet {
            Network::Main
        } else {
            Network::Test
        };
        match *self {
            Self::Secp256k1Blake160(ref data) => AddressBuilder::default()
                .network(network)
                .code_hash_by_index(CodeHashIndex::Secp256k1Blake160)
                .args_simple(data.to_vec())
                .build()
                .unwrap(),
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match *self {
            Self::Secp256k1Blake160(ref data) => &data[..],
        }
    }
}
