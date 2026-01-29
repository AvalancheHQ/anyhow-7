use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

/// Benchmark creating errors using anyhow! macro
#[divan::bench]
fn create_error_from_macro() -> Result<()> {
    Err(anyhow!("An error occurred"))
}

/// Benchmark creating errors from std::io::Error
#[divan::bench]
fn create_error_from_io() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(anyhow::Error::from(io_err))
}

/// Benchmark adding context to errors
#[divan::bench]
fn add_context() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).context("Failed to read configuration")
}

/// Benchmark adding context with closure
#[divan::bench]
fn add_context_with_closure() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).with_context(|| "Failed to read configuration")
}

/// Benchmark error downcasting
#[divan::bench]
fn downcast_error() {
    let err: anyhow::Error = io::Error::new(io::ErrorKind::NotFound, "file not found").into();
    divan::black_box(err.downcast_ref::<io::Error>());
}

/// Benchmark error chain iteration
#[divan::bench]
fn iterate_error_chain() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: Result<()> = Err(io_err).context("Failed to read configuration");
    
    if let Err(err) = err {
        for cause in err.chain() {
            divan::black_box(cause);
        }
    }
}

/// Benchmark error formatting
#[divan::bench]
fn format_error() -> String {
    let err: anyhow::Error = anyhow!("An error occurred");
    format!("{}", err)
}

/// Benchmark error debug formatting
#[divan::bench]
fn format_error_debug() -> String {
    let err: anyhow::Error = anyhow!("An error occurred");
    format!("{:?}", err)
}
