// src/data/xml.rs
use crate::config::Config;
use anyhow::Context;
use quick_xml::de::from_str;
use serde::Deserialize;

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
    pub needs: HouseholdNeedsXml,
    pub income_per_tick: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StartingInventoryXml {
    #[serde(rename = "item", default)]
    pub items: Vec<InventoryItemXml>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InventoryItemXml {
    #[serde(rename = "@good_ref")]
    pub good_ref: String,
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

#[derive(Debug, Clone)]
pub struct RawXml {
    pub goods: GoodsXml,
    pub needs: NeedsXml,
    pub household_types: HouseholdTypesXml,
    pub production: ProductionRulesXml,
}

pub fn load_all(cfg: &Config) -> anyhow::Result<RawXml> {
    let goods_s = std::fs::read_to_string(&cfg.data_paths.goods)
        .with_context(|| format!("reading {}", cfg.data_paths.goods))?;
    let needs_s = std::fs::read_to_string(&cfg.data_paths.needs)
        .with_context(|| format!("reading {}", cfg.data_paths.needs))?;
    let hh_s = std::fs::read_to_string(&cfg.data_paths.households)
        .with_context(|| format!("reading {}", cfg.data_paths.households))?;
    let prod_s = std::fs::read_to_string(&cfg.data_paths.production)
        .with_context(|| format!("reading {}", cfg.data_paths.production))?;

    let goods: GoodsXml = from_str(&goods_s).context("parsing goods.xml")?;
    let needs: NeedsXml = from_str(&needs_s).context("parsing needs.xml")?;
    let household_types: HouseholdTypesXml = from_str(&hh_s).context("parsing households.xml")?;
    let production: ProductionRulesXml = from_str(&prod_s).context("parsing production.xml")?;

    Ok(RawXml { goods, needs, household_types, production })
}
