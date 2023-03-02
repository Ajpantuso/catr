// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::fs;
use std::io::{self, BufRead, BufWriter, Write};
use std::path;

pub mod options;
mod prefixer;
mod squeezer;

pub struct Command<'a> {
    options: &'a options::Options,
    prefixer: prefixer::LinePrefixer,
    squeezer: squeezer::LineSqueezer,
}

impl<'a> From<&'a options::Options> for Command<'a> {
    fn from(options: &'a options::Options) -> Command {
        Command {
            options,
            prefixer: prefixer::LinePrefixer::from(options),
            squeezer: squeezer::LineSqueezer::new(),
        }
    }
}

impl<'a> Command<'a> {
    pub fn run(&self) -> Result<()> {
        let mut stdout = BufWriter::new(io::stdout());

        for line in self
            .options
            .file
            .iter()
            .flat_map(to_lines)
            .filter_map(|r| self.apply_squeeze_blank(r))
            .map(|r| self.apply_show_tabs(r))
            .map(|r| self.apply_show_ends(r))
            .filter_map(|r| self.apply_prefix(r))
        {
            writeln!(stdout, "{line}")?;
        }

        stdout.flush()?;

        Ok(())
    }
    fn apply_squeeze_blank(
        &self,
        result: Result<String, io::Error>,
    ) -> Option<Result<String, io::Error>> {
        match result {
            Ok(l) => {
                if self.options.squeeze_blank {
                    return Ok(self.squeezer.squeeze(&l)).transpose();
                }

                Some(Ok(l))
            }
            Err(e) => Some(Err(e)),
        }
    }
    fn apply_show_tabs(&self, result: Result<String, io::Error>) -> Result<String, io::Error> {
        result.map(|l| {
            if !self.options.show_tabs {
                return l;
            }

            l.replace('\t', "^I")
        })
    }
    fn apply_show_ends(&self, result: Result<String, io::Error>) -> Result<String, io::Error> {
        result.map(|l| {
            if !self.options.show_ends {
                return l;
            }

            format!("{}$", l)
        })
    }
    fn apply_prefix(&self, result: Result<String, io::Error>) -> Option<String> {
        match result {
            Ok(s) => Some(self.prefixer.prefix(&s)),
            Err(e) => Some(format!("{}: {}", clap::crate_name!(), e)),
        }
    }
}

fn to_lines(p: impl AsRef<path::Path>) -> Box<dyn Iterator<Item = Result<String, std::io::Error>>> {
    if p.as_ref() == path::Path::new("-") {
        return Box::new(io::BufReader::new(io::stdin()).lines());
    }

    match fs::File::open(&p) {
        Ok(f) => Box::new(io::BufReader::new(f).lines()),
        Err(e) => Box::new(std::iter::once(Err(io::Error::new(
            io::ErrorKind::Other,
            format!("{}: {}", p.as_ref().display(), e),
        )))),
    }
}
