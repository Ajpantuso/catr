// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

use std::cell;

use crate::options;

pub struct LinePrefixer {
    count: cell::Cell<i32>,
    number_lines: bool,
    number_non_blank_lines: bool,
}

impl LinePrefixer {
    pub fn prefix(&self, line: &str) -> String {
        let mut result = String::from(line);

        if self.number_non_blank_lines && line.is_empty() {
            return result;
        }

        if self.number_lines || self.number_non_blank_lines {
            result = format!("{:6}\t{result}", self.count.get());
        }

        self.count.set(self.count.get() + 1);

        result
    }
}

impl<'a> From<&'a options::Options> for LinePrefixer {
    fn from(opts: &'a options::Options) -> Self {
        LinePrefixer {
            count: cell::Cell::new(1),
            number_lines: opts.number_lines,
            number_non_blank_lines: opts.number_non_blank_lines,
        }
    }
}

#[cfg(test)]
mod lineprefixer_test {
    use super::*;
    use test_case::test_case;

    #[test_case(false, false, &["abc"], "abc" ; "no options/non-blank")]
    #[test_case(false, false, &["abc", "def"], "abc\ndef" ; "no options/multi-line")]
    #[test_case(false, false, &["abc", "", "def"], "abc\n\ndef" ; "no options/multi-line-blanks")]
    #[test_case(false, false, &[""], "" ; "no options/blank")]
    #[test_case(true, false, &["abc"], "     1\tabc" ; "number_lines/non-blank")]
    #[test_case(true, false, &["abc", "def"], "     1\tabc\n     2\tdef" ; "number_lines/multi-line")]
    #[test_case(true, false, &["abc", "", "def"], "     1\tabc\n     2\t\n     3\tdef" ; "number_lines/multi-line-blanks")]
    #[test_case(true, false, &[""], "     1\t" ; "number_lines/blank")]
    #[test_case(false, true, &["abc"], "     1\tabc" ; "number_non_blank_lines/non-blank")]
    #[test_case(false, true, &["abc", "def"], "     1\tabc\n     2\tdef" ; "number_non_blank_lines/multi-line")]
    #[test_case(false, true, &["abc", "", "def"], "     1\tabc\n\n     2\tdef" ; "number_non_blank_lines/multi-line-blanks")]
    #[test_case(false, true, &[""], "" ; "number_non_blank_lines/blank")]
    #[test_case(true, true, &["abc"], "     1\tabc" ; "number_lines+number_non_blank_lines/non-blank")]
    #[test_case(true, true, &["abc", "def"], "     1\tabc\n     2\tdef" ; "number_lines+number_non_blank_lines/multi-line")]
    #[test_case(true, true, &["abc", "", "def"], "     1\tabc\n\n     2\tdef" ; "number_lines+number_non_blank_lines/multi-line-blanks")]
    #[test_case(true, true, &[""], "" ; "number_lines+number_non_blank_lines/blank")]
    fn prefix(number_lines: bool, number_non_blank_lines: bool, lines: &[&str], expected: &str) {
        let prefixer = LinePrefixer {
            count: cell::Cell::new(1),
            number_lines,
            number_non_blank_lines,
        };

        assert_eq!(
            expected,
            lines
                .iter()
                .map(|l| prefixer.prefix(l))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}
