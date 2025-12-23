// src/simulation/engine.rs
use crate::data::compiled::Structure;
use crate::data::ids::HouseholdTypeId;
use crate::simulation::economy::EconomyMetrics;
use crate::simulation::household::{Household, HouseholdId, Inventory, NeedState};
use crate::simulation::market::Market;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
pub struct Simulation {
    pub structure: Structure,
    pub households: Vec<Household>,
    pub market: Market,
    pub tick: u64,
    rng: ChaCha8Rng,
    pub metrics: EconomyMetrics,
}

impl Simulation {
    pub fn new(structure: Structure, seed: u64, start_households: usize) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let market = Market::new(&structure);
        let mut sim = Self {
            structure,
            households: Vec::new(),
            market,
            tick: 0,
            rng,
            metrics: EconomyMetrics::default(),
        };
        sim.spawn_households(start_households);
        sim
    }

    fn spawn_households(&mut self, n: usize) {
        let tlen = self.structure.household_types.len().max(1);
        for i in 0..n {
            let t = (self.rng.gen::<u32>() as usize) % tlen;
            let kind = HouseholdTypeId(t as u32);
            let td = &self.structure.household_types[t];

            let mut inv = Inventory::new(self.structure.goods.len());
            for &(g, q) in &td.starting_inventory {
                inv.add(g, q);
            }

            let mut needs = Vec::new();
            for &nid in &td.needs {
                let nd = &self.structure.needs[nid.0 as usize];
                needs.push(NeedState {
                    need: nid,
                    next_due_in: nd.interval_ticks,
                    fulfilled_last: true,
                });
            }

            self.households.push(Household::new(HouseholdId(i as u32), kind, td.starting_cash, inv, needs));
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        self.market.reset_pressures();

        for h in &mut self.households {
            let td = &self.structure.household_types[h.kind.0 as usize];
            h.apply_income(td.income_per_tick);
            h.apply_decay(&self.structure);
            h.step_needs(&self.structure);
        }

        for h in &mut self.households {
            let mut want = Vec::new();
            for ns in &h.needs {
                if ns.fulfilled_last {
                    continue;
                }
                let nd = &self.structure.needs[ns.need.0 as usize];
                want.push((nd.good, nd.amount));
            }
            want.sort_by_key(|(g, _)| g.0);

            for (g, amount) in want {
                let price = self.market.price[g.0 as usize];
                let cost = price * amount;
                self.market.note_demand(g, amount);
                if h.cash >= cost {
                    h.cash -= cost;
                    h.inventory.add(g, amount);
                }
            }
        }

        self.market.adjust_prices();
        self.metrics.tick = self.tick;
    }
}
