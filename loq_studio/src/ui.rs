use std::cmp::min;

use super::app::App;
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, StatefulWidget, Widget, Wrap},
    Frame,
};
pub static FILLER_TEXT:&str="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n";
pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let active_style = Style::default().bg(Color::Gray).fg(Color::Black);
    let normal_style = Style::default().bg(Color::DarkGray).fg(Color::White);
    let header_style = Style::default().bg(Color::Black).fg(Color::Gray);
    let footer_style = Style::default().bg(Color::Black).fg(Color::Gray);

    let mut editor_style = normal_style.clone();
    let mut env_style = normal_style.clone();
    let mut console_style = normal_style.clone();
    match app.currently_active {
        crate::app::AppWindow::Editor => editor_style = active_style,
        crate::app::AppWindow::Env => env_style = active_style,
        crate::app::AppWindow::Console => console_style = active_style,
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let (header_area, footer_area) = (chunks[0], chunks[2]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Fill(1)])
        .split(chunks[1]);

    let env_area = chunks[1];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Fill(1)])
        .split(chunks[0]);

    let (editor_area, console_area) = (chunks[0], chunks[1]);

    let header_block = Block::default().style(header_style);
    let footer_block = Block::default().style(footer_style);

    let editor_block = Block::default()
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
        .border_set(symbols::border::Set {
            top_right: symbols::line::NORMAL.horizontal_down,
            ..symbols::border::PLAIN
        })
        .style(editor_style)
        .title("Editor");

    let env_block = Block::default()
        .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
        .style(env_style)
        .title("Environment");
    let header = Paragraph::new(Text::styled("LoqStudio", header_style))
        .block(header_block)
        .centered();

    let env = Paragraph::new(Text::styled(FILLER_TEXT, env_style))
        .block(env_block)
        .wrap(Wrap { trim: false });
    let footer = Paragraph::new(Text::styled("Footer", footer_style))
        .block(footer_block)
        .centered();
    app.editor.set_style(editor_style);
    app.console.set_style(console_style);
    app.editor.set_block(editor_block);
    frame.render_widget(header, header_area);
    frame.render_widget(&app.console, console_area);
    frame.render_widget(&app.editor, editor_area);
    frame.render_widget(env, env_area);
    frame.render_widget(footer, footer_area);
}
