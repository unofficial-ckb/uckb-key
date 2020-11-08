// Copyright (C) 2019-2020 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{fmt, str};

use bech32::{FromBase32, ToBase32};
use property::Property;

pub mod error;
use error::{Error, Result};

use crate::{blake2b, utilities};

#[cfg(test)]
mod tests;

pub const CODE_HASH_SIZE: usize = 32;
pub const BLAKE160_SIZE: usize = 20;
pub const SINCE_SIZE: usize = 8;

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
    Secp256k1MultiSig = 0x01,
}

/// Code Hash Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeHashType {
    Data = 0x02,
    Type = 0x04,
}

/// Code Hash
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeHash {
    Index(CodeHashIndex),
    Data {
        hash_type: CodeHashType,
        content: [u8; CODE_HASH_SIZE],
    },
}

/// Args
#[derive(Debug, Clone)]
pub enum Args {
    Simple(Vec<u8>),
    MultiSig {
        version: u8,
        first_n_required: u8,
        threshold: u8,
        contents: Vec<[u8; BLAKE160_SIZE]>,
        since: Option<[u8; 8]>,
    },
}

#[derive(Property, Clone)]
#[property(get(public), set(disable), mut(disable))]
pub struct Address {
    network: Network,
    code_hash: CodeHash,
    args: Args,
}

#[derive(Property, Default, Clone)]
#[property(get(disable), set(public, prefix = "", type = "own"), mut(disable))]
pub struct AddressBuilder {
    network: Network,
    code_hash: CodeHash,
    args: Args,
}

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

impl Default for Args {
    fn default() -> Self {
        Self::Simple(vec![0u8; BLAKE160_SIZE])
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

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            0x01 => Ok(Self::Secp256k1MultiSig),
            v => Err(Error::UnknownCodeHashIndex(v)),
        }
    }
}

impl CodeHashType {
    pub fn value(self) -> u8 {
        self as u8
    }
}

impl fmt::Display for CodeHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CodeHash::Index(index) => write!(f, "CodeHash::Index({:?})", index),
            CodeHash::Data {
                hash_type,
                ref content,
            } => write!(
                f,
                "CodeHash::Data {{ hash_type: {:?}, content: {} }}",
                hash_type,
                utilities::hex_string(&content[..])
            ),
        }
    }
}

