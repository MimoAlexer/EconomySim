// src/config.rs
use clap::Parser;
use serde::Deserialize;
use std::path::Path;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataPaths {
    pub goods: String,
    pub needs: String,
    pub households: String,
    pub production: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub tick_hz: u64,
    pub ui_hz: u64,
    pub seed: u64,
    pub start_households: usize,
    pub debug: bool,
    pub data_paths: DataPaths,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_hz: 20,
            ui_hz: 30,
            seed: 1337,
            start_households: 200,
            debug: false,
            data_paths: DataPaths {
                goods: "data/goods.xml".to_string(),
                needs: "data/needs.xml".to_string(),
                households: "data/households.xml".to_string(),
                production: "data/production.xml".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load(path: Option<&str>) -> anyhow::Result<Self> {
        let path = path.unwrap_or("config.toml");
        if !Path::new(path).exists() {
            return Ok(Self::default());
        }
        let txt = std::fs::read_to_string(path)?;
        let mut cfg: Config = toml::from_str(&txt)?;
        cfg.tick_hz = cfg.tick_hz.max(1);
        cfg.ui_hz = cfg.ui_hz.max(1);
        Ok(cfg)
    }
}
