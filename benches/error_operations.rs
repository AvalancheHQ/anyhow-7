use anyhow::Context;
use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn error_creation() -> anyhow::Error {
    anyhow::anyhow!("error message")
}

#[divan::bench]
fn error_with_context() -> anyhow::Result<()> {
    Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
        .context("operation failed")
}

#[divan::bench]
fn error_with_dynamic_context() -> anyhow::Result<()> {
    let value = 42;
    Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
        .with_context(|| format!("operation failed with value: {}", value))
}

#[divan::bench]
fn error_downcast() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "not found").into();
    divan::black_box(error.downcast_ref::<io::Error>());
}

#[divan::bench]
fn error_chain_iteration() {
    let error: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "not found").into();
    let result: anyhow::Result<()> = Err(error);
    let error = result.context("level 1").unwrap_err();
    let result: anyhow::Result<()> = Err(error);
    let error = result.context("level 2").unwrap_err();
    
    for cause in error.chain() {
        divan::black_box(cause);
    }
}

#[divan::bench]
fn error_display_format() -> String {
    let error = anyhow::anyhow!("error message");
    format!("{}", error)
}

#[divan::bench]
fn error_debug_format() -> String {
    let error = anyhow::anyhow!("error message");
    format!("{:?}", error)
}
