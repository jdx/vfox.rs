use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::set_envs;

pub mod helpers;

#[plugin_fn]
pub fn setup() -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("setup")?
        .pkgx()?
        .with_packages(vec!["curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn clippy() -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("clippy")?
        .pkgx()?
        .with_packages(vec!["curl", "wget"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .with_exec(vec!["rustup", "component", "add", "clippy"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "cargo", "install", "clippy-sarif", "--version", "0.3.0"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH","cargo", "install", "sarif-fmt", "--version", "0.3.0"])?
        .with_exec(vec![
            "PATH=$HOME/.cargo/bin:$PATH",
            "cargo",
            "clippy",
            "--all-features",
            "--message-format=json",
            " | clippy-sarif | tee rust-clippy-results.sarif | sarif-fmt)"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn llvmcov() -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("llvmcov")?
        .pkgx()?
        .with_packages(vec!["curl", "wget"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .with_exec(vec!["rustup", "component", "add", "llvm-tools"])?
        .with_exec(vec![
            "wget",
            "https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.5.36/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz"])?
        .with_exec(vec!["tar", "xvf", "cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz"])?
        .with_exec(vec!["mv", "cargo-llvm-cov", "/usr/local/bin"])?
        .with_exec(vec![
            "PATH=$HOME/.cargo/bin:$PATH",
            "cargo", 
            "llvm-cov",
            "--all-features",
            "--lib",
            "--workspace",
            "--lcov",
            "--output-path",
            "lcov.info"
        ])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("test")?
        .pkgx()?
        .with_packages(vec!["curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "cargo", "test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("build")?
        .pkgx()?
        .with_packages(vec!["curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "cargo", "build", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn target_add(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("target_add")?
        .pkgx()?
        .with_packages(vec!["curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "rustup", "target", "add", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn component_add(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("component_add")?
        .pkgx()?
        .with_packages(vec![ "curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "rustup", "component", "add", &args])?
        .stdout()?;
    Ok(stdout)
}
