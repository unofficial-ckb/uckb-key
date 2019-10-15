// Copyright (C) 2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod subcmd;

fn execute() -> error::Result<()> {
    let config = config::build_commandline()?;
    match config {
        config::AppConfig::Key(args) => subcmd::key::execute(args),
        config::AppConfig::Addr(args) => subcmd::addr::execute(args),
    };
    Ok(())
}

fn main() {
    if let Err(error) = execute() {
        eprintln!("Fatal: {}", error);
        ::std::process::exit(1);
    }
}
