#![allow(non_snake_case)]

use crate::clock::utils::Time;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

pub fn TimeUI(frame: &mut Frame, time: &Time) {
    let area = frame.area();

    // Minimum required size
    let min_width = 76;
    let min_height = 10;

    // Check if terminal is too small
    if area.width < min_width || area.height < min_height {
        let message = Paragraph::new(vec![
            Line::from(Span::styled("Terminal too small!", Color::Red)),
            Line::from(""),
            Line::from(format!("Minimum size: {}x{}", min_width, min_height)),
            Line::from(format!("Current size: {}x{}", area.width, area.height)),
        ])
        .alignment(Alignment::Center)
        .block(Block::default());

        let centered = area.centered(Constraint::Length(30), Constraint::Length(4));
        frame.render_widget(message, centered);
        return;
    }

    let big_hours = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(time.get_time()).white()])
        .alignment(Alignment::Right)
        .build();

    let am_or_pm = BigText::builder()
        .pixel_size(PixelSize::Octant)
        .lines([Line::from(time.get_am_or_pm()).yellow().bold()])
        .alignment(Alignment::Left)
        .build();

    let content_width = 78;
    let content_height = 10;

    let box_area = area.centered(
        Constraint::Length(content_width),
        Constraint::Length(content_height),
    );

    let lay = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Length(65),
            Constraint::Length(1),
            Constraint::Length(10),
        ])
        .split(box_area);

    // Render Widgets just for testing
    // let b = Block::default().borders(Borders::ALL);

    // frame.render_widget(&b, box_area);
    // frame.render_widget(&b, lay[0]);
    // frame.render_widget(&b, lay[1]);
    // frame.render_widget(&b, lay[2]);

    frame.render_widget(
        big_hours,
        lay[1].inner(Margin {
            horizontal: 0,
            vertical: 3,
        }),
    );

    frame.render_widget(
        am_or_pm,
        lay[3].inner(Margin {
            horizontal: 1,
            vertical: 4,
        }),
    );
}
