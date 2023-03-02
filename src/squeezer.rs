// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use std::cell;

pub struct LineSqueezer {
    previous_blank: cell::Cell<bool>,
}

impl LineSqueezer {
    pub fn new() -> Self {
        LineSqueezer {
            previous_blank: cell::Cell::new(false),
        }
    }
    pub fn squeeze(&self, line: &str) -> Option<String> {
        if self.previous_blank.get() && line.is_empty() {
            return None;
        }

        self.previous_blank.set(line.is_empty());

        Some(line.to_string())
    }
}
