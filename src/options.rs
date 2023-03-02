// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = clap::crate_authors!())]
#[command(version = clap::crate_version!())]
#[command(about = format!("{} - concatenate files and pring on the standard output", clap::crate_name!()))]
pub struct Options {
    #[arg(default_value = "-")]
    pub file: Vec<String>,
    /// number all output lines
    #[arg(short = 'n', long = "number")]
    pub number_lines: bool,
    /// number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank")]
    pub number_non_blank_lines: bool,
    #[arg(short = 'E')]
    pub show_ends: bool,
    #[arg(short = 'T')]
    pub show_tabs: bool,
    /// suppress repeated empty output lines
    #[arg(short = 's', long = "squeeze-blank")]
    pub squeeze_blank: bool,
}
