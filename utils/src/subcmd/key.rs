// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use kernel::{HashAlgo, SignAlgo};

use crate::{config::KeyArgs, error::Result};

pub(crate) fn execute(args: KeyArgs) -> Result<()> {
    match args.sign_algo() {
        SignAlgo::Secp256k1(sk) => {
            let pk = sk.public_key();
            match args.hash_algo() {
                HashAlgo::Blake2b256 => {
                    let pkh = pk.pkhash_blake160();
                    let addrm = pkh.address(true);
                    let addrt = pkh.address(false);
                    println!("Secp256k1 + Blake160:\n");
                    println!("    secret  = {}", sk);
                    println!("    public  = {}", pk);
                    println!("    pk-hash = {}", pkh);
                    println!("    mainnet = {}", addrm);
                    println!("    testnet = {}", addrt);
                }
            }
        }
    };
    println!(
        "\nNOTICE:\n\n    \
         This utility is very simple, it just prints the secret key to the screen.\n\n    \
         This brings a potential security risk:\n\n        \
         *** This secret key perhaps has been LEAKED ***\n\n        \
         (for example, someone saw it, or there is a camera behind you)\n\n    \
         How to use the secret key depends on yourself. \
         The author (ME) do NOT bear any responsibility.\n"
    );
    Ok(())
}
