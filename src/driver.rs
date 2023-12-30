#![feature(rustc_private)]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]
// warn on lints, that are included in `rust-lang/rust`s bootstrap
#![warn(rust_2018_idioms, unused_lifetimes)]
// warn on rustc internal lints
#![warn(rustc::internal)]

// FIXME: switch to something more ergonomic here, once available.
// (Currently there is no way to opt into sysroot crates without `extern crate`.)
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_target;

use rustc_driver::Compilation;
use rustc_hir::Item;
use rustc_interface::{interface, Queries};
use rustc_middle::ty::Ty;
use rustc_session::config::ErrorOutputType;
use rustc_session::EarlyDiagCtxt;
use rustc_target::abi;

use std::collections::BTreeMap;
use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

/// If a command-line option matches `find_arg`, then apply the predicate `pred` on its value. If
/// true, then return it. The parameter is assumed to be either `--arg=value` or `--arg value`.
fn arg_value<'a, T: Deref<Target = str>>(
    args: &'a [T],
    find_arg: &str,
    pred: impl Fn(&str) -> bool,
) -> Option<&'a str> {
    let mut args = args.iter().map(Deref::deref);
    while let Some(arg) = args.next() {
        let mut arg = arg.splitn(2, '=');
        if arg.next() != Some(find_arg) {
            continue;
        }

        match arg.next().or_else(|| args.next()) {
            Some(v) if pred(v) => return Some(v),
            _ => {}
        }
    }
    None
}

#[test]
fn test_arg_value() {
    let args = &["--bar=bar", "--foobar", "123", "--foo"];

    assert_eq!(arg_value(&[] as &[&str], "--foobar", |_| true), None);
    assert_eq!(arg_value(args, "--bar", |_| false), None);
    assert_eq!(arg_value(args, "--bar", |_| true), Some("bar"));
    assert_eq!(arg_value(args, "--bar", |p| p == "bar"), Some("bar"));
    assert_eq!(arg_value(args, "--bar", |p| p == "foo"), None);
    assert_eq!(arg_value(args, "--foobar", |p| p == "foo"), None);
    assert_eq!(arg_value(args, "--foobar", |p| p == "123"), Some("123"));
    assert_eq!(
        arg_value(args, "--foobar", |p| p.contains("12")),
        Some("123")
    );
    assert_eq!(arg_value(args, "--foo", |_| true), None);
}

struct DefaultCallbacks;
impl rustc_driver::Callbacks for DefaultCallbacks {}

#[derive(Default)]
struct TypeSize {
    sizes: BTreeMap<String, u64>,
}

impl TypeSize {
    fn add_type(&mut self, ty: Ty<'_>, item: &Item<'_>, layout: abi::Layout<'_>) {
        self.sizes
            .insert(format!("{ty:?} - {:?}", item.span), layout.size().bytes());
    }

    fn print_typesizes(&self) {
        let mut sorted = Vec::from_iter(self.sizes.iter());
        sorted.sort_by_key(|(name, bytes)| (*bytes, name.clone()));

        if let Ok(bin) = env::var("CARGO_BIN_NAME") {
            println!("Inspecting layout of bin: {bin}");
        } else if let Ok(lib) = env::var("CARGO_PKG_NAME") {
            println!("Inspecting layout of lib: {lib}");
        }

        let max_bytes_len = if let Some((_, largest)) = sorted.last() {
            format!("{largest}").len()
        } else {
            0
        };

        for (name, bytes) in sorted {
            let bytes_str = format!("{bytes}");
            let pad =
                String::from_iter(std::iter::repeat(" ").take(max_bytes_len - bytes_str.len()));
            println!("{pad}{bytes_str}\t{name}");
        }
    }
}

impl rustc_driver::Callbacks for TypeSize {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &interface::Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        if !self.sizes.is_empty() {
            panic!("Already computed sizes");
        }
        // Analyze the program and inspect the types of definitions.
        queries.global_ctxt().unwrap().enter(|tcx| {
            for id in tcx.hir().items() {
                let hir = tcx.hir();
                let item = hir.item(id);

                use rustc_hir::ItemKind;
                match item.kind {
                    ItemKind::GlobalAsm(..)
                    | ItemKind::Static(..)
                    | ItemKind::Const(..)
                    | ItemKind::TyAlias(..)
                    | ItemKind::OpaqueTy(..)
                    | ItemKind::Enum(..)
                    | ItemKind::Struct(..)
                    | ItemKind::Union(..) => (),
                    _ => continue,
                }

                let ty = tcx.type_of(item.owner_id.def_id);
                let ty = ty.instantiate_identity();
                let param_env = tcx.param_env(item.owner_id.def_id);
                match tcx.layout_of(param_env.and(ty)) {
                    Ok(layout) => {
                        self.add_type(ty, item, layout.layout);
                    }
                    Err(_) => continue,
                }
            }
        });

