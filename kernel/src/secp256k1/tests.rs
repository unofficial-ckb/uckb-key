// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{kernel, SecretKey};

#[test]
fn test_random() {
    let sk0 = SecretKey::from_seed(1);
    let sk1 = SecretKey::from_seed(1);
    let sk2 = SecretKey::from_seed(2);
    assert_eq!(sk0, sk1);
    assert_ne!(sk0, sk2);
}

#[test]
fn test_zeroize() {
    let mut sk0 = SecretKey::from_seed(1);
    let sk1 = SecretKey::from_seed(1);
    assert_eq!(sk0, sk1);
    sk0.zeroize();
    assert_ne!(sk0, sk1);
    assert_eq!(sk0.len(), kernel::constants::SECRET_KEY_SIZE);
    for i in 0..sk0.len() {
        assert_eq!(sk0[i], 0);
    }
}
