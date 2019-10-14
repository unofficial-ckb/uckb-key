// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use lazy_static::lazy_static;

pub(self) use secp256k1_kernel as kernel;

lazy_static! {
    pub(self) static ref SECP256K1: kernel::Secp256k1<kernel::All> = kernel::Secp256k1::new();
}

mod public;
mod secret;

pub use kernel::Error;
pub use public::PublicKey;
pub use secret::SecretKey;

#[cfg(test)]
mod tests;
