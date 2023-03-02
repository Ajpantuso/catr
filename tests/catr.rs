// SPDX-FileCopyrightText: 2023 Andrew Pantuso <ajpantuso@gmail.com>
//
// SPDX-License-Identifier: Apache-2.0

#[cfg(test)]
mod catr {
    use anyhow::Result;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use std::path;
    use test_case::test_case;

    #[test_case("", &["no_blanks.txt"], load_output("no_blanks.txt") ; "no blank input/no options")]
    #[test_case("", &["-n", "no_blanks.txt"], load_output("no_blanks_numbered.txt") ; "no blank input/number lines")]
    #[test_case("", &["-b", "no_blanks.txt"], load_output("no_blanks_numbered.txt") ; "no blank input/number non blank lines")]
    #[test_case("", &["-s", "no_blanks.txt"], load_output("no_blanks.txt") ; "no blank input/squeeze")]
    #[test_case("", &["-sn", "no_blanks.txt"], load_output("no_blanks_numbered.txt") ; "no blank input/squeeze+number lines")]
    #[test_case("", &["-sb", "no_blanks.txt"], load_output("no_blanks_numbered.txt") ; "no blank input/squeeze+number non blank lines")]
    #[test_case("", &["-nb", "no_blanks.txt"], load_output("no_blanks_numbered.txt") ; "no blank input/number lines+number non blank lines")]
    #[test_case("", &["-snb", "no_blanks.txt"], load_output("no_blanks_numbered.txt") ; "no blank input/squeeze+number lines+number non blank lines")]
    #[test_case("", &["blanks.txt"], load_output("blanks.txt") ; "blank input/no options")]
    #[test_case("", &["-n", "blanks.txt"], load_output("blanks_numbered.txt") ; "blank input/number lines")]
    #[test_case("", &["-b", "blanks.txt"], load_output("blanks_number_non_blank.txt") ; "blank input/number non blank lines")]
    #[test_case("", &["-s", "blanks.txt"], load_output("blanks_squeeze.txt") ; "blank input/squeeze")]
    #[test_case("", &["-sn", "blanks.txt"], load_output("blanks_squeeze_numbered.txt") ; "blank input/squeeze+number lines")]
    #[test_case("", &["-sb", "blanks.txt"], load_output("blanks_squeeze_number_non_blanks.txt") ; "blank input/squeeze+number non blank lines")]
    #[test_case("", &["-nb", "blanks.txt"], load_output("blanks_number_non_blank.txt") ; "blank input/number lines+number non blank lines")]
    #[test_case("", &["-snb", "blanks.txt"], load_output("blanks_squeeze_number_non_blanks.txt") ; "blank input/squeeze+number lines+number non blank lines")]
    #[test_case("", &["no_blanks.txt", "blanks.txt"], load_output("concat.txt") ; "concat/no options")]
    #[test_case("", &["-n", "no_blanks.txt", "blanks.txt"], load_output("concat_numbered.txt") ; "concat/number lines")]
    #[test_case("", &["-b", "no_blanks.txt", "blanks.txt"], load_output("concat_number_non_blank.txt") ; "concat/number non blank lines")]
    #[test_case("", &["-s", "no_blanks.txt", "blanks.txt"], load_output("concat_squeeze.txt") ; "concat/squeeze")]
    #[test_case("", &["-sn", "no_blanks.txt", "blanks.txt"], load_output("concat_squeeze_numbered.txt") ; "concat/squeeze+number lines")]
    #[test_case("", &["-sb", "no_blanks.txt", "blanks.txt"], load_output("concat_squeeze_number_non_blank.txt") ; "concat/squeeze+number non blank lines")]
    #[test_case("", &["-nb", "no_blanks.txt", "blanks.txt"], load_output("concat_number_non_blank.txt") ; "concat/number lines+number non blank lines")]
    #[test_case("", &["-snb", "no_blanks.txt", "blanks.txt"], load_output("concat_squeeze_number_non_blank.txt") ; "concat/squeeze+number lines+number non blank lines")]
    #[test_case("abcd\nefgh\n\nijkl\n", &[], load_output("stdin.txt") ; "stdin only/no options")]
    #[test_case("abcd\nefgh\n\nijkl\n", &["-"], load_output("stdin.txt") ; "stdin only/explicit '-'")]
    #[test_case("abcd\nefgh\n\nijkl\n", &["-n"], load_output("stdin_numbered.txt") ; "stdin only/number lines")]
    #[test_case("abcd\nefgh\n\nijkl\n", &["-b"], load_output("stdin_number_non_blank.txt") ; "stdin only/number non blank lines")]
    #[test_case("abcd\nefgh\n\nijkl\n", &["-bn"], load_output("stdin_number_non_blank.txt") ; "stdin only/number lines+number non blank lines")]
    fn valid(stdin: &str, args: &[&str], output: String) -> Result<()> {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .write_stdin(stdin)
            .args(args)
            .current_dir(root().join("inputs"))
            .assert()
            .stdout(predicate::str::contains(output))
            .success();

        Ok(())
    }
    fn load_output(name: &str) -> String {
        fs::read_to_string(root().join("outputs").join(name)).unwrap()
    }
    fn root<'a>() -> &'a path::Path {
        path::Path::new("tests")
    }
}
