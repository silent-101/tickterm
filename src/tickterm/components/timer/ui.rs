#![allow(non_snake_case)]

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Margin},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

use crate::tickterm::components::{
    stopwatch::state::BtnMode,
    timer::{
        state::{TimerCtx, TimerMode, TimerSagemnt},
        utils::TimerUtils,
    },
};

pub fn TimerUI(
    frame: &mut Frame,
    timer_ctx: &TimerCtx,
    btn_focus: &BtnMode,
    timer_utils: &TimerUtils,
) {
    let area = frame.area();

    let timer_segmant = {
        match (
            timer_ctx.get_timer_sagment(),
            timer_ctx.get_timer_mode() == &TimerMode::SetterMode,
        ) {
            (TimerSagemnt::Hour, true) => 'H',
            (TimerSagemnt::Min, true) => 'M',
            (TimerSagemnt::Sec, true) => 'S',
            (_, false) => '?',
        }
    };
    let timer_hour = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(timer_utils.get_hour()).fg(
            if timer_segmant == 'H' && timer_ctx.get_timer_mode() == &TimerMode::SetterMode {
                Color::Rgb(255, 215, 0)
            } else if timer_ctx.get_timer_mode() == &TimerMode::SetterMode {
                Color::Rgb(200, 200, 200)
            } else {
                Color::White
            },
        )])
        .alignment(Alignment::Right)
        .build();

    let timer_min = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(timer_utils.get_min()).fg(
            if timer_segmant == 'M' && timer_ctx.get_timer_mode() == &TimerMode::SetterMode {
                Color::Rgb(255, 215, 0)
            } else if timer_ctx.get_timer_mode() == &TimerMode::SetterMode {
                Color::Rgb(200, 200, 200)
            } else {
                Color::White
            },
        )])
        .alignment(Alignment::Right)
        .build();

    let timer_sec = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(timer_utils.get_sec()).fg(
            if timer_segmant == 'S' && timer_ctx.get_timer_mode() == &TimerMode::SetterMode {
                Color::Rgb(255, 215, 0)
            } else if timer_ctx.get_timer_mode() == &TimerMode::SetterMode {
                Color::Rgb(200, 200, 200)
            } else {
                Color::White
            },
        )])
        .alignment(Alignment::Right)
        .build();
    let timer_sep1 = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(":").white()])
        .alignment(Alignment::Right)
        .build();

    let timer_sep2 = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines([Line::from(":").white()])
        .alignment(Alignment::Right)
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
        .constraints(vec![Constraint::Length(5), Constraint::Length(65)])
        .split(lay_box[0]);

    let timer_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Fill(1),
            Constraint::Length(6),
            Constraint::Fill(1),
        ])
        .split(lay[1].inner(Margin {
            horizontal: 0,
            vertical: 1,
        }));

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

    let has_value = timer_utils.has_value();
    let start_btn = Paragraph::new("\u{25B6} Start")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .centered()
        .fg(if !has_value {
            Color::Rgb(50, 50, 50)
        } else if let BtnMode::Start = btn_focus {
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

    let set_btn = Paragraph::new("\u{1F836} Set")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .centered()
        .fg(if let BtnMode::Set = btn_focus {
            Color::Rgb(135, 206, 250)
        } else {
            Color::Rgb(70, 70, 70)
        })
        .bold();

    if timer_ctx.get_timer_mode() != &TimerMode::SetterMode {
        // frame.render_widget(&b, box_area);
        if timer_ctx.get_timer_mode() == &TimerMode::RunningMode {
            frame.render_widget(&stop_btn, buttons[0]);
        } else if !has_value {
            frame.render_widget(&set_btn, buttons[0]);
        } else {
            frame.render_widget(&start_btn, buttons[0]);
        }
        frame.render_widget(&reset_btn, buttons[2]);
    }
    frame.render_widget(timer_hour, timer_box[0]);
    frame.render_widget(timer_sep1, timer_box[1]);
    frame.render_widget(timer_min, timer_box[2]);
    frame.render_widget(timer_sep2, timer_box[3]);
    frame.render_widget(timer_sec, timer_box[4]);
}
