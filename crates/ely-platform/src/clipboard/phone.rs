//! Phones: no rich clipboard yet; gpui carries text.

use anyhow::{Result, bail};

use super::Clip;

const NONE: &str = "this phone build has no rich clipboard yet";

pub fn write(_: &Clip) -> Result<()> {
    bail!(NONE)
}

pub fn read() -> Result<Option<Clip>> {
    bail!(NONE)
}

pub fn html() -> Result<Option<String>> {
    bail!(NONE)
}