impl Args {
    pub fn serialize_into(&self, buf: &mut Vec<u8>) {
        match self {
            Args::Simple(ref args) => {
                buf.extend_from_slice(&args[..]);
            }
            Args::MultiSig {
                version,
                first_n_required,
                threshold,
                contents,
                since,
            } => {
                let len = 4 + BLAKE160_SIZE * contents.len();
                let mut bin = Vec::with_capacity(len);
                bin.push(*version);
                bin.push(*first_n_required);
                bin.push(*threshold);
                bin.push(contents.len() as u8);
                for content in &contents[..] {
                    bin.extend_from_slice(&content[..]);
                }
                let hash = blake2b::blake160(&bin);
                buf.extend_from_slice(&hash[..]);
                if let Some(since) = since {
                    buf.extend_from_slice(&since[..]);
                }
            }
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Args {{")?;
        match self {
            Self::Simple(content) => {
                write!(f, " Simple({})", utilities::hex_string(&content[..]))?;
            }
            Self::MultiSig {
                version,
                first_n_required,
                threshold,
                contents,
                since,
            } => {
                write!(f, " MultiSig {{")?;
                write!(f, " version: {}", version)?;
                write!(f, " first_n: {}", first_n_required)?;
                write!(f, " threshold: {}", threshold)?;
                write!(f, " number: {}", contents.len())?;
                let mut first = true;
                write!(f, " contents: [")?;
                for content in &contents[..] {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", utilities::hex_string(&content[..]))?;
                }
                write!(f, "]")?;
                if let Some(since) = since {
                    write!(f, " since: {}", utilities::hex_string(&since[..]))?;
                }
                write!(f, " }}")?;
            }
        };
        write!(f, " }}")
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

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Address {{")?;
        write!(f, " network: {}", self.network)?;
        write!(f, " , code_hash: {}", self.code_hash)?;
        write!(f, " , args: {}", self.args)?;
        write!(f, " }}")
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hrp = self.network.value();
        let data: Vec<u8> = match self.code_hash {
            CodeHash::Index(index) => {
                let mut data = Vec::with_capacity(2 + BLAKE160_SIZE);
                data.push(PayloadFormat::Short.value());
                data.push(index.value());
                match index {
                    CodeHashIndex::Secp256k1Blake160 => match self.args {
                        Args::Simple(_) => {
                            self.args.serialize_into(&mut data);
                            Ok(data)
                        }
                        _ => Err(Error::Unreachable(
                            "unsupported args for Secp256k1Blake160".to_owned(),
                        )),
                    },
                    CodeHashIndex::Secp256k1MultiSig => match self.args {
                        Args::Simple(_) => {
                            self.args.serialize_into(&mut data);
                            Ok(data)
                        }
                        Args::MultiSig { since, .. } => {
                            if since.is_none() {
                                self.args.serialize_into(&mut data);
                                Ok(data)
                            } else {
                                Err(Error::Unreachable(
                                    "since should be None for Secp256k1MultiSig in Short Format"
                                        .to_owned(),
                                ))
                            }
                        }
                    },
                }
            }
            CodeHash::Data {
                hash_type,
                ref content,
            } => {
                let args_len = match self.args {
                    Args::Simple(ref args) => args.len(),
                    Args::MultiSig { since, .. } => {
                        BLAKE160_SIZE + since.map(|x| x.len()).unwrap_or(0)
                    }
                };
                let mut data = Vec::with_capacity(1 + CODE_HASH_SIZE + content.len() + args_len);
                data.push(PayloadFormat::Full(hash_type).value());
                data.extend_from_slice(&content[..]);
                self.args.serialize_into(&mut data);
                Ok(data)
            }
        }
        .unwrap();
        bech32::encode_to_fmt(f, hrp, data.to_base32()).unwrap()
    }
}

impl str::FromStr for Address {
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
                    let format = PayloadFormat::from_value(data[0])?;
                    offset += 1;
                    data = &bytes[offset..];
                    let (code_hash, args) = match format {
                        PayloadFormat::Short => {
                            if data.is_empty() {
                                return Err(Error::InvalidDataSince(offset));
                            }
                            let index = CodeHashIndex::from_value(data[0])?;
                            offset += 1;
                            data = &bytes[offset..];
                            let args = if data.len() == BLAKE160_SIZE {
                                Ok(Args::Simple(data.to_owned()))
                            } else {
                                Err(Error::ShortFormatArgs)
                            }?;
                            (CodeHash::Index(index), args)
                        }
                        PayloadFormat::Full(hash_type) => {
                            if data.len() < CODE_HASH_SIZE {
                                return Err(Error::InvalidDataSince(offset));
                            }
                            let mut content = [0u8; CODE_HASH_SIZE];
                            content.copy_from_slice(&data[..CODE_HASH_SIZE]);
                            offset += CODE_HASH_SIZE;
                            data = &bytes[offset..];
                            (
                                CodeHash::Data { hash_type, content },
                                Args::Simple(data.to_owned()),
                            )
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn code_hash_by_index(mut self, index: CodeHashIndex) -> Self {
        self.code_hash = CodeHash::Index(index);
        self
    }

    pub fn code_hash_by_data(
        mut self,
        hash_type: CodeHashType,
        content: [u8; CODE_HASH_SIZE],
    ) -> Self {
        self.code_hash = CodeHash::Data { hash_type, content };
        self
    }

    pub fn args_simple(mut self, args: Vec<u8>) -> Self {
        self.args = Args::Simple(args);
        self
    }

    pub fn args_multisig(
        mut self,
        version: u8,
        first_n_required: u8,
        threshold: u8,
        contents: Vec<[u8; BLAKE160_SIZE]>,
        since: Option<[u8; SINCE_SIZE]>,
    ) -> Self {
        self.args = Args::MultiSig {
            version,
            first_n_required,
            threshold,
            contents,
            since,
        };
        self
    }

    pub fn build(self) -> Result<Address> {
        let Self {
            network,
            code_hash,
            args,
        } = self;
        match code_hash {
            CodeHash::Index(index) => match index {
                CodeHashIndex::Secp256k1Blake160 => match args {
                    Args::Simple(ref content) => {
                        if content.len() == BLAKE160_SIZE {
                            Ok(())
                        } else {
                            Err(Error::ShortFormatArgs)
                        }
                    }
                    _ => Err(Error::Secp256k1Blake160Args),
                },
                CodeHashIndex::Secp256k1MultiSig => match args {
                    Args::Simple(ref content) => {
                        if content.len() == BLAKE160_SIZE {
                            Ok(())
                        } else {
                            Err(Error::ShortFormatArgs)
                        }
                    }
                    Args::MultiSig { .. } => Ok(()),
                },
            },
            CodeHash::Data { .. } => Ok(()),
        }?;
        if let Args::MultiSig {
            first_n_required,
            threshold,
            ref contents,
            ..
        } = args
        {
            if first_n_required > threshold || threshold > contents.len() as u8 {
                return Err(Error::MultiSigArgs);
            }
        };
        Ok(Address {
            network,
            code_hash,
            args,
        })
    }
}
