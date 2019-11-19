// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{kernel, Error};
use crate::utilities;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(pub(super) kernel::recovery::RecoverableSignature);

impl_std_traits!(Signature, kernel::recovery::RecoverableSignature);

impl ::std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl ::std::fmt::Display for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let bytes = self.to_bytes();
        write!(f, "{}", utilities::hex_string(&bytes))
    }
}

impl Signature {
    pub fn from_bytes(bytes: &[u8; 65]) -> Result<Self, Error> {
        let id = kernel::recovery::RecoveryId::from_i32(i32::from(bytes[64]))?;
        kernel::recovery::RecoverableSignature::from_compact(&bytes[..64], id).map(Self)
    }

    pub fn to_bytes(&self) -> [u8; 65] {
        let (id, data) = self.serialize_compact();
        let mut bytes = [0; 65];
        bytes[0..64].copy_from_slice(&data[0..64]);
        bytes[64] = id.to_i32() as u8;
        bytes
    }
}
