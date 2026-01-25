#![allow(non_snake_case)]

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

use crate::tickterm::components::stopwatch::{state::BtnMode, utils::StopWatch};

pub fn StopWatchUI(frame: &mut Frame, watch: &StopWatch, btn_focus: &BtnMode) {
    let area = frame.area();

    let timer = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(watch.get_time()).white()])
        .alignment(Alignment::Right)
        .build();

    let ms = BigText::builder()
        .pixel_size(PixelSize::Octant)
        .lines([Line::from(watch.get_ms())
            .bold()
            .style(Style::default().fg(Color::Rgb(255, 215, 0)))])
        .alignment(Alignment::Left)
        .build();

    let content_width = 78;
    let content_height = 9;

    let box_area = area.centered(
        Constraint::Length(content_width),
        Constraint::Length(content_height),
    );

    let lay_box = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(10), Constraint::Length(3)])
        .split(box_area);

    let lay = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Length(65),
            Constraint::Length(1),
            Constraint::Length(10),
        ])
        .split(lay_box[0]);

    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(25),
            Constraint::Length(1),
            Constraint::Length(15),
        ])
        .split(
            lay_box[1]
                .centered(Constraint::Length(40), Constraint::Fill(1))
                .inner(Margin {
                    horizontal: 5,
                    vertical: 0,
                }),
        );

    // Render Widgets just for testing
    // let b = Block::default().borders(Borders::ALL);
    let start_btn = Paragraph::new("\u{25B6} Start")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .centered()
        .fg(if let BtnMode::Start = btn_focus {
            Color::Rgb(154, 205, 50)
        } else {
            Color::Rgb(70, 70, 70)
        })
        .bold();

    let reset_btn = Paragraph::new("\u{25CF} Reset")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .centered()
        .fg(if let BtnMode::Reset = btn_focus {
            Color::Rgb(147, 112, 219)
        } else {
            Color::Rgb(70, 70, 70)
        })
        .bold();

    let stop_btn = Paragraph::new("\u{25A0} Stop")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .centered()
        .fg(if let BtnMode::Stop = btn_focus {
            Color::Rgb(255, 127, 80)
        } else {
            Color::Rgb(70, 70, 70)
        })
        .bold();

    // frame.render_widget(&b, box_area);
    if watch.is_running() {
        frame.render_widget(&stop_btn, buttons[0]);
    } else {
        frame.render_widget(&start_btn, buttons[0]);
    }
    frame.render_widget(&reset_btn, buttons[2]);

    frame.render_widget(
        timer,
        lay[1].inner(Margin {
            horizontal: 0,
            vertical: 1,
        }),
    );
    frame.render_widget(
        ms,
        lay[3].inner(Margin {
            horizontal: 1,
            vertical: 2,
        }),
    );
}
