// src/simulation/stock_market.rs
#![allow(dead_code)]

use crate::data::compiled::Structure;
use crate::data::ids::StockId;

#[derive(Debug, Clone)]
pub struct StockMarket {
    pub price: Vec<f64>,
    pub demand: Vec<f64>,
    pub supply: Vec<f64>,
}

impl StockMarket {
    pub fn new(s: &Structure) -> Self {
        let mut price = vec![0.0; s.stocks.len()];
        for st in &s.stocks {
            price[st.id.0 as usize] = st.base_price.max(0.01);
        }
        let n = s.stocks.len();
        Self {
            price,
            demand: vec![0.0; n],
            supply: vec![0.0; n],
        }
    }

    pub fn reset(&mut self) {
        self.demand.fill(0.0);
        self.supply.fill(0.0);
    }

    pub fn note_buy(&mut self, id: StockId, qty: f64) {
        self.demand[id.0 as usize] += qty.max(0.0);
    }

    pub fn note_sell(&mut self, id: StockId, qty: f64) {
        self.supply[id.0 as usize] += qty.max(0.0);
    }

    pub fn adjust(&mut self, s: &Structure) {
        for st in &s.stocks {
            let i = st.id.0 as usize;
            let d = self.demand[i];
            let sup = self.supply[i];
            let pressure = if d + sup <= 0.0 { 0.0 } else { (d - sup) / (d + sup) }.clamp(-0.8, 0.8);
            let vol = st.volatility.clamp(0.0, 1.0);
            let next = self.price[i] * (1.0 + vol * pressure);
            self.price[i] = next.max(0.01);
        }
    }
}
