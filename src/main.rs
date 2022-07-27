#![cfg_attr(feature = "deny-warnings", deny(warnings))]
// warn on lints, that are included in `rust-lang/rust`s bootstrap
#![warn(rust_2018_idioms, unused_lifetimes)]

use std::env;
use std::path::PathBuf;
use std::process::{self, Command};

const CARGO_TYPESIZE_HELP: &str = r#"Lists type sizes for all types in a package.

Usage:
    cargo typesize [options] [--] [<opts>...]

Options:
    build                    Run with cargo build
    check                    Run with cargo check (default)
    test                     Run with cargo test
    -h, --help               Print this message
    -V, --version            Print version info and exit
"#;

fn show_help() {
    println!("{}", CARGO_TYPESIZE_HELP);
}

fn show_version() {
    println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn main() {
    if env::args().any(|a| a == "--help" || a == "-h") {
        show_help();
        return;
    }

    if env::args().any(|a| a == "--version" || a == "-V") {
        show_version();
        return;
    }

    if let Err(code) = process(env::args().skip(2)) {
        process::exit(code);
    }
}

struct TypeSizeCmd {
    cargo_subcommand: &'static str,
    args: Vec<String>,
    typesize_args: Vec<String>,
}

impl TypeSizeCmd {
    fn new(old_args: impl Iterator<Item = String>) -> Self {
        let mut cargo_subcommand = "check";
        let mut args = vec![];
        let typesize_args: Vec<String> = vec![];

        for arg in old_args {
            match arg.as_str() {
                "build" => {
                    cargo_subcommand = "build";
                    continue;
                }
                "check" => {
                    cargo_subcommand = "check --no-run";
                    continue;
                }
                "test" => {
                    cargo_subcommand = "test";
                    continue;
                }
                "--" => break,
                _ => {}
            }
            args.push(arg);
        }

        Self {
            cargo_subcommand,
            args,
            typesize_args,
        }
    }

    fn path() -> PathBuf {
        let mut path = env::current_exe()
            .expect("current executable path invalid")
            .with_file_name("typesize-driver");

        if cfg!(windows) {
            path.set_extension("exe");
        }

        path
    }

    fn into_std_cmd(self) -> Command {
        let mut cmd = Command::new("cargo");
        let typesize_args: String = self
            .typesize_args
            .iter()
            .map(|arg| format!("{}__TYPESIZE_HACKERY__", arg))
            .collect();

        cmd.env("RUSTC_WORKSPACE_WRAPPER", Self::path())
            .env("TYPESIZE_ARGS", typesize_args)
            .arg(self.cargo_subcommand)
            .args(&self.args);

        cmd
    }
}

fn process<I>(old_args: I) -> Result<(), i32>
where
    I: Iterator<Item = String>,
{
    let cmd = TypeSizeCmd::new(old_args);

    let mut cmd = cmd.into_std_cmd();

    let exit_status = cmd
        .spawn()
        .expect("could not run cargo")
        .wait()
        .expect("failed to wait for cargo?");

    if exit_status.success() {
        Ok(())
    } else {
        Err(exit_status.code().unwrap_or(-1))
    }
}
