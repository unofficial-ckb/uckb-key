// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use uckb_key_kernel::secp256k1;

use crate::config::KeyArgs;

pub(crate) fn execute(args: KeyArgs) {
    let sk = args
        .secret()
        .cloned()
        .unwrap_or_else(secp256k1::SecretKey::random);
    let pk = sk.public_key();
    let key = pk.pubkey_blake160();
    println!("Secp256k1 + Blake160:\n");
    println!("    secret  = {}", sk);
    println!("    public  = {}", pk);
    println!("    key     = {}", key);
    println!(
        "\nNOTICE:\n\n    \
         This utility is very simple, it just prints the secret key to the screen.\n\n    \
         This brings a potential security risk:\n\n        \
         *** This secret key perhaps has been LEAKED ***\n\n        \
         (for example, someone saw it, or there is a camera behind you)\n\n    \
         How to use the secret key depends on yourself. \
         The author (ME) do NOT bear any responsibility.\n"
    );
}
