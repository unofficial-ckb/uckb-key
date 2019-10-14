// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bech32::ToBase32;
use property::Property;

/// Nervos Network
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NervosNetwork {
    Main,
    Test,
}

/// Payload Format Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadFormat {
    Short,
    Full(CodeHashType),
}

/// Code Hash Index
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeHashIndex {
    Secp256k1Blake160 = 0x00,
}

/// Code Hash Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeHashType {
    Data = 0x02,
    Type = 0x04,
}

/// Code Hash Index
#[derive(Debug, Clone)]
pub enum CodeHash {
    Index(CodeHashIndex),
    Data {
        hash_type: CodeHashType,
        content: [u8; 32],
    },
}

#[derive(Clone)]
pub struct Address {
    network: NervosNetwork,
    code_hash: CodeHash,
    args: Vec<Vec<u8>>,
}

#[derive(Default, Property)]
#[property(get(disable), set(public, prefix = "", type = "own"), mut(disable))]
pub struct AddressBuilder {
    network: NervosNetwork,
    #[property(set(disable))]
    code_hash: CodeHash,
    args: Vec<Vec<u8>>,
}

impl Default for NervosNetwork {
    fn default() -> Self {
        NervosNetwork::Main
    }
}

impl Default for CodeHashIndex {
    fn default() -> Self {
        CodeHashIndex::Secp256k1Blake160
    }
}

impl Default for CodeHash {
    fn default() -> Self {
        Self::Index(Default::default())
    }
}

impl NervosNetwork {
    pub fn value(self) -> &'static str {
        match self {
            Self::Main => "ckb",
            Self::Test => "ckt",
        }
    }
}

impl ::std::fmt::Display for NervosNetwork {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let s = match self {
            Self::Main => "mainnet",
            Self::Test => "testnet",
        };
        write!(f, "{}", s)
    }
}

impl PayloadFormat {
    pub fn value(self) -> u8 {
        if let Self::Full(t) = self {
            t.value()
        } else {
            0x01 // Short
        }
    }
}

impl CodeHashIndex {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl CodeHashType {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl ::std::fmt::Display for CodeHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            CodeHash::Index(index) => write!(f, "CodeHash::Index({:?})", index),
            CodeHash::Data {
                hash_type,
                ref content,
            } => {
                let s = faster_hex::hex_string(&content[..]).unwrap();
                write!(
                    f,
                    "CodeHash::Data {{ hash_type: {:?}, content: {} }}",
                    hash_type, s
                )
            }
        }
    }
}

impl Address {
    pub fn into_builder(self) -> AddressBuilder {
        let Self {
            network,
            code_hash,
            args,
        } = self;
        AddressBuilder {
            network,
            code_hash,
            args,
        }
    }
}

impl ::std::fmt::Debug for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Address {{")?;
        write!(f, " network: {}", self.network)?;
        write!(f, " , code_hash: {}", self.code_hash)?;
        write!(f, " , args: [")?;
        let mut first = true;
        for arg in &self.args[..] {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            let s = faster_hex::hex_string(&arg[..]).unwrap();
            write!(f, "{}", s)?;
        }
        write!(f, "]")?;
        write!(f, " }}")
    }
}

impl ::std::fmt::Display for Address {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let hrp = self.network.value();
        let data: Vec<u8> = match self.code_hash {
            CodeHash::Index(index) => {
                let arg0 = &self.args[0];
                let mut data = Vec::with_capacity(2 + arg0.len());
                data.push(PayloadFormat::Short.value());
                data.push(index.value());
                data.extend_from_slice(&arg0[..]);
                data
            }
            CodeHash::Data {
                hash_type,
                ref content,
            } => {
                let args_len = self.args.len() + self.args.iter().map(Vec::len).sum::<usize>();
                let mut data = Vec::with_capacity(1 + 32 + args_len);
                data.push(PayloadFormat::Full(hash_type).value());
                data.extend_from_slice(&content[..]);
                for arg in self.args.iter() {
                    data.push(arg.len() as u8);
                    data.extend_from_slice(&arg[..]);
                }
                data
            }
        };
        bech32::encode_to_fmt(f, hrp, data.to_base32()).unwrap()
    }
}

impl AddressBuilder {
    pub fn new(args: Vec<Vec<u8>>) -> Self {
        Self::default().args(args)
    }

    pub fn code_hash_by_index(mut self, index: CodeHashIndex) -> Self {
        self.code_hash = CodeHash::Index(index);
        self
    }

    pub fn code_hash_by_data(mut self, hash_type: CodeHashType, content: [u8; 32]) -> Self {
        self.code_hash = CodeHash::Data { hash_type, content };
        self
    }

    pub fn build(self) -> Result<Address, ()> {
        let Self {
            network,
            code_hash,
            args,
        } = self;
        if let CodeHash::Index(_) = code_hash {
            if args.len() != 1 {
                return Err(());
            }
        }
        Ok(Address {
            network,
            code_hash,
            args,
        })
    }
}

#[cfg(test)]
mod tests {
    use slices::u8_slice;

    #[test]
    fn test_short() {
        let pk = u8_slice!("0x13e41d6f9292555916f17b4882a5477c01270142");
        let expected = "ckb1qyqp8eqad7ffy42ezmchkjyz54rhcqf8q9pqrn323p";
        let addr = super::AddressBuilder::default()
            .args(vec![pk.to_vec()])
            .build()
            .unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_full() {
        let code_hash =
            u8_slice!("0x48a2ce278d84e1102b67d01ac8a23b31a81cc54e922e3db3ec94d2ec4356c67c");
        let arg0 = u8_slice!("0xdde7801c073dfb3464c7b1f05b806bb2bbb84e99");
        let arg1 = u8_slice!("0x00c1ddf9c135061b7635ca51e735fc2b03cee339");
        let expected = "ckb1qfy29n383kzwzyptvlgp4j9z8vc6s8x9f6fzu0dnaj2d9mzr2mr8c9xau7qpcpealv6xf3a37pdcq6ajhwuyaxg5qrqam7wpx5rpka34efg7wd0u9vpuaceeu5fsh5";
        let addr = super::AddressBuilder::default()
            .code_hash_by_data(super::CodeHashType::Data, *code_hash)
            .args(vec![arg0.to_vec(), arg1.to_vec()])
            .build()
            .unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
    }
}
