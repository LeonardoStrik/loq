use super::app::App;
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
static FILLER_TEXT:&str="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
pub fn draw_ui(frame: &mut Frame, app: &App) {
    let active_style = Style::default().bg(Color::Gray).fg(Color::Black);
    let normal_style = Style::default().bg(Color::DarkGray).fg(Color::White);
    let header_style = Style::default().bg(Color::Black).fg(Color::Gray);
    let footer_style = Style::default().bg(Color::Black).fg(Color::Gray);

    let mut editor_style = normal_style.clone();
    let mut env_style = normal_style.clone();
    let mut console_style = normal_style.clone();
    match app.currently_active {
        super::app::AppWindow::Editor => editor_style = active_style,
        super::app::AppWindow::Env => env_style = active_style,
        super::app::AppWindow::Console => console_style = active_style,
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

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(2)])
        .split(console_area);

    let (console_contents_area, console_input_area) = (chunks[0], chunks[1]);
    let header_block = Block::default().style(header_style);
    let footer_block = Block::default().style(footer_style);

    let editor_block = Block::default()
        .borders(Borders::all())
        .style(editor_style)
        .title("Editor");

    let console_contents_block = Block::default()
        .borders(Borders::all())
        .title("Console")
        .style(console_style);
    let console_input_block = Block::default().style(console_style);
    let env_block = Block::default()
        .borders(Borders::all())
        .style(env_style)
        .title("Environment");
    let header = Paragraph::new(Text::styled("LoqStudio", header_style))
        .block(header_block)
        .centered();
    let editor = Paragraph::new(Text::styled(FILLER_TEXT, editor_style))
        .block(editor_block)
        .wrap(Wrap { trim: false });
    let console_contents = Paragraph::new(Text::styled(app.contents_to_text(), console_style))
        .block(console_contents_block)
        .wrap(Wrap { trim: false });
    let console_input = Paragraph::new(Text::styled(
        app.input_buffer.iter().collect::<String>().to_owned(),
        console_style,
    ))
    .block(console_input_block)
    .wrap(Wrap { trim: false });
    let env = Paragraph::new(Text::styled(FILLER_TEXT, env_style))
        .block(env_block)
        .wrap(Wrap { trim: false });
    let footer = Paragraph::new(Text::styled("Footer", footer_style))
        .block(footer_block)
        .centered();
    frame.render_widget(header, header_area);
    frame.render_widget(editor, editor_area);
    frame.render_widget(console_contents, console_contents_area);
    frame.render_widget(console_input, console_input_area);
    frame.render_widget(env, env_area);
    frame.render_widget(footer, footer_area);
}
