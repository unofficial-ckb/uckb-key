// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kernel::{blake2b, HashAlgo};

use crate::config::HashArgs;

pub(crate) fn execute(args: HashArgs) {
    let output = match args.algo() {
        HashAlgo::Blake2b256 => blake2b::blake2b_256(args.input()),
    };
    println!("output = {}", faster_hex::hex_string(&output[..]).unwrap());
}
