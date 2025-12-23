use std::io::{self, stdout};
use std::time::Duration;

use anyhow::Context;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use economysim::{
    app::App,
    config::Cli,
    config::Config,
    data, ui,
    util::{self, Ticker},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = Config::load(cli.config.as_deref()).context("loading config")?;
    let structure = data::load_and_compile(&cfg).context("loading structure")?;
    let mut app = App::new(cfg, structure);

    let _guard = TerminalGuard::enter()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut sim_ticker = Ticker::new(app.cfg.tick_hz);
    let mut ui_ticker = Ticker::new(app.cfg.ui_hz);

    loop {
        if let Some(ev) = util::poll_event(Duration::from_millis(5))? {
            if app.on_event(ev)? {
                break;
            }
        }

        if sim_ticker.should_tick() && !app.paused {
            app.sim.tick();
            app.recompute_metrics();
        }

        if ui_ticker.should_tick() {
            terminal.draw(|f| ui::render::render(f, &mut app))?;
        }
    }

    Ok(())
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if let Err(err) = disable_raw_mode() {
            eprintln!("Warning: failed to disable raw mode: {err}");
        }
        if let Err(err) = execute!(io::stdout(), LeaveAlternateScreen) {
            eprintln!("Warning: failed to leave alternate screen: {err}");
        }
    }
}
