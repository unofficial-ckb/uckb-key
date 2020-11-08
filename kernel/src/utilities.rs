// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! impl_std_traits {
    ($newtype:ident, $oldtype:ty) => {
        impl ::std::convert::AsRef<$oldtype> for $newtype {
            fn as_ref(&self) -> &$oldtype {
                &self.0
            }
        }

        impl ::std::convert::AsMut<$oldtype> for $newtype {
            fn as_mut(&mut self) -> &mut $oldtype {
                &mut self.0
            }
        }

        impl ::std::ops::Deref for $newtype {
            type Target = $oldtype;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $newtype {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[cfg(not(feature = "insecure"))]
macro_rules! impl_std_fmt_masked {
    ($type:ident) => {
        impl ::std::fmt::Debug for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}(_)", stringify!($type))
            }
        }

        impl ::std::fmt::Display for $type {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}(_)", stringify!($type))
            }
        }
    };
}

pub fn hex_string(bin: &[u8]) -> String {
    faster_hex::hex_string(&bin[..]).unwrap()
}
