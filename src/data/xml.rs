// src/data/xml.rs
use crate::config::Config;
use anyhow::{anyhow, Context};
use quick_xml::de::from_str;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct GoodsXml {
    #[serde(rename = "good")]
    pub goods: Vec<GoodXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoodXml {
    #[serde(rename = "@id")]
    pub id: String,
    pub display_name: String,
    pub base_price: f64,
    pub decay_rate: f64,
    pub stackable: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NeedsXml {
    #[serde(rename = "need")]
    pub needs: Vec<NeedXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NeedXml {
    #[serde(rename = "@id")]
    pub id: String,
    pub good_ref: String,
    pub amount: f64,
    pub interval_ticks: u64,
    pub priority: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HouseholdTypesXml {
    #[serde(rename = "household_type")]
    pub types: Vec<HouseholdTypeXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HouseholdTypeXml {
    #[serde(rename = "@id")]
    pub id: String,
    pub display_name: String,
    pub starting_cash: f64,
    #[serde(default)]
    pub starting_inventory: StartingInventoryXml,
    #[serde(default)]
    pub starting_portfolio: StartingPortfolioXml,
    #[serde(default)]
    pub needs: HouseholdNeedsXml,
    pub income_per_tick: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StartingInventoryXml {
    #[serde(rename = "item", default)]
    pub items: Vec<InventoryItemXml>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StartingPortfolioXml {
    #[serde(rename = "item", default)]
    pub items: Vec<PortfolioItemXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InventoryItemXml {
    #[serde(rename = "@good_ref")]
    pub good_ref: String,
    #[serde(rename = "@qty")]
    pub qty: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PortfolioItemXml {
    #[serde(rename = "@stock_ref")]
    pub stock_ref: String,
    #[serde(rename = "@qty")]
    pub qty: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct HouseholdNeedsXml {
    #[serde(rename = "need_ref", default)]
    pub need_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProductionRulesXml {
    #[serde(rename = "rule")]
    pub rules: Vec<ProductionRuleXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProductionRuleXml {
    #[serde(rename = "@id")]
    pub id: String,
    pub display_name: String,
    pub ticks: u64,
    #[serde(default)]
    pub inputs: RuleIOXml,
    #[serde(default)]
    pub outputs: RuleIOXml,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RuleIOXml {
    #[serde(rename = "item", default)]
    pub items: Vec<InventoryItemXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StocksXml {
    #[serde(rename = "stock")]
    pub stocks: Vec<StockXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StockXml {
    #[serde(rename = "@id")]
    pub id: String,
    pub display_name: String,
    pub base_price: f64,
    pub volatility: f64,
    pub shares_outstanding: u64,
}

#[derive(Debug, Clone)]
pub struct RawXml {
    pub goods: GoodsXml,
    pub needs: NeedsXml,
    pub household_types: HouseholdTypesXml,
    pub production: ProductionRulesXml,
    pub stocks: StocksXml,
}

fn read_text_with_fallbacks(p: &str) -> anyhow::Result<String> {
    let md = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut candidates: Vec<PathBuf> = Vec::new();
    candidates.push(PathBuf::from(p));
    candidates.push(md.join(p));
    for c in candidates {
        if c.exists() {
            return std::fs::read_to_string(&c).with_context(|| format!("reading {}", c.display()));
        }
    }
    Err(anyhow!("file not found: {}", p))
}

pub fn load_all(cfg: &Config) -> anyhow::Result<RawXml> {
    let goods_s = read_text_with_fallbacks(&cfg.data_paths.goods)?;
    let needs_s = read_text_with_fallbacks(&cfg.data_paths.needs)?;
    let hh_s = read_text_with_fallbacks(&cfg.data_paths.households)?;
    let prod_s = read_text_with_fallbacks(&cfg.data_paths.production)?;
    let stocks_s = read_text_with_fallbacks(&cfg.data_paths.stocks)?;

    let goods: GoodsXml = from_str(&goods_s).context("parsing goods.xml")?;
    let needs: NeedsXml = from_str(&needs_s).context("parsing needs.xml")?;
    let household_types: HouseholdTypesXml = from_str(&hh_s).context("parsing households.xml")?;
    let production: ProductionRulesXml = from_str(&prod_s).context("parsing production.xml")?;
    let stocks: StocksXml = from_str(&stocks_s).context("parsing stocks.xml")?;

    Ok(RawXml { goods, needs, household_types, production, stocks })
}
