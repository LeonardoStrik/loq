use super::ui::FILLER_TEXT;
use std::{
    cmp::{max, min},
    io::{stderr, Write},
};

use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    symbols,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
#[derive(Default)]
struct Cursor {
    line: usize,
    ch: usize,
}

pub struct Console {
    input_buffer: Vec<Vec<char>>,
    content: Vec<String>,
    cursor: Cursor,

    style: Style,
}
impl Console {
    pub fn new() -> Self {
        Console {
            input_buffer: vec![Vec::new()],
            content: Vec::new(),
            cursor: Cursor::default(),
            style: Style::default(),
        }
    }
    fn to_rows(&self, line_length: usize) -> Vec<String> {
        let mut out = vec![];
        for line in self.input_buffer.iter() {
            let mut start_idx = 0;
            let mut end_idx = min(line.len(), line_length);
            while start_idx < line.len() {
                out.push(line[start_idx..end_idx].iter().collect());
                start_idx = end_idx;
                end_idx = min(line.len(), start_idx + line_length)
            }
        }
        out
    }
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    fn get_current_line_len(&self) -> usize {
        self.input_buffer[self.cursor.line].len()
    }
    // TODO: line wrapping and stuff
    fn increment_cursor(&mut self) {
        if self.get_current_line_len() > self.cursor.ch {
            self.cursor.ch += 1;
        }
    }
    fn decrement_cursor(&mut self) {
        self.cursor.ch = self.cursor.ch.saturating_sub(1);
    }
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            crossterm::event::KeyCode::Enter => {
                if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.content.push(
                        self.input_buffer
                            .iter()
                            .map(|line| line.iter().collect::<String>())
                            .collect::<Vec<String>>()
                            .join(" "),
                    );
                    self.input_buffer = vec![Vec::new()];
                    self.cursor = Cursor::default();
                }
            }

            crossterm::event::KeyCode::Left => self.decrement_cursor(),
            crossterm::event::KeyCode::Right => self.increment_cursor(),
            crossterm::event::KeyCode::Up => (),
            crossterm::event::KeyCode::Down => (),
            crossterm::event::KeyCode::Home => (),
            crossterm::event::KeyCode::End => (),
            crossterm::event::KeyCode::PageUp => (),
            crossterm::event::KeyCode::PageDown => (),
            crossterm::event::KeyCode::Tab => (),
            crossterm::event::KeyCode::BackTab => (),
            crossterm::event::KeyCode::Backspace => {
                if (self.get_current_line_len() >= self.cursor.ch) && self.cursor.ch > 0 {
                    // Fix double non-zero checks by returning boolean?
                    self.decrement_cursor();
                    let _ = self.input_buffer[self.cursor.line].remove(self.cursor.ch);
                }
            }
            crossterm::event::KeyCode::Delete => {
                if self.get_current_line_len() >= self.cursor.ch {
                    let _ = self.input_buffer[self.cursor.line].remove(self.cursor.ch);
                }
            }
            crossterm::event::KeyCode::Insert => (),
            crossterm::event::KeyCode::F(_) => (),
            crossterm::event::KeyCode::Char(mut ch) => {
                if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    ch = ch.to_ascii_uppercase() //TODO: presumably this only works on ASCII text, and utf-8 is a lot more fun
                }
                self.input_buffer[self.cursor.line].insert(self.cursor.ch, ch);
                self.increment_cursor();
            }
            crossterm::event::KeyCode::Null => (),
            crossterm::event::KeyCode::Esc => (),
            crossterm::event::KeyCode::CapsLock => (),
            crossterm::event::KeyCode::ScrollLock => (),
            crossterm::event::KeyCode::NumLock => (),
            crossterm::event::KeyCode::PrintScreen => (),
            crossterm::event::KeyCode::Pause => (),
            crossterm::event::KeyCode::Menu => (),
            crossterm::event::KeyCode::KeypadBegin => (),
            crossterm::event::KeyCode::Media(media_key_code) => (),
            crossterm::event::KeyCode::Modifier(modifier_key_code) => (),
        }
    }
}
impl Widget for &Console {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let console_contents_block = Block::default()
            .borders(Borders::all())
            .border_set(symbols::border::Set {
                top_left: symbols::line::NORMAL.vertical_right,
                top_right: symbols::line::NORMAL.vertical_left,
                bottom_left: symbols::line::NORMAL.vertical_right,
                bottom_right: symbols::line::NORMAL.vertical_left,
                ..symbols::border::PLAIN
            })
            .title("Console")
            .style(self.style);
        let console_input_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_set(symbols::border::Set {
                bottom_right: symbols::line::NORMAL.horizontal_up,
                ..symbols::border::PLAIN
            })
            .style(self.style);
        let input_inner = console_input_block.inner(area);
        let rows = self.to_rows(input_inner.width as usize);
        let chunks = Layout::default()
            .constraints([
                Constraint::Min(3),
                Constraint::Length(max(rows.len() as u16 + 1, 2)),
            ])
            .split(area);
        let (console_contents_area, console_input_area) = (chunks[0], chunks[1]);

        let console_contents = Paragraph::new(Text::styled(self.content.join("\n"), self.style))
            .block(console_contents_block)
            .wrap(Wrap { trim: false });
        console_contents.render(console_contents_area, buf);
        console_input_block.clone().render(console_input_area, buf);
        let input_inner = console_input_block.inner(console_input_area);
        for (i, line) in rows.iter().enumerate() {
            buf.set_string(
                input_inner.left(),
                input_inner.top() + i as u16,
                line,
                self.style,
            );
        }
        buf.set_style(
            Rect {
                x: input_inner.left() + self.cursor.ch as u16,
                y: input_inner.top() + self.cursor.line as u16,
                width: 1,
                height: 1,
            },
            Style::default().bg(Color::Black).fg(Color::White),
        );
        // TODO: all this need to take all the proper line wrapping into account
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let cons = Console::new();
        for row in cons.to_rows(6) {
            println!("{}", row)
        }
    }
}
