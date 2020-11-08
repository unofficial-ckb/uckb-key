// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("internal error: should be unreachable, {0}")]
    Unreachable(String),

    #[error("bech32 error: {0}")]
    Bech32(bech32::Error),

    #[error("unknown network: {0}")]
    UnknownNetwork(String),
    #[error("unknown payload format: {0}")]
    UnknownPayloadFormat(u8),
    #[error("unknown code hash index: {0}")]
    UnknownCodeHashIndex(u8),
    #[error("invalid data since offset {0}")]
    InvalidDataSince(usize),

    #[error("args error: Secp256k1Blake160")]
    Secp256k1Blake160Args,
    #[error("args error: ShortFormatArgs")]
    ShortFormatArgs,
    #[error("args error: MultiSig")]
    MultiSigArgs,
}

pub type Result<T> = ::std::result::Result<T, Error>;