        Compilation::Stop
    }
}

fn display_help() {
    println!(
        "\
Prints the size of every type in a Rust crate.

Usage:
    cargo typesize [options] [--] [<opts>...]

Common options:
    -h, --help               Print this message
        --rustc              Pass all args to rustc
"
    );
}

fn toolchain_path(home: Option<String>, toolchain: Option<String>) -> Option<PathBuf> {
    home.and_then(|home| {
        toolchain.map(|toolchain| {
            let mut path = PathBuf::from(home);
            path.push("toolchains");
            path.push(toolchain);
            path
        })
    })
}

fn read_sys_root(sys_root_arg: &Option<&str>) -> String {
    // Get the sysroot, looking from most specific to this invocation to the least:
    // - command line
    // - runtime environment
    //    - SYSROOT
    //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
    // - sysroot from rustc in the path
    // - compile-time environment
    //    - SYSROOT
    //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN

    sys_root_arg
        .map(PathBuf::from)
        .or_else(|| std::env::var("SYSROOT").ok().map(PathBuf::from))
        .or_else(|| {
            let home = std::env::var("RUSTUP_HOME")
                .or_else(|_| std::env::var("MULTIRUST_HOME"))
                .ok();
            let toolchain = std::env::var("RUSTUP_TOOLCHAIN")
                .or_else(|_| std::env::var("MULTIRUST_TOOLCHAIN"))
                .ok();
            toolchain_path(home, toolchain)
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("--print")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME")
                .or(option_env!("MULTIRUST_HOME"))
                .map(ToString::to_string);
            let toolchain = option_env!("RUSTUP_TOOLCHAIN")
                .or(option_env!("MULTIRUST_TOOLCHAIN"))
                .map(ToString::to_string);
            toolchain_path(home, toolchain)
        })
        .map(|pb| pb.to_string_lossy().to_string())
        .expect(
            "need to specify SYSROOT env var during clippy compilation, or use rustup or multirust",
        )
}

pub fn main() {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&early_dcx);
    exit(rustc_driver::catch_with_exit_code(move || {
        let mut orig_args: Vec<String> = env::args().collect();
        let sys_root_arg = arg_value(&orig_args, "--sysroot", |_| true);
        let have_sys_root_arg = sys_root_arg.is_some();

        let sys_root = read_sys_root(&sys_root_arg);

        // make "typesize-driver --rustc" work like a subcommand that passes further args to "rustc"
        // for example `typesize-driver --rustc --version` will print the rustc version that typesize-driver
        // uses
        if let Some(pos) = orig_args.iter().position(|arg| arg == "--rustc") {
            orig_args.remove(pos);
            orig_args[0] = "rustc".to_string();

            // if we call "rustc", we need to pass --sysroot here as well
            let mut args: Vec<String> = orig_args.clone();
            if !have_sys_root_arg {
                args.extend(vec!["--sysroot".into(), sys_root]);
            };

            return rustc_driver::RunCompiler::new(&args, &mut DefaultCallbacks).run();
        }
        // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
        // We're invoking the compiler programmatically, so we ignore this/
        let wrapper_mode =
            orig_args.get(1).map(Path::new).and_then(Path::file_stem) == Some("rustc".as_ref());

        if wrapper_mode {
            // we still want to be able to invoke it normally though
            orig_args.remove(1);
        }

        if !wrapper_mode
            && (orig_args.iter().any(|a| a == "--help" || a == "-h") || orig_args.len() == 1)
        {
            display_help();
            exit(0);
        }

        // this conditional check for the --sysroot flag is there so users can call
        // `typesize_driver` directly
        // without having to pass --sysroot or anything
        let mut args: Vec<String> = orig_args.clone();
        if !have_sys_root_arg {
            args.extend(vec!["--sysroot".into(), sys_root]);
        };

        if env::var("CARGO_PRIMARY_PACKAGE").is_ok() {
            let mut typesize = TypeSize::default();
            rustc_driver::RunCompiler::new(&args, &mut typesize).run()?;

            typesize.print_typesizes();
        }

        // We always run the compiler again in the default mode after analysis.
        //
        // This is done to support cases where we are analyzing multiple interdependent
        // crates at once (e.g. in a workspace).
        //
        // Crates earlier in the compilation pipeline need to produce build artifacts
        // for dependent crates to compile correctly.
        //
        // There's probably a way to achieve this with a single compiler pass...
        rustc_driver::RunCompiler::new(&args, &mut DefaultCallbacks).run()
    }))
}
