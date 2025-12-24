// src/ui/render.rs
use crate::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5), Constraint::Length(2)])
        .split(area);

    render_header(f, chunks[0], app);
    match app.view {
        View::Overview => render_overview(f, chunks[1], app),
        View::Households => render_households(f, chunks[1], app),
        View::Goods => render_goods(f, chunks[1], app),
        View::Stocks => render_stocks(f, chunks[1], app),
    }
    render_footer(f, chunks[2], app);
}

fn tab_title(v: View) -> &'static str {
    match v {
        View::Overview => "Overview",
        View::Households => "Households",
        View::Goods => "Goods",
        View::Stocks => "Stocks",
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let tabs = ["Overview", "Households", "Goods", "Stocks"];
    let idx = match app.view {
        View::Overview => 0,
        View::Households => 1,
        View::Goods => 2,
        View::Stocks => 3,
    };
    let t = Tabs::new(tabs)
        .select(idx)
        .block(Block::default().borders(Borders::ALL).title(format!(
            "EconomySim  | tick {} | households {} | paused {}",
            app.derived.tick, app.derived.households, app.paused
        )))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(t, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let help = "q quit | p pause | . step | ←/→ tabs | ↑/↓ select | r reset | x force sell all stocks";
    let msg = if app.last_action.is_empty() { help.to_string() } else { format!("{}  |  last: {}", help, app.last_action) };
    let p = Paragraph::new(msg).block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}

fn render_overview(f: &mut Frame, area: Rect, app: &App) {
    let lines = vec![
        Line::from(Span::styled(tab_title(app.view), Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(format!("Total cash: {:.2}", app.derived.total_cash)),
        Line::from(format!("Average utility: {:.3}", app.derived.avg_utility)),
        Line::from(""),
        Line::from(format!("Goods: {}", app.sim.structure.goods.len())),
        Line::from(format!("Needs: {}", app.sim.structure.needs.len())),
        Line::from(format!("Household types: {}", app.sim.structure.household_types.len())),
        Line::from(format!("Stocks: {}", app.sim.structure.stocks.len())),
    ];
    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Overview")).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn render_households(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(area);

    let items: Vec<ListItem> = app
        .sim
        .households
        .iter()
        .map(|h| {
            let kind = &app.sim.structure.household_types[h.kind.0 as usize].display_name;
            ListItem::new(format!("#{} {:<10} cash={:>8.2} util={:>6.2}", h.id.0, kind, h.cash, h.utility))
        })
        .collect();

    let mut state = ListState::default();
    if !items.is_empty() {
        state.select(Some(app.selected_household.min(items.len() - 1)));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Households"))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    f.render_stateful_widget(list, cols[0], &mut state);

    render_household_detail(f, cols[1], app);
}

fn render_household_detail(f: &mut Frame, area: Rect, app: &App) {
    if app.sim.households.is_empty() {
        f.render_widget(Paragraph::new("No households").block(Block::default().borders(Borders::ALL)), area);
        return;
    }

    let idx = app.selected_household.min(app.sim.households.len() - 1);
    let h = &app.sim.households[idx];
    let kind = &app.sim.structure.household_types[h.kind.0 as usize].display_name;

    let mut lines = Vec::new();
    lines.push(Line::from(format!("Household #{}  |  Type: {}", h.id.0, kind)));
    lines.push(Line::from(format!("Cash: {:.2}  |  Utility: {:.3}", h.cash, h.utility)));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled("Inventory", Style::default().add_modifier(Modifier::BOLD))));
    for gd in &app.sim.structure.goods {
        let q = h.inventory.get(gd.id);
        if q.abs() > 1e-9 {
            lines.push(Line::from(format!("  {:<18} {:>10.2}", gd.display_name, q)));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("Portfolio", Style::default().add_modifier(Modifier::BOLD))));
    for st in &app.sim.structure.stocks {
        let q = h.portfolio.get(st.id);
        if q.abs() > 1e-9 {
            lines.push(Line::from(format!("  {:<18} {:>10.4}", st.display_name, q)));
        }
    }

    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Details")).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn render_goods(f: &mut Frame, area: Rect, app: &App) {
    let mut rows = Vec::new();
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
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(17),
            Constraint::Percentage(18),
        ],
    )
        .header(Row::new(vec!["Good", "Price", "Buy", "Sell"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title("Goods Market"))
        .column_spacing(1);

    f.render_widget(table, area);
}

fn render_stocks(f: &mut Frame, area: Rect, app: &App) {
    let mut rows = Vec::new();
    for st in &app.sim.structure.stocks {
        let i = st.id.0 as usize;
        rows.push(Row::new(vec![
            st.display_name.clone(),
            format!("{:.3}", app.sim.stock_market.price[i]),
            format!("{:.2}", app.sim.stock_market.demand[i]),
            format!("{:.2}", app.sim.stock_market.supply[i]),
        ]));
    }

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(45),
            Constraint::Percentage(20),
            Constraint::Percentage(17),
            Constraint::Percentage(18),
        ],
    )
        .header(Row::new(vec!["Stock", "Price", "Buy", "Sell"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title("Stock Market"))
        .column_spacing(1);

    f.render_widget(table, area);
}
