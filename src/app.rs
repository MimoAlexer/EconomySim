// src/app.rs
use crate::config::Config;
use crate::data::compiled::Structure;
use crate::simulation::{engine::Simulation, economy::EconomyMetrics};
use crossterm::event::{Event, KeyCode, KeyEvent};

#[derive(Debug, Copy, Clone)]
pub enum View {
    Overview,
    Households,
    Goods,
    Stocks,
}

#[derive(Debug)]
pub struct App {
    pub cfg: Config,
    pub sim: Simulation,
    pub paused: bool,
    pub view: View,
    pub selected_household: usize,
    pub derived: EconomyMetrics,
    pub last_action: String,
}

impl App {
    pub fn new(cfg: Config, structure: Structure) -> Self {
        let sim = Simulation::new(structure, cfg.seed, cfg.start_households);
        let mut app = Self {
            cfg,
            sim,
            paused: false,
            view: View::Overview,
            selected_household: 0,
            derived: EconomyMetrics::default(),
            last_action: String::new(),
        };
        app.recompute_metrics();
        app
    }

    pub fn reset(&mut self) {
        let structure = self.sim.structure.clone();
        self.sim = Simulation::new(structure, self.cfg.seed, self.cfg.start_households);
        self.selected_household = 0;
        self.last_action.clear();
        self.recompute_metrics();
    }

    pub fn recompute_metrics(&mut self) {
        let mut total_cash = 0.0;
        let mut total_utility = 0.0;
        for h in &self.sim.households {
            total_cash += h.cash;
            total_utility += h.utility;
        }
        self.derived = EconomyMetrics {
            tick: self.sim.tick,
            households: self.sim.households.len(),
            total_cash,
            avg_utility: if self.sim.households.is_empty() { 0.0 } else { total_utility / self.sim.households.len() as f64 },
        };
    }

    pub fn on_event(&mut self, ev: Event) -> anyhow::Result<bool> {
        match ev {
            Event::Key(KeyEvent { code, .. }) => Ok(self.on_key(code)),
            _ => Ok(false),
        }
    }

    fn on_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('p') => self.paused = !self.paused,
            KeyCode::Char('.') => {
                if self.paused {
                    self.sim.tick();
                    self.recompute_metrics();
                }
            }
            KeyCode::Char('r') => self.reset(),
            KeyCode::Left => self.prev_view(),
            KeyCode::Right => self.next_view(),
            KeyCode::Up => self.select_prev(),
            KeyCode::Down => self.select_next(),
            KeyCode::Char('x') => {
                self.sim.force_sell_all_stocks();
                self.last_action = "FORCE SELL ALL STOCKS".to_string();
            }
            _ => {}
        }
        false
    }

    fn prev_view(&mut self) {
        self.view = match self.view {
            View::Overview => View::Stocks,
            View::Households => View::Overview,
            View::Goods => View::Households,
            View::Stocks => View::Goods,
        };
    }

    fn next_view(&mut self) {
        self.view = match self.view {
            View::Overview => View::Households,
            View::Households => View::Goods,
            View::Goods => View::Stocks,
            View::Stocks => View::Overview,
        };
    }

    fn select_prev(&mut self) {
        if self.selected_household > 0 {
            self.selected_household -= 1;
        }
    }

    fn select_next(&mut self) {
        if self.selected_household + 1 < self.sim.households.len() {
            self.selected_household += 1;
        }
    }
}
