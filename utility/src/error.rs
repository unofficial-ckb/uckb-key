// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use failure::Fail;

use uckb_key_kernel::{address, secp256k1};

#[derive(Debug, Fail)]
pub(crate) enum Error {
    #[fail(display = "hex error: {}", _0)]
    Hex(String),
    #[fail(display = "secp256k1 error: {}", _0)]
    Secp256k1(secp256k1::Error),
    #[fail(display = "address error: {}", _0)]
    Address(address::error::Error),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

macro_rules! convert_error {
    ($name:ident, $inner_error:ty) => {
        impl ::std::convert::From<$inner_error> for Error {
            fn from(error: $inner_error) -> Self {
                Self::$name(error)
            }
        }
    };
}

convert_error!(Secp256k1, secp256k1::Error);
convert_error!(Address, address::error::Error);
