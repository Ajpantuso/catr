// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use catr::options;
use catr::Command;
use clap::Parser;

fn main() {
    let opts = options::Options::parse();
    if let Err(e) = Command::from(&opts).run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
