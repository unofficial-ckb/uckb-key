// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "internal error: should be unreachable, {}", _0)]
    Unreachable(String),

    #[fail(display = "bech32 error: {}", _0)]
    Bech32(bech32::Error),

    #[fail(display = "unknown network: {}", _0)]
    UnknownNetwork(String),
    #[fail(display = "unknown payload format: {}", _0)]
    UnknownPayloadFormat(u8),
    #[fail(display = "unknown code hash index: {}", _0)]
    UnknownCodeHashIndex(u8),
    #[fail(display = "invalid data since offset {}", _0)]
    InvalidDataSince(usize),

    #[fail(display = "args error: Secp256k1Blake160")]
    Secp256k1Blake160Args,
    #[fail(display = "args error: ShortFormatArgs")]
    ShortFormatArgs,
    #[fail(display = "args error: MultiSig")]
    MultiSigArgs,
}

pub type Result<T> = ::std::result::Result<T, Error>;
