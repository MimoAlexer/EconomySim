// src/util.rs
use crossterm::event::{self, Event};
use std::time::{Duration, Instant};

pub struct Ticker {
    period: Duration,
    next: Instant,
}

impl Ticker {
    pub fn new(hz: u64) -> Self {
        let period = Duration::from_secs_f64(1.0 / hz as f64);
        let now = Instant::now();
        Self { period, next: now + period }
    }

    pub fn should_tick(&mut self) -> bool {
        let now = Instant::now();
        if now >= self.next {
            self.next = self.next + self.period;
            true
        } else {
            false
        }
    }
}

pub fn poll_event(timeout: Duration) -> anyhow::Result<Option<Event>> {
    if event::poll(timeout)? {
        Ok(Some(event::read()?))
    } else {
        Ok(None)
    }
}
