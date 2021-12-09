use std::env;
use std::path::{Path, PathBuf};

use bindgen;

mod defaults {
  pub const SOLANA: &str = "../../solana";
  pub const PC: &str = "../../pyth-client";
  pub const WRAPPER: &str = "wrapper.h";
}

fn env_or(key: &str, default: &str) -> String {
  println!("cargo:rerun-if-env-changed={}", key);
  env::var(key).unwrap_or(default.to_string())
}

macro_rules! getenv {
  ($key: ident) => {
    env_or(stringify!($key), defaults::$key)
  };
}

fn output_dir() -> PathBuf {
  PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn bpf_dir() -> PathBuf {
  PathBuf::from(getenv!(SOLANA)).join("sdk/bpf")
}

fn solana_inc_dir() -> PathBuf {
  bpf_dir().join("c/inc")
}

fn llvm_inc_dir() -> PathBuf {
  bpf_dir().join("dependencies/bpf-tools/llvm/lib/clang/12.0.1/include")
}

fn pc_src_dir() -> PathBuf {
  PathBuf::from(getenv!(PC)).join("program/src")
}

fn oracle_header() -> PathBuf {
  pc_src_dir().join("oracle/oracle.h")
}

fn inc_arg(dir: &Path) -> String {
  String::from("-I") + dir.to_str().unwrap()
}

fn sys_inc_arg(dir: &Path) -> String {
  String::from("-isystem") + dir.to_str().unwrap()
}

fn build_c_bindings(header: &Path) {
  // Rebuild whenever this header file changes:
  println!("cargo:rerun-if-changed={}", header.to_str().unwrap());
  let src_dir = header.parent().unwrap().parent().unwrap();
  let rs_path = output_dir()
    .join(header.file_stem().unwrap())
    .with_extension("rs");
  let builder = bindgen::Builder::default()
    .header(header.to_str().unwrap())
    .header(getenv!(WRAPPER))
    .clang_args(["-std=c17", "-D__bpf__"])
    .clang_arg(sys_inc_arg(&solana_inc_dir()))
    .clang_arg(sys_inc_arg(&llvm_inc_dir()))
    .clang_arg(inc_arg(src_dir))
    // Rebuild whenever an included header file changes:
    .parse_callbacks(Box::new(bindgen::CargoCallbacks));
  let bindings = builder.generate()
    .expect("Unable to generate bindings");
  bindings.write_to_file(rs_path).expect("Couldn't write bindings!")
}

fn main() {
  build_c_bindings(oracle_header().as_path());
}
