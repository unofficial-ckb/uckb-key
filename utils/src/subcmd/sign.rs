// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kernel::SignAlgo;

use crate::{config::SignArgs, error::Result};

pub(crate) fn execute(args: SignArgs) -> Result<()> {
    let signature = match args.algo() {
        SignAlgo::Secp256k1(secret) => secret
            .sign_recoverable(args.input())
            .map(|sign| sign.to_bytes())
            .map(|bytes| bytes.to_vec())?,
    };
    println!(
        "signature = {}",
        faster_hex::hex_string(&signature[..]).unwrap()
    );
    Ok(())
}
