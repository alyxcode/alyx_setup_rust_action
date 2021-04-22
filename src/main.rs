use std::{
    env::var,
    fmt::Write as _,
    io::{stderr, stdout, Write as _},
    process::{exit, Command, Output},
};

fn main() {
    rustup_version();
    rustup_set_profile();
    rustup_install_toolchain();
    rustup_default_toolchain();
    rustup_show_toolchain();
    rustup_show_components();
    rustup_show_targets();
    rustup_query_versions();
}

fn rustup_version() {
    let command = "rustup --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
}

fn rustup_set_profile() {
    let profile = input("profile");
    let profile = match profile.as_str() {
        profile @ "minimal" => profile,
        profile @ "default" => profile,
        "complete" => "default",
        _ => "minimal",
    };
    let command = format!("rustup set profile {}", profile);
    exec(command);
}

fn rustup_install_toolchain() {
    let mut command = "rustup toolchain install ".to_string();
    let components: Vec<_> = input("components")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    for component in components.iter() {
        write!(&mut command, "--component={} ", component).unwrap();
    }
    let targets: Vec<_> = input("targets")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    for target in targets.iter() {
        write!(&mut command, "--target={} ", target).unwrap();
    }
    let toolchain = input("toolchain");
    if toolchain.starts_with("nightly") {
        if input("downgrade").parse().unwrap_or(true) {
            write!(&mut command, "--allow-downgrade ").unwrap();
        } else {
            write!(&mut command, "--force ").unwrap();
        }
    }
    write!(&mut command, "{}", toolchain).unwrap();
    exec(command);
}

fn rustup_default_toolchain() {
    let toolchain = input("toolchain");
    if input("default").parse().unwrap_or(true) {
        let command = format!("rustup default {}", toolchain);
        exec(&command);
    }
}

fn rustup_show_toolchain() {
    let command = "rustup show active-toolchain";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
}

fn rustup_show_components() {
    let toolchain = input("toolchain");
    let command = format!("rustup component list --toolchain {}", toolchain);
    let output = exec(command);
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|l| l.strip_suffix("(installed)").map(str::trim))
        .for_each(|component| println!("{}", component));
}

fn rustup_show_targets() {
    let toolchain = input("toolchain");
    let command = format!("rustup target list --toolchain {}", toolchain);
    let output = exec(&command);
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|l| l.strip_suffix("(installed)").map(str::trim))
        .for_each(|target| println!("{}", target));
}

fn rustup_query_versions() {
    let command = "rustup --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
    let command = "rustc --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
    let command = "rustdoc --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
    let command = "cargo --version";
    let output = exec(command);
    stdout().write_all(&output.stdout).unwrap_or_else(|error| {
        eprintln!("stdout error: {}", error);
        exit(1);
    });
    let components = input("components");
    if components.contains("rustfmt") {
        let command = "rustfmt --version";
        let output = exec(command);
        stdout().write_all(&output.stdout).unwrap_or_else(|error| {
            eprintln!("stdout error: {}", error);
            exit(1);
        });
    }
    if components.contains("clippy") {
        let command = "cargo clippy --version";
        let output = exec(command);
        stdout().write_all(&output.stdout).unwrap_or_else(|error| {
            eprintln!("stdout error: {}", error);
            exit(1);
        });
    }
}

fn exec<S: AsRef<str>>(command: S) -> Output {
    let command = command.as_ref();
    println!("❯ {}", command);
    let split: Vec<_> = command.trim().split_whitespace().collect();
    let (executable, arguments) = split.split_first().unwrap();
    let output = Command::new(executable).args(arguments).output().unwrap();
    if output.status.success() {
        output
    } else {
        eprintln!("--- status ---");
        eprintln!("{}", output.status);
        eprintln!();
        eprintln!("--- stdout ---");
        stderr().write_all(&output.stdout).unwrap_or_else(|error| {
            eprintln!("stderr error: {}", error);
            exit(1);
        });
        eprintln!();
        eprintln!("--- stderr ---");
        stderr().write_all(&output.stderr).unwrap_or_else(|error| {
            eprintln!("stderr error: {}", error);
            exit(1);
        });
        eprintln!();
        exit(1);
    }
}

fn input(name: &str) -> String {
    let variable = format!("INPUT_{}", name.to_uppercase());
    env(variable.as_str())
}

fn env(name: &str) -> String {
    var(name).unwrap_or_else(|error| {
        eprintln!("environment variable error: {}", error);
        exit(1);
    })
}
