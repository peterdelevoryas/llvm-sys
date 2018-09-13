use bindgen;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn link_lib(name: &str) {
	println!("cargo:rustc-link-lib={}", name);
}

fn link_search(path: &str) {
	println!("cargo:rustc-link-search={}", path);
}

fn output(c: &mut Command) -> String {
	let output = match c.stderr(Stdio::inherit()).output() {
		Ok(s) => s,
		Err(e) => panic!("failed to execute {:?}: {}", c, e),
	};
	if !output.status.success() {
		panic!("command exited with error: {:?}: {}", c, output.status);
	}
	String::from_utf8(output.stdout).unwrap()
}

fn main() {
    let bindings = bindgen::Builder::default()
        .header("llvm.h")
        .generate()
        .expect("unable to generate bindings from llvm.h");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_path.join("llvm.rs");
    bindings
        .write_to_file(&out_file)
        .expect(&format!("unable to writing bindings to {}", out_file.display()));

	let mut cmd = Command::new("llvm-config");
	cmd.arg("--cxxflags")
		.arg("--ldflags")
		.arg("--system-libs")
		.arg("--libs")
		.arg("x86");

	for arg in output(&mut cmd).split_whitespace() {
		if arg.starts_with("-l") {
			link_lib(&arg[2..]);
			continue
		}

		if arg.starts_with("-L") {
			link_search(&arg[2..]);
		}
	}

	link_lib("c++");
}
