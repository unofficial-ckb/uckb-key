// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use blake2b_rs::{Blake2b, Blake2bBuilder};

pub const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";

pub fn blake2b_256<T: AsRef<[u8]>>(s: T) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut blake2b = Blake2bBuilder::new(32)
        .personal(CKB_HASH_PERSONALIZATION)
        .build();
    blake2b.update(s.as_ref());
    blake2b.finalize(&mut result);
    result
}

pub fn blake160<T: AsRef<[u8]>>(s: T) -> [u8; 20] {
    let mut result = [0u8; 20];
    let hash = blake2b_256(s);
    result.copy_from_slice(&hash[..20]);
    result
}
