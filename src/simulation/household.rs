// src/simulation/household.rs
use crate::data::compiled::{NeedDef, Structure};
use crate::data::ids::{GoodId, HouseholdTypeId, NeedId, StockId};
use smallvec::SmallVec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HouseholdId(pub u32);

#[derive(Debug, Clone)]
pub struct Inventory {
    pub qty: Vec<f64>,
}

impl Inventory {
    pub fn new(goods_len: usize) -> Self {
        Self { qty: vec![0.0; goods_len] }
    }

    pub fn get(&self, g: GoodId) -> f64 {
        self.qty[g.0 as usize]
    }

    pub fn add(&mut self, g: GoodId, amount: f64) {
        let v = &mut self.qty[g.0 as usize];
        *v += amount;
        if *v < 0.0 {
            *v = 0.0;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Portfolio {
    pub qty: Vec<f64>,
}

impl Portfolio {
    pub fn new(stocks_len: usize) -> Self {
        Self { qty: vec![0.0; stocks_len] }
    }

    pub fn get(&self, s: StockId) -> f64 {
        self.qty[s.0 as usize]
    }

    pub fn add(&mut self, s: StockId, amount: f64) {
        let v = &mut self.qty[s.0 as usize];
        *v += amount;
        if *v < 0.0 {
            *v = 0.0;
        }
    }

    pub fn clear_all(&mut self) {
        self.qty.fill(0.0);
    }
}

#[derive(Debug, Clone)]
pub struct NeedState {
    pub need: NeedId,
    pub next_due_in: u64,
    pub fulfilled_last: bool,
}

#[derive(Debug, Clone)]
pub struct Household {
    pub id: HouseholdId,
    pub kind: HouseholdTypeId,
    pub cash: f64,
    pub inventory: Inventory,
    pub portfolio: Portfolio,
    pub needs: Vec<NeedState>,
    pub utility: f64,
    pub last_consumed: SmallVec<[(GoodId, f64); 8]>,
}

impl Household {
    pub fn new(
        id: HouseholdId,
        kind: HouseholdTypeId,
        cash: f64,
        inventory: Inventory,
        portfolio: Portfolio,
        needs: Vec<NeedState>,
    ) -> Self {
        Self {
            id,
            kind,
            cash,
            inventory,
            portfolio,
            needs,
            utility: 0.0,
            last_consumed: SmallVec::new(),
        }
    }

    pub fn apply_income(&mut self, income_per_tick: f64) {
        self.cash += income_per_tick;
    }

    pub fn apply_decay(&mut self, s: &Structure) {
        for gd in &s.goods {
            let q = self.inventory.get(gd.id);
            if q <= 0.0 || gd.decay_rate <= 0.0 {
                continue;
            }
            let decayed = q * gd.decay_rate;
            self.inventory.add(gd.id, -decayed);
        }
    }

    pub fn step_needs(&mut self, s: &Structure) {
        self.last_consumed.clear();
        let mut indices: Vec<usize> = (0..self.needs.len()).collect();
        indices.sort_by_key(|&i| {
            let nd = &s.needs[self.needs[i].need.0 as usize];
            (nd.priority, self.needs[i].need.0)
        });

        for i in indices {
            let state = &mut self.needs[i];
            if state.next_due_in > 0 {
                state.next_due_in -= 1;
                continue;
            }

            let nd: &NeedDef = &s.needs[state.need.0 as usize];
            let have = self.inventory.get(nd.good);

            if have >= nd.amount {
                self.inventory.add(nd.good, -nd.amount);
                self.utility += 1.0;
                state.fulfilled_last = true;
                self.last_consumed.push((nd.good, nd.amount));
            } else {
                self.utility -= 0.5;
                state.fulfilled_last = false;
            }

            state.next_due_in = nd.interval_ticks;
        }
    }
}
