// src/simulation/economy.rs
#[derive(Debug, Clone, Default)]
pub struct EconomyMetrics {
    pub tick: u64,
    pub households: usize,
    pub total_cash: f64,
    pub avg_utility: f64,
}
