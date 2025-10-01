#![allow(clippy::let_underscore_untyped)]

#[rustversion::not(nightly)]
#[ignore = "requires nightly"]
#[test]
fn test_backtrace() {}

#[rustversion::nightly]
#[test]
fn test_backtrace() {
    use anyhow_serde::anyhow;

    let error = anyhow!("oh no!");
    let _ = error.backtrace();
}
