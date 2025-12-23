// src/ui/layout.rs
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct Regions {
    pub header: Rect,
    pub main: Rect,
    pub footer: Rect,
    pub debug: Option<Rect>,
}

pub fn split(area: Rect, show_debug: bool) -> Regions {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(area);

    if show_debug {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(vertical[1]);

        Regions {
            header: vertical[0],
            main: main_chunks[0],
            footer: vertical[2],
            debug: Some(main_chunks[1]),
        }
    } else {
        Regions {
            header: vertical[0],
            main: vertical[1],
            footer: vertical[2],
            debug: None,
        }
    }
}
