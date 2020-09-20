use std::{env::args, fs, io::stdout, process::exit};

use chrono::Local;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

struct Adresses<'a> {
    path: String,
    addresses: Vec<&'a str>,
    headers: Vec<ListItem<'a>>,
    state: ListState,
}

impl<'a> Adresses<'a> {
    pub fn new(path: String, content: &'a String) -> Adresses<'a> {
        // content can be zero terminated and ends in a final CR+LF+CR+LF to be trimmed, before we split on that pattern
        let addresses: Vec<&str> = content
            .trim_matches('\0')
            .trim()
            .split("\r\n\r\n")
            .collect();
        let headers: Vec<ListItem> = addresses
            .iter()
            .map(|address| ListItem::new(address.split("\r\n").nth(0).unwrap()))
            .collect();
        let state = ListState::default();
        Adresses {
            path,
            addresses,
            headers,
            state,
        }
    }

    fn draw_block<B: Backend>(&mut self, f: &mut Frame<B>) -> Rect {
        let size = f.size();
        let block = Block::default()
            .title(format!(" {} ", self.path))
            .borders(Borders::ALL);

        let counter = format!(" #{} ", self.headers.len());
        let counter_len = counter.len() as u16;
        let counter_size = Rect::new(
            size.x + size.width - counter_len - 2,
            size.y,
            counter_len,
            1,
        );
        let counter_box = Paragraph::new(Span::raw(counter));

        let datetime = format!(" {} ", Local::now().format("%a %d %b %y %R"));
        let datetime_size = Rect::new(
            size.x + 2,
            size.y + size.height - 1,
            datetime.len() as u16,
            1,
        );
        let datetime_box = Paragraph::new(Span::raw(datetime));

        f.render_widget(block, size);
        f.render_widget(counter_box, counter_size);
        f.render_widget(datetime_box, datetime_size);

        // inner size of block, with 1 char margin on the left
        Rect::new(size.x + 2, size.y + 1, size.width - 3, size.height - 2)
    }

    pub fn draw_list<B: Backend>(&mut self, f: &mut Frame<B>) {
        let items = List::new(self.headers.clone())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White));
        let inner_size = self.draw_block(f);
        f.render_stateful_widget(items, inner_size, &mut self.state);
    }

    pub fn draw_selected<B: Backend>(&mut self, f: &mut Frame<B>) {
        let address = Paragraph::new(Text::from(
            self.addresses[match self.state.selected() {
                Some(i) => i,
                None => {
                    self.state.select(Some(0));
                    0
                }
            }],
        ));
        let inner_size = self.draw_block(f);
        f.render_widget(address, inner_size);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.headers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.headers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() {
    let path = match args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Missing argument: no path to file given");
            exit(255);
        }
    };

    let content = match fs::read_to_string(path.clone()) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file {}", path);
            eprintln!("{}", error);
            exit(254);
        }
    };
    let mut addresses = Adresses::new(path, &content);
    let mut is_list_mode = true;

    enable_raw_mode().unwrap(); // prevent key presses reaching stdout
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| addresses.draw_list(f)).unwrap();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if is_list_mode {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => is_list_mode = !is_list_mode,
                    KeyCode::Down => addresses.next(),
                    KeyCode::Up => addresses.previous(),
                    _ => continue,
                }
            } else {
                match key.code {
                    KeyCode::Esc => is_list_mode = !is_list_mode,
                    KeyCode::PageDown => addresses.next(),
                    KeyCode::PageUp => addresses.previous(),
                    _ => continue,
                }
            }
            terminal
                .draw(|f| {
                    // mode might have changed, check it again
                    if is_list_mode {
                        addresses.draw_list(f);
                    } else {
                        addresses.draw_selected(f);
                    }
                })
                .unwrap();
        }
    }

    // cleanup
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
}
