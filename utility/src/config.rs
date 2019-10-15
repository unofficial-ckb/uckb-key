// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::convert::TryFrom;

use property::Property;

use ckb_key_kernel::{address, secp256k1};

use crate::error::{Error, Result};

pub(crate) enum AppConfig {
    Key(KeyArgs),
    Addr(AddrArgs),
}

#[derive(Property)]
pub(crate) struct KeyArgs {
    secret: Option<secp256k1::SecretKey>,
}

#[derive(Property)]
pub(crate) struct AddrArgs {
    address: address::Address,
}

pub(crate) fn build_commandline() -> Result<AppConfig> {
    let yaml = clap::load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    AppConfig::try_from(&matches)
}

impl<'a> TryFrom<&'a clap::ArgMatches<'a>> for AppConfig {
    type Error = Error;
    fn try_from(matches: &'a clap::ArgMatches) -> Result<Self> {
        match matches.subcommand() {
            ("key", Some(matches)) => KeyArgs::try_from(matches).map(AppConfig::Key),
            ("addr", Some(matches)) => AddrArgs::try_from(matches).map(AppConfig::Addr),
            _ => unreachable!(),
        }
    }
}

impl<'a> TryFrom<&'a clap::ArgMatches<'a>> for KeyArgs {
    type Error = Error;
    fn try_from(matches: &'a clap::ArgMatches) -> Result<Self> {
        let secret = matches
            .value_of("secret")
            .map(|hex_str| decode_hex(&hex_str))
            .transpose()?
            .map(|data| secp256k1::SecretKey::from_slice(&data[..]))
            .transpose()?;
        Ok(Self { secret })
    }
}

impl<'a> TryFrom<&'a clap::ArgMatches<'a>> for AddrArgs {
    type Error = Error;
    fn try_from(matches: &'a clap::ArgMatches) -> Result<Self> {
        let network = matches
            .value_of("network")
            .map(|value| match value {
                "mainnet" => address::Network::Main,
                "testnet" => address::Network::Test,
                _ => unreachable!(),
            })
            .unwrap_or_else(|| unreachable!());
        let code_hash = if let Some(value) = matches.value_of("code-hash-index") {
            let index = match value {
                "secp256k1-blake160" => address::CodeHashIndex::Secp256k1Blake160,
                _ => unreachable!(),
            };
            address::CodeHash::Index(index)
        } else {
            let hash_type = matches
                .value_of("code-hash-type")
                .map(|value| match value {
                    "data" => address::CodeHashType::Data,
                    "type" => address::CodeHashType::Type,
                    _ => unreachable!(),
                })
                .unwrap_or_else(|| unreachable!());
            matches
                .value_of("code-hash")
                .map(|data| decode_hex(data))
                .transpose()?
                .map(|content| address::CodeHash::Data { hash_type, content })
                .unwrap_or_else(|| unreachable!())
        };
        let args = matches
            .values_of("address-args")
            .map(|values| values.collect())
            .map(|args: Vec<&str>| {
                args.iter()
                    .map(|arg| decode_hex(arg))
                    .collect::<Result<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_else(Vec::new);
        let address = address::AddressBuilder::new(Vec::new())
            .network(network)
            .code_hash(code_hash)
            .args(args)
            .build()?;
        Ok(Self { address })
    }
}

fn decode_hex(hex_str: &str) -> Result<Vec<u8>> {
    let hex_bytes = hex_str.as_bytes();
    if &hex_bytes[0..2] != b"0x" || hex_str.len() % 2 != 0 || hex_bytes.len() % 2 != 0 {
        return Err(Error::Hex("the format of input is not right".to_owned()));
    }
    let mut decoded = vec![0; (hex_bytes.len() >> 1) - 1];
    faster_hex::hex_decode(&hex_bytes[2..], &mut decoded)
        .map_err(|err| Error::Hex(err.to_string()))
        .map(|_| decoded)
}
