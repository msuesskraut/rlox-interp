use derive_builder::Builder;
use std::process::{Command, Output};
use std::str::from_utf8;

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Builder)]
pub struct LitTester {
    #[builder(setter(into))]
    cmd: String,
    #[builder(default = "vec![]")]
    args: Vec<String>,
    #[builder(default = "vec![]")]
    checks: Vec<String>,
}

impl LitTesterBuilder {
    pub fn test(&self) {
        self.build().expect("LitTest incomplete").test()
    }
}

pub type LitTest = LitTesterBuilder;

type LitError = Vec<String>;

fn apply_check<'a>(check_str: &str, output: &'a str) -> Result<&'a str, String> {
    if let Some(pos) = output.find(check_str) {
        Ok(output.split_at(pos + check_str.len()).1)
    } else {
        Err(format!("failed to find check string: {}", check_str))
    }
}

fn rec_check_output(checks: &[String], stdout: &str, errs: &mut Vec<String>) {
    if let Some((first_check, rest_checks)) = checks.split_first() {
        match apply_check(first_check, stdout) {
            Ok(rest) => rec_check_output(rest_checks, rest, errs),
            Err(msg) => {
                errs.push(msg);
                rec_check_output(rest_checks, stdout, errs);
            }
        }
    }
    // else: end recursion
}

impl LitTester {
    fn run_command(&self) -> Result<Output, LitError> {
        let res = Command::new(&self.cmd)
            .current_dir(CRATE_PATH)
            .args(&self.args)
            .output();

        match res {
            Ok(output) => {
                if output.status.success() {
                    Ok(output)
                } else {
                    Err(vec![format!(
                        "Command failed with exit code: {:?}",
                        output.status.code()
                    )])
                }
            }
            Err(err) => Err(vec![format!("OS Error running command: {:?}", err)]),
        }
    }

    fn check_output(&self, output: Output) -> Result<(), LitError> {
        match from_utf8(&output.stdout) {
            Ok(s) => self.check_output_stdout(s),
            Err(err) => Err(vec![format!("Utf8 decoding stdout failed with {:?}", err)]),
        }
    }

    fn check_output_stdout(&self, stdout: &str) -> Result<(), LitError> {
        let mut errs = Vec::new();
        rec_check_output(&self.checks, stdout, &mut errs);

        if !errs.is_empty() {
            Err(errs)
        } else {
            Ok(())
        }
    }

    fn run_test(&self) -> Result<(), LitError> {
        self.check_output(self.run_command()?)
    }

