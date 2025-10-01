#![deny(unused_must_use)]

use anyhow_serde::anyhow;

fn main() -> anyhow_serde::Result<()> {
    if true {
        // meant to write bail!
        anyhow!("it failed");
    }
    Ok(())
}
