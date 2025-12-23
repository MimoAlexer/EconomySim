// src/simulation/market.rs
#![allow(dead_code)]

use crate::data::compiled::Structure;
use crate::data::ids::GoodId;

#[derive(Debug, Clone)]
pub struct Market {
    pub price: Vec<f64>,
    pub demand: Vec<f64>,
    pub supply: Vec<f64>,
    pub adjustment_rate: f64,
}

impl Market {
    pub fn new(s: &Structure) -> Self {
        let mut price = vec![0.0; s.goods.len()];
        for g in &s.goods {
            price[g.id.0 as usize] = g.base_price.max(0.01);
        }
        let n = s.goods.len();
        Self {
            price,
            demand: vec![0.0; n],
            supply: vec![0.0; n],
            adjustment_rate: 0.02,
        }
    }

    pub fn reset_pressures(&mut self) {
        self.demand.fill(0.0);
        self.supply.fill(0.0);
    }

    pub fn note_demand(&mut self, g: GoodId, amount: f64) {
        self.demand[g.0 as usize] += amount.max(0.0);
    }

    pub fn note_supply(&mut self, g: GoodId, amount: f64) {
        self.supply[g.0 as usize] += amount.max(0.0);
    }

    pub fn adjust_prices(&mut self) {
        for i in 0..self.price.len() {
            let d = self.demand[i];
            let s = self.supply[i];
            let pressure = if d + s <= 0.0 { 0.0 } else { (d - s) / (d + s) }.clamp(-0.5, 0.5);
            let next = self.price[i] * (1.0 + self.adjustment_rate * pressure);
            self.price[i] = next.max(0.01);
        }
    }
}
