use std::{env::args, fs::read, io::stdout, process::exit};

use chrono::Local;

use codepage_437::{BorrowFromCp437, CP437_CONTROL};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame, Terminal,
};

const TAB_WIDTH: usize = 8;

struct Addresses<'a> {
    path: String,
    addresses: Vec<Paragraph<'a>>,
    headers: Vec<ListItem<'a>>,
    is_list_mode: bool,
    state: ListState,
}

impl<'a> Addresses<'a> {
    pub fn new(path: String, content: &'a str) -> Addresses<'a> {
        // content can be zero terminated and ends in a final LF+LF to be trimmed, before we split on that pattern
        let block = Block::new()
            .borders(Borders::TOP)
            .padding(Padding::horizontal(1));
        let mut headers: Vec<ListItem> = vec![];
        let addresses: Vec<Paragraph> = content
            .trim_matches('\0')
            .trim()
            .split("\n\n")
            .map(|page| {
                let mut header = String::new();
                let mut address = String::new();
                let mut lines = page.split('\n');

                for column in lines.next().unwrap().split('\t') {
                    pad_column_into(column, &mut header);
                }
                header.truncate(header.trim_end().len());
                headers.push(ListItem::new(header));

                for line in lines {
                    for column in line.split('\t') {
                        pad_column_into(column, &mut address);
                    }
                    address.truncate(address.trim_end().len());
                    address.push('\n');
                }
                address.pop(); // trailing new line
                Paragraph::new(Text::from(address)).block(block.clone())
            })
            .collect();
        let state = ListState::default();
        Addresses {
            path,
            addresses,
            headers,
            is_list_mode: true,
            state,
        }
    }

    fn draw_block(&mut self, f: &mut Frame) -> Rect {
        let area = f.area();
        let block = Block::default()
            .title(format!("═ {} ", self.path))
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        let total = self.headers.len();
        let counter = if self.is_list_mode {
            format!(" #{total} ")
        } else {
            let index = match self.state.selected() {
                Some(i) => i + 1,
                None => 1,
            };
            format!(" {index} of #{total} ")
        };
        let counter_len = counter.len() as u16;
        let counter_size = Rect::new(
            area.x + area.width - counter_len - 2,
            area.y,
            counter_len,
            1,
        );
        let counter_box = Paragraph::new(Span::raw(counter));

        let datetime = Local::now().format(" %a %d %b %y %R ").to_string();
        let datetime_size = Rect::new(
            area.x + 2,
            area.y + area.height - 1,
            datetime.len() as u16,
            1,
        );
        let datetime_box = Paragraph::new(Span::raw(datetime));

        f.render_widget(block, area);
        f.render_widget(counter_box, counter_size);
        f.render_widget(datetime_box, datetime_size);

        // inner size of block, with 1 char margin on the left
        Rect::new(area.x + 2, area.y + 1, area.width - 3, area.height - 2)
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let inner_area = self.draw_block(f);
        if self.is_list_mode {
            let items = List::new(self.headers.clone())
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White));
            f.render_stateful_widget(items, inner_area, &mut self.state);
        } else {
            let index = match self.state.selected() {
                Some(i) => i,
                None => {
                    self.state.select(Some(0));
                    0
                }
            };
            let header = List::new([self.headers[index].clone()]);
            f.render_widget(header, inner_area.rows().next().unwrap());

            let remaining_area = Rect::new(
                inner_area.x - 1,
                inner_area.y + 1,
                inner_area.width + 1,
                inner_area.height - 1,
            );
            f.render_widget(self.addresses[index].clone(), remaining_area);
        }
    }

    pub fn is_list_mode(&mut self) -> bool {
        self.is_list_mode
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

    pub fn toggle_mode(&mut self) {
        self.is_list_mode = !self.is_list_mode;
    }
}

// appends column onto destination, padded with spaces
fn pad_column_into(column: &str, destination: &mut String) {
    destination.push_str(column);
    let mut pad = TAB_WIDTH - column.chars().count() % TAB_WIDTH;
    if pad == 0 {
        pad = TAB_WIDTH; // add tab width
    }
    while pad > 0 {
        destination.push(' ');
        pad -= 1;
    }
}

fn main() {
    // stores parsed string slices of addresses and descriptions, tracks
    // position in the list, draws frame
    let mut addresses;
    // parsed string, borrowed to addresses for tracking the offsets into it for
    // the string slices of addresses and descriptions
    let utf8_content;

    // closure to drop parsing state is when done
    {
        let path = match args().nth(1) {
            Some(path) => path,
            None => {
                eprintln!("Missing argument: no path to file given");
                exit(255);
            }
        };

        let dos_content = match read(&path) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Error reading file {}", path);
                eprintln!("{}", error);
                exit(254);
            }
        };
        utf8_content =
            String::borrow_from_cp437(&dos_content, &CP437_CONTROL).replace("\r\n", "\n");
        addresses = Addresses::new(path, &utf8_content);
    }

    enable_raw_mode().unwrap(); // prevent key presses reaching stdout
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| addresses.draw(f)).unwrap();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if addresses.is_list_mode() {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => addresses.toggle_mode(),
                    KeyCode::Down => addresses.next(),
                    KeyCode::Up => addresses.previous(),
                    _ => continue,
                }
            } else {
                match key.code {
                    KeyCode::Esc => addresses.toggle_mode(),
                    KeyCode::PageDown => addresses.next(),
                    KeyCode::PageUp => addresses.previous(),
                    _ => continue,
                }
            }
            terminal
                .draw(|f| {
                    addresses.draw(f);
                })
                .unwrap();
        }
    }

    // cleanup
    terminal.clear().unwrap();
    disable_raw_mode().unwrap();
}
