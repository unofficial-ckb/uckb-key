// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

use kernel::{address, secp256k1};

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("arguments error: {0}")]
    Args(String),
    #[error("hex error: {0}")]
    Hex(String),
    #[error("secp256k1 error: {0}")]
    Secp256k1(#[from] secp256k1::Error),
    #[error("address error: {0}")]
    Address(#[from] address::error::Error),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;
