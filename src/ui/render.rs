// src/ui/render.rs
use crate::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let regions = crate::ui::layout::split(f.area(), app.show_debug);

    render_header(f, regions.header, app);
    render_footer(f, regions.footer);

    match app.view {
        View::Overview => render_overview(f, regions.main, app),
        View::Households => render_households(f, regions.main, app),
        View::Markets => render_markets(f, regions.main, app),
        View::Debug => render_debug_main(f, regions.main, app),
    }

    if let Some(dbg) = regions.debug {
        render_debug_side(f, dbg, app);
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        "EconomySim | tick={} | households={} | paused={} | view={:?}",
        app.derived.tick, app.derived.households, app.paused, app.view
    );
    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(block, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let help = "q quit | p pause | . step | ←/→ view | ↑/↓ select | r reset | d debug";
    let p = Paragraph::new(help).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}

fn render_overview(f: &mut Frame, area: Rect, app: &App) {
    let lines = vec![
        Line::from(format!("Total cash: {:.2}", app.derived.total_cash)),
        Line::from(format!("Average utility: {:.3}", app.derived.avg_utility)),
        Line::from(""),
        Line::from("Structure:"),
        Line::from(format!("  Goods: {}", app.sim.structure.goods.len())),
        Line::from(format!("  Needs: {}", app.sim.structure.needs.len())),
        Line::from(format!("  Household types: {}", app.sim.structure.household_types.len())),
        Line::from(format!("  Production rules: {}", app.sim.structure.production_rules.len())),
    ];
    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Overview"));
    f.render_widget(p, area);
}

fn render_households(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    let items: Vec<ListItem> = app
        .sim
        .households
        .iter()
        .map(|h| {
            let kind = &app.sim.structure.household_types[h.kind.0 as usize].display_name;
            ListItem::new(format!("#{} {} cash={:.1} util={:.2}", h.id.0, kind, h.cash, h.utility))
        })
        .collect();

    let mut state = ListState::default();
    if !items.is_empty() {
        state.select(Some(app.selected_household.min(items.len() - 1)));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Households"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(list, chunks[0], &mut state);

    render_household_detail(f, chunks[1], app);
}

fn render_household_detail(f: &mut Frame, area: Rect, app: &App) {
    if app.sim.households.is_empty() {
        let p = Paragraph::new("No households").block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(p, area);
        return;
    }

    let idx = app.selected_household.min(app.sim.households.len() - 1);
    let h = &app.sim.households[idx];
    let kind = &app.sim.structure.household_types[h.kind.0 as usize].display_name;

    let mut lines: Vec<Line> = vec![
        Line::from(format!("Household #{}", h.id.0)),
        Line::from(format!("Type: {}", kind)),
        Line::from(format!("Cash: {:.2}", h.cash)),
        Line::from(format!("Utility: {:.3}", h.utility)),
        Line::from(""),
        Line::from("Last consumed:"),
    ];

    if h.last_consumed.is_empty() {
        lines.push(Line::from("  (none)"));
    } else {
        for (g, a) in &h.last_consumed {
            let name = app.sim.structure.good_name(*g);
            lines.push(Line::from(format!("  {} x{:.2}", name, a)));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from("Inventory (non-zero):"));
    for gd in &app.sim.structure.goods {
        let q = h.inventory.get(gd.id);
        if q.abs() > 1e-9 {
            lines.push(Line::from(format!("  {}: {:.2}", gd.display_name, q)));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from("Needs:"));
    for ns in &h.needs {
        let nd = &app.sim.structure.needs[ns.need.0 as usize];
        let gname = app.sim.structure.good_name(nd.good);
        lines.push(Line::from(format!(
            "  {} need: {} amt={:.2} due_in={} ok_last={}",
            ns.need.0, gname, nd.amount, ns.next_due_in, ns.fulfilled_last
        )));
    }

    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn render_markets(f: &mut Frame, area: Rect, app: &App) {
    let mut rows: Vec<Row> = Vec::new();
    for gd in &app.sim.structure.goods {
        let i = gd.id.0 as usize;
        rows.push(Row::new(vec![
            gd.display_name.clone(),
            format!("{:.3}", app.sim.market.price[i]),
            format!("{:.2}", app.sim.market.demand[i]),
            format!("{:.2}", app.sim.market.supply[i]),
        ]));
    }

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ],
    )
        .header(Row::new(vec!["Good", "Price", "Demand", "Supply"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title("Market"))
        .column_spacing(1);

    f.render_widget(table, area);
}

fn render_debug_main(f: &mut Frame, area: Rect, app: &App) {
    let text = vec![
        Line::from("Debug View"),
        Line::from(format!("tick={}", app.sim.tick)),
        Line::from(format!("seed={}", app.cfg.seed)),
    ];
    let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Debug"));
    f.render_widget(p, area);
}

fn render_debug_side(f: &mut Frame, area: Rect, app: &App) {
    let lines = vec![
        Line::from("Derived"),
        Line::from(format!("total_cash={:.2}", app.derived.total_cash)),
        Line::from(format!("avg_utility={:.3}", app.derived.avg_utility)),
        Line::from(""),
        Line::from("Config"),
        Line::from(format!("tick_hz={}", app.cfg.tick_hz)),
        Line::from(format!("ui_hz={}", app.cfg.ui_hz)),
        Line::from(format!("start_households={}", app.cfg.start_households)),
        Line::from(format!("debug={}", app.cfg.debug)),
    ];
    let p = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Panel"))
        .wrap(Wrap { trim: true });
    f.render_widget(p, area);
}
