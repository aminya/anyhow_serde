use anyhow_serde::anyhow;

#[derive(Debug)]
struct Error;

fn main() {
    let _ = anyhow!(Error);
}
