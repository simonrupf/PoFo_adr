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
    backend::CrosstermBackend,
    layout::Rect,
    text::{Span, Spans, Text},
    Terminal,
    widgets::{Block, Borders, Paragraph},
};

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
    let addresses: Vec<&str> = content.split("\r\n\r\n").collect();
    let headers: Vec<Spans> = addresses.iter().map(|address| {
        Spans::from(
            Span::raw(
                address.split("\r\n").nth(0).unwrap().to_string()
            )
        )
    }).collect();

    enable_raw_mode().unwrap(); // prevent key presses reaching stdout
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(format!(" {} ", path))
            .borders(Borders::ALL);
        let content_box = Paragraph::new(Text::from(headers));
        let counter = format!(" #{} ", addresses.len());
        let counter_len = counter.len() as u16;
        let counter_box = Paragraph::new(Text::from(vec![Spans::from(Span::raw(counter))]));

        let inner_size = Rect::new(size.x + 2, size.y + 1, size.width - 3, size.height - 2);
        let counter_size = Rect::new(size.x + size.width - counter_len - 2, size.y, counter_len, 1);

        f.render_widget(block, size);
        f.render_widget(content_box, inner_size);
        f.render_widget(counter_box, counter_size);
    }).unwrap();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.code == KeyCode::Esc {
                break;
            }
        }
    }

    // cleanup
    disable_raw_mode().unwrap();
}
