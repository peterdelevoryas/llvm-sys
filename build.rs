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
    let mut cmd = Command::new("llvm-config");
    cmd.arg("--libs")
        .arg("core")
        .arg("x86codegen")
        .arg("--system-libs");

    let llvm_config_output = output(&mut cmd);
    println!("llvm-config output: {}", llvm_config_output);

    for arg in llvm_config_output.split_whitespace() {
        if arg.starts_with("-l") {
            link_lib(&arg[2..]);
            continue
        }

        if arg.starts_with("-L") {
            link_search(&arg[2..]);
        }
    }

    link_lib("c");
    link_lib("z");
}
