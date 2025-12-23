// src/main.rs
mod app;
mod config;
mod util;

mod data;
mod simulation;
mod ui;

use anyhow::Context;
use clap::Parser;
use config::{Cli, Config};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = Config::load(cli.config.as_deref()).context("loading config")?;
    let structure = data::load_and_compile(&cfg).context("loading structure")?;

    enable_raw_mode().context("enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("enter alt screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("create terminal")?;
    terminal.clear().ok();

    let res = run(terminal, cfg, structure);

    disable_raw_mode().ok();
    execute!(io::stdout(), LeaveAlternateScreen).ok();

    res
}

fn run(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    cfg: Config,
    structure: data::compiled::Structure,
) -> anyhow::Result<()> {
    let mut app = app::App::new(cfg, structure);
    let mut tick_clock = util::Ticker::new(app.cfg.tick_hz);
    let mut ui_clock = util::Ticker::new(app.cfg.ui_hz);

    loop {
        if let Some(ev) = util::poll_event(Duration::from_millis(1))? {
            if app.on_event(ev)? {
                break;
            }
        }

        while tick_clock.should_tick() && !app.paused {
            app.sim.tick();
            app.recompute_metrics();
        }

        if ui_clock.should_tick() {
            terminal.draw(|f| ui::render::render(f, &mut app))?;
        }
    }

    Ok(())
}
