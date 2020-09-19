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
    text::{Text},
    Terminal,
    widgets::{Block, Borders, Paragraph, Wrap},
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

    enable_raw_mode().unwrap(); // prevent key presses reaching stdout
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(path)
            .borders(Borders::ALL);
        let text = Text::from(&content[..]);
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        f.render_widget(paragraph, size);
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
