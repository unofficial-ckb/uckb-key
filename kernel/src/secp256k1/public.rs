// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{kernel, Error, SecretKey, SECP256K1};
use crate::{blake2b, utilities, PubKeyHash};

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct PublicKey(pub(super) kernel::PublicKey);

impl_std_traits!(PublicKey, kernel::PublicKey);

impl ::std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ::std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let data = self.0.serialize();
        write!(f, "{}", utilities::hex_string(&data))
    }
}

impl PublicKey {
    pub fn from_secret_key(sk: &SecretKey) -> Self {
        Self(kernel::PublicKey::from_secret_key(&*SECP256K1, sk.as_ref()))
    }

    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        kernel::PublicKey::from_slice(data).map(Self)
    }

    pub fn pkhash_blake160(&self) -> PubKeyHash {
        PubKeyHash::from_secp256k1_blake160(blake2b::blake160(&self.0.serialize()[..]))
    }
}
