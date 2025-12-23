// src/data/mod.rs
pub mod compiled;
pub mod ids;
pub mod xml;

use crate::config::Config;

pub fn load_and_compile(cfg: &Config) -> anyhow::Result<compiled::Structure> {
    let raw = xml::load_all(cfg)?;
    compiled::Structure::compile(raw)
}
