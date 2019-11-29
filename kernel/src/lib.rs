// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod utilities;

mod pkhash;
pub use pkhash::PubKeyHash;

pub mod address;

pub mod blake2b;
pub mod secp256k1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgo {
    Blake2b256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignAlgo {
    Secp256k1(secp256k1::SecretKey),
}
