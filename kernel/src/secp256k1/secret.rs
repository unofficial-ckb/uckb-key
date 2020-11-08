// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{ops::Drop, ptr, sync::atomic};

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

use super::{kernel, Error, PublicKey, Signature, SECP256K1};

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct SecretKey(pub(super) kernel::SecretKey);

impl_std_traits!(SecretKey, kernel::SecretKey);

#[cfg(feature = "insecure")]
impl ::std::fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(feature = "insecure")]
impl ::std::fmt::Display for SecretKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(not(feature = "insecure"))]
impl_std_fmt_masked!(SecretKey);

impl SecretKey {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self(kernel::SecretKey::new(rng))
    }

    pub fn from_slice(data: &[u8]) -> Result<Self, Error> {
        kernel::SecretKey::from_slice(data).map(Self)
    }

    pub fn from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        Self::new(&mut rng)
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self::new(&mut rng)
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_secret_key(self)
    }

    pub fn sign_recoverable(&self, input: &[u8]) -> Result<Signature, Error> {
        kernel::Message::from_slice(input)
            .map(|msg| (&*SECP256K1).sign_recoverable(&msg, self))
            .map(Signature)
    }

    pub(crate) fn zeroize(&mut self) {
        let Self(inner) = self;
        let dst = inner.as_mut_ptr();
        let len = inner.len();
        for of in 0..len {
            unsafe {
                ptr::write_volatile(dst.add(of), 0);
            }
            atomic::compiler_fence(atomic::Ordering::SeqCst);
        }
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.zeroize()
    }
}