    pub fn test(&self) {
        match self.run_test() {
            Ok(()) => {}
            Err(msgs) => {
                eprintln!("Lit test failed with:");
                for msg in msgs {
                    eprintln!("  {}", msg);
                }
                panic!("lit test failed");
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_command_success() -> Result<(), LitError> {
        let lt = LitTester {
            cmd: "cargo".into(),
            args: vec!["help".into()],
            checks: Vec::new(),
        };

        match lt.run_command() {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    #[test]
    fn run_command_failed_no_success() -> Result<(), Output> {
        let lt = LitTester {
            cmd: "cargo".into(),
            args: vec!["holp".into()],
            checks: Vec::new(),
        };

        match lt.run_command() {
            Ok(ok) => Err(ok),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn run_command_failed_os_error() -> Result<(), Output> {
        let lt = LitTester {
            cmd: "command_does_not_exist".into(),
            args: vec![],
            checks: Vec::new(),
        };

        match lt.run_command() {
            Ok(ok) => Err(ok),
            Err(_) => Ok(()),
        }
    }

    fn dummy_exit_status() -> std::process::ExitStatus {
        Command::new("cargo")
            .arg("help")
            .output()
            .expect("command for dummy ExitStatus failed")
            .status
    }

    fn dummy_lit_test() -> LitTester {
        LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec![],
        }
    }

    #[test]
    fn decode_stdout_valid_utf8() {
        let output = Output {
            status: dummy_exit_status(),
            stdout: "Hello".as_bytes().to_vec(),
            stderr: vec![],
        };

        assert_eq!(Ok(()), dummy_lit_test().check_output(output));
    }

    #[test]
    fn decode_stdout_invalid_utf8() {
        let output = Output {
            status: dummy_exit_status(),
            stdout: vec![255u8],
            stderr: vec![],
        };

        dummy_lit_test()
            .check_output(output)
            .expect_err("unexpected decode without error");
    }

    #[test]
    fn run_test_success() -> Result<(), LitError> {
        LitTester {
            cmd: "cargo".into(),
            args: vec!["help".into()],
            checks: vec![],
        }
        .run_test()
    }

    #[test]
    fn run_test_failed() {
        LitTester {
            cmd: "cargo".into(),
            args: vec!["holp".into()],
            checks: vec![],
        }
        .run_test()
        .expect_err("expected LitTest to fail");
    }

    #[test]
    fn test_success() {
        LitTester {
            cmd: "cargo".into(),
            args: vec!["help".into()],
            checks: vec![],
        }
        .test();
    }

    #[should_panic]
    #[test]
    fn test_fails() {
        LitTester {
            cmd: "cargo".into(),
            args: vec!["holp".into()],
            checks: vec![],
        }
        .test();
    }

    #[test]
    fn apply_check_at_start() {
        let output = "Hello world";
        let check_str = "Hello";

        assert_eq!(
            " world",
            apply_check(check_str, output).expect("expect to find substring")
        );
    }

    #[test]
    fn apply_check_mid() {
        let output = "Hello world";
        let check_str = "lo";

        assert_eq!(
            " world",
            apply_check(check_str, output).expect("expect to find substring")
        );
    }

    #[test]
    fn apply_check_err() {
        let output = "Hello world";
        let check_str = "the";

        assert_eq!(
            "failed to find check string: the",
            apply_check(check_str, output).expect_err("expect apply_check to fail")
        );
    }

    #[test]
    fn check_output_empty_checks() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec![],
        };

        assert_eq!(Ok(()), lt.check_output_stdout(""));
    }

    #[test]
    fn check_output_first_check_fails() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["the".into()],
        };

        assert_eq!(
            Err(vec!["failed to find check string: the".into()]),
            lt.check_output_stdout("hello world")
        );
    }

    #[test]
    fn check_output_one_check() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["lo".into()],
        };

        assert_eq!(Ok(()), lt.check_output_stdout("hello world"));
    }

    #[test]
    fn check_output_two_checks() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["lo".into(), "orl".into()],
        };

        assert_eq!(Ok(()), lt.check_output_stdout("hello world"));
    }

    #[test]
    fn check_output_two_checks_adjecent() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["lo".into(), " wo".into()],
        };

        assert_eq!(Ok(()), lt.check_output_stdout("hello world"));
    }

    #[test]
    fn check_output_two_checks_wrong_order() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["wo".into(), "lo".into()],
        };

        assert_eq!(
            Err(vec!["failed to find check string: lo".into()]),
            lt.check_output_stdout("hello world")
        );
    }

    #[test]
    fn check_output_two_checks_only_first_fails() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["al".into(), "lo".into()],
        };

        assert_eq!(
            Err(vec!["failed to find check string: al".into()]),
            lt.check_output_stdout("hello world")
        );
    }

    #[test]
    fn check_output_three_checks_middle_fails() {
        let lt = LitTester {
            cmd: "".into(),
            args: vec![],
            checks: vec!["el".into(), "el".into(), "lo".into()],
        };

        assert_eq!(
            Err(vec!["failed to find check string: el".into()]),
            lt.check_output_stdout("hello world")
        );
    }

    #[test]
    fn builder_pattern() {
        let lt = LitTest::default()
            .cmd("cargo")
            .build()
            .expect("LitTestBuilder failed");
        assert_eq!("cargo", lt.cmd);
        assert!(lt.args.is_empty());
        assert!(lt.checks.is_empty());
    }
}
