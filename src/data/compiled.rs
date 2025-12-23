// src/data/compiled.rs
#![allow(dead_code)]

use crate::data::ids::*;
use crate::data::xml::RawXml;
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct GoodDef {
    pub id: GoodId,
    pub display_name: String,
    pub base_price: f64,
    pub decay_rate: f64,
    pub stackable: bool,
}

#[derive(Debug, Clone)]
pub struct NeedDef {
    pub id: NeedId,
    pub good: GoodId,
    pub amount: f64,
    pub interval_ticks: u64,
    pub priority: i32,
}

#[derive(Debug, Clone)]
pub struct HouseholdTypeDef {
    pub id: HouseholdTypeId,
    pub display_name: String,
    pub starting_cash: f64,
    pub starting_inventory: Vec<(GoodId, f64)>,
    pub needs: Vec<NeedId>,
    pub income_per_tick: f64,
}

#[derive(Debug, Clone)]
pub struct ProductionRuleDef {
    pub id: RuleId,
    pub display_name: String,
    pub ticks: u64,
    pub inputs: Vec<(GoodId, f64)>,
    pub outputs: Vec<(GoodId, f64)>,
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub goods: Vec<GoodDef>,
    pub needs: Vec<NeedDef>,
    pub household_types: Vec<HouseholdTypeDef>,
    pub production_rules: Vec<ProductionRuleDef>,
    pub good_ids: Interner,
    pub need_ids: Interner,
    pub household_type_ids: Interner,
    pub rule_ids: Interner,
}

impl Structure {
    pub fn compile(raw: RawXml) -> anyhow::Result<Self> {
        let mut good_ids = Interner::default();
        let mut need_ids = Interner::default();
        let mut household_type_ids = Interner::default();
        let mut rule_ids = Interner::default();

        for g in &raw.goods.goods {
            good_ids.intern(&g.id);
        }
        for n in &raw.needs.needs {
            need_ids.intern(&n.id);
        }
        for h in &raw.household_types.types {
            household_type_ids.intern(&h.id);
        }
        for r in &raw.production.rules {
            rule_ids.intern(&r.id);
        }

        let mut goods = vec![None; good_ids.len()];
        for g in raw.goods.goods {
            let id = GoodId(good_ids.intern(&g.id));
            goods[id.0 as usize] = Some(GoodDef {
                id,
                display_name: g.display_name,
                base_price: g.base_price,
                decay_rate: g.decay_rate,
                stackable: g.stackable,
            });
        }
        let goods: Vec<GoodDef> = goods
            .into_iter()
            .map(|o| o.ok_or_else(|| anyhow!("missing good slot")))
            .collect::<Result<_, _>>()?;

        let mut needs = vec![None; need_ids.len()];
        for n in raw.needs.needs {
            let id = NeedId(need_ids.intern(&n.id));
            let good_u32 = good_ids
                .map
                .get(&n.good_ref)
                .ok_or_else(|| anyhow!("need {} references unknown good {}", n.id, n.good_ref))?;
            let good = GoodId(*good_u32);
            needs[id.0 as usize] = Some(NeedDef {
                id,
                good,
                amount: n.amount,
                interval_ticks: n.interval_ticks.max(1),
                priority: n.priority,
            });
        }
        let needs: Vec<NeedDef> = needs
            .into_iter()
            .map(|o| o.ok_or_else(|| anyhow!("missing need slot")))
            .collect::<Result<_, _>>()?;

        let mut household_types = vec![None; household_type_ids.len()];
        for h in raw.household_types.types {
            let id = HouseholdTypeId(household_type_ids.intern(&h.id));

            let mut inv = Vec::new();
            for it in h.starting_inventory.items {
                let gid_u32 = good_ids.map.get(&it.good_ref).ok_or_else(|| {
                    anyhow!(
                        "household_type {} references unknown good {}",
                        h.id,
                        it.good_ref
                    )
                })?;
                inv.push((GoodId(*gid_u32), it.qty));
            }

            let mut nrefs = Vec::new();
            for nr in h.needs.need_refs {
                let nid_u32 = need_ids.map.get(&nr).ok_or_else(|| {
                    anyhow!("household_type {} references unknown need {}", h.id, nr)
                })?;
                nrefs.push(NeedId(*nid_u32));
            }

            household_types[id.0 as usize] = Some(HouseholdTypeDef {
                id,
                display_name: h.display_name,
                starting_cash: h.starting_cash,
                starting_inventory: inv,
                needs: nrefs,
                income_per_tick: h.income_per_tick,
            });
        }
        let household_types: Vec<HouseholdTypeDef> = household_types
            .into_iter()
            .map(|o| o.ok_or_else(|| anyhow!("missing household_type slot")))
            .collect::<Result<_, _>>()?;

        let mut production_rules = vec![None; rule_ids.len()];
        for r in raw.production.rules {
            let id = RuleId(rule_ids.intern(&r.id));

            let mut inputs = Vec::new();
            for it in r.inputs.items {
                let gid_u32 = good_ids.map.get(&it.good_ref).ok_or_else(|| {
                    anyhow!(
                        "rule {} input references unknown good {}",
                        r.id,
                        it.good_ref
                    )
                })?;
                inputs.push((GoodId(*gid_u32), it.qty));
            }

            let mut outputs = Vec::new();
            for it in r.outputs.items {
                let gid_u32 = good_ids.map.get(&it.good_ref).ok_or_else(|| {
                    anyhow!(
                        "rule {} output references unknown good {}",
                        r.id,
                        it.good_ref
                    )
                })?;
                outputs.push((GoodId(*gid_u32), it.qty));
            }

            production_rules[id.0 as usize] = Some(ProductionRuleDef {
                id,
                display_name: r.display_name,
                ticks: r.ticks.max(1),
                inputs,
                outputs,
            });
        }
        let production_rules: Vec<ProductionRuleDef> = production_rules
            .into_iter()
            .map(|o| o.ok_or_else(|| anyhow!("missing rule slot")))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            goods,
            needs,
            household_types,
            production_rules,
            good_ids,
            need_ids,
            household_type_ids,
            rule_ids,
        })
    }

    pub fn good_name(&self, id: GoodId) -> &str {
        &self.goods[id.0 as usize].display_name
    }
}
