// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bech32::{FromBase32, ToBase32};
use failure::Fail;
use property::Property;

pub const CODE_HASH_SIZE: usize = 32;

/// CKB Network
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
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
        content: Vec<u8>,
    },
}

#[derive(Clone, Property)]
#[property(get(public), set(disable), mut(disable))]
pub struct Address {
    network: Network,
    code_hash: CodeHash,
    args: Vec<Vec<u8>>,
}

#[derive(Default, Property)]
#[property(get(disable), set(public, prefix = "", type = "own"), mut(disable))]
pub struct AddressBuilder {
    network: Network,
    code_hash: CodeHash,
    args: Vec<Vec<u8>>,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "there should be only one but found {} arguments", number)]
    NotSingleArg { number: usize },
    #[fail(display = "the size(={}) of hash is not match", length)]
    HashSize { length: usize },
    #[fail(
        display = "the length(={}) of the No.{} argument is overflow",
        length, index
    )]
    ArgOverflow { index: usize, length: usize },
    #[fail(display = "bech32 error: {}", _0)]
    Bech32(bech32::Error),
    #[fail(display = "unknown network: {}", _0)]
    UnknownNetwork(String),
    #[fail(display = "unknown payload format: {}", _0)]
    UnknownPayloadFormat(u8),
    #[fail(display = "unknown code hash index: {}", _0)]
    UnknownCodeHashIndex(u8),
    #[fail(display = "invalid data since offset {}", _0)]
    InvalidDataSince(usize),
}

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

impl Default for Network {
    fn default() -> Self {
        Network::Main
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

impl Network {
    pub fn value(self) -> &'static str {
        match self {
            Self::Main => "ckb",
            Self::Test => "ckt",
        }
    }

    pub fn from_value(value: &str) -> Result<Self> {
        match value {
            "ckb" => Ok(Self::Main),
            "ckt" => Ok(Self::Test),
            v => Err(Error::UnknownNetwork(v.to_owned())),
        }
    }
}

impl ::std::fmt::Display for Network {
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

    pub fn from_value(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(Self::Short),
            0x02 => Ok(Self::Full(CodeHashType::Data)),
            0x04 => Ok(Self::Full(CodeHashType::Type)),
            v => Err(Error::UnknownPayloadFormat(v)),
        }
    }
}

impl CodeHashIndex {
    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn from_value(value: u8) -> Result<Self> {
        match value {
            0x00 => Ok(Self::Secp256k1Blake160),
            v => Err(Error::UnknownCodeHashIndex(v)),
        }
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
                let mut data = Vec::with_capacity(1 + CODE_HASH_SIZE + args_len);
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

impl ::std::str::FromStr for Address {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        bech32::decode(s)
            .map_err(Error::Bech32)
            .and_then(|(ref hrp, ref base32)| {
                let network = Network::from_value(&hrp)?;
                let bytes = Vec::<u8>::from_base32(&base32).map_err(Error::Bech32)?;
                let mut offset = 0;
                let mut data = &bytes[offset..];
                if data.is_empty() {
                    Err(Error::InvalidDataSince(offset))
                } else {
                    let mut args = Vec::new();
                    let format = PayloadFormat::from_value(data[0])?;
                    offset += 1;
                    data = &bytes[offset..];
                    let code_hash = match format {
                        PayloadFormat::Short => {
                            if data.is_empty() {
                                return Err(Error::InvalidDataSince(offset));
                            }
                            let index = CodeHashIndex::from_value(data[0])?;
                            offset += 1;
                            data = &bytes[offset..];
                            let arg = data.to_owned();
                            args.push(arg);
                            CodeHash::Index(index)
                        }
                        PayloadFormat::Full(hash_type) => {
                            if data.len() < CODE_HASH_SIZE {
                                return Err(Error::InvalidDataSince(offset));
                            }
                            let content = (&data[..CODE_HASH_SIZE]).to_owned();
                            offset += CODE_HASH_SIZE;
                            data = &bytes[offset..];
                            while !data.is_empty() {
                                let size = data[0] as usize;
                                offset += 1;
                                data = &bytes[offset..];
                                if data.len() < size {
                                    return Err(Error::InvalidDataSince(offset));
                                }
                                let arg = (&data[..size]).to_owned();
                                args.push(arg);
                                offset += size;
                                data = &bytes[offset..];
                            }
                            CodeHash::Data { hash_type, content }
                        }
                    };
                    AddressBuilder::default()
                        .network(network)
                        .code_hash(code_hash)
                        .args(args)
                        .build()
                }
            })
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

    pub fn code_hash_by_data(mut self, hash_type: CodeHashType, content: Vec<u8>) -> Self {
        self.code_hash = CodeHash::Data { hash_type, content };
        self
    }

    pub fn build(self) -> Result<Address> {
        let Self {
            network,
            code_hash,
            args,
        } = self;
        match code_hash {
            CodeHash::Index(_) => {
                let number = args.len();
                if number != 1 {
                    return Err(Error::NotSingleArg { number });
                }
            }
            CodeHash::Data { ref content, .. } => {
                let length = content.len();
                if length != CODE_HASH_SIZE {
                    return Err(Error::HashSize { length });
                }
                for (index, arg) in args.iter().enumerate() {
                    let length = arg.len();
                    if length > 255 {
                        return Err(Error::ArgOverflow { index, length });
                    }
                }
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
    use std::str::FromStr;

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
        let addr = super::Address::from_str(&actual).unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_full() {
        let code_hash =
            u8_slice!("0x48a2ce278d84e1102b67d01ac8a23b31a81cc54e922e3db3ec94d2ec4356c67c");
        let arg0 = u8_slice!("0xdde7801c073dfb3464c7b1f05b806bb2bbb84e99");
        let arg1 = u8_slice!("0x00c1ddf9c135061b7635ca51e735fc2b03cee339");
        let expected = "ckb1\
                        qfy29n383kzwzyptvlgp4j9z8vc6s8x9f6fzu0dnaj2d9mzr2mr8c9xau7qpcpe\
                        alv6xf3a37pdcq6ajhwuyaxg5qrqam7wpx5rpka34efg7wd0u9vpuaceeu5fsh5";
        let addr = super::AddressBuilder::default()
            .code_hash_by_data(super::CodeHashType::Data, (&code_hash[..]).to_owned())
            .args(vec![arg0.to_vec(), arg1.to_vec()])
            .build()
            .unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
        let addr = super::Address::from_str(&actual).unwrap();
        let actual = addr.to_string();
        assert_eq!(expected, actual);
    }
}
