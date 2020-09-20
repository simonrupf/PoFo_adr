use std::{
    env::args,
    fs,
    io::stdout,
    process::exit,
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{
    backend::{Backend, CrosstermBackend},
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    Terminal,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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
        let addresses: Vec<&str> = content.trim_matches('\0').trim().split("\r\n\r\n").collect();
        let headers: Vec<ListItem> = addresses.iter().map(|address| {
            ListItem::new(
                address.split("\r\n").nth(0).unwrap()
            )
        }).collect();
        let state = ListState::default();
        Adresses {
            path,
            addresses,
            headers,
            state,
        }
    }

    pub fn draw_list<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let block = Block::default()
            .title(format!(" {} ", self.path))
            .borders(Borders::ALL);
        let items = List::new(self.headers.clone())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White));
        let counter = format!(" #{} ", self.headers.len());
        let counter_len = counter.len() as u16;
        let counter_box = Paragraph::new(Span::raw(counter));

        let inner_size = Rect::new(size.x + 2, size.y + 1, size.width - 3, size.height - 2);
        let counter_size = Rect::new(size.x + size.width - counter_len - 2, size.y, counter_len, 1);

        f.render_widget(block, size);
        f.render_stateful_widget(items, inner_size, &mut self.state);
        f.render_widget(counter_box, counter_size);
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
    enable_raw_mode().unwrap(); // prevent key presses reaching stdout
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| addresses.draw_list(f)).unwrap();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Down => addresses.next(),
                KeyCode::Up => addresses.previous(),
                _ => {}
            }
            terminal.draw(|f| addresses.draw_list(f)).unwrap();
        }
    }

    // cleanup
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
}
