use std::env::args;
use std::fs;
use std::process::exit;

extern crate pancurses;
use pancurses::{initscr, endwin, Input, noecho, Window};
use Input::*;

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
    println!("File {}:", path);
    println!("{}", content);

    let window = initscr();
    window.printw("File {} - Press Escape to quit\n");
    window.addstr(content);
    window.draw_box(0, 0);
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(key) if [KeyUp, KeyDown, KeyLeft, KeyRight].contains(&key) => handle_cursor_key(window, key),
            Some(Character('\x1b')) => break, // escape key
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => ()
        }
    }
    endwin();
}

fn handle_cursor_key(&window: Window, key: Input) {
    let x = window.get_cur_x();
    let y = window.get_cur_y();
    let mut updated_x = x;
    let mut updated_y = y;

    match key {
        KeyUp => {
            updated_y = y - 1;
        },
        KeyDown => {
            updated_y = y + 1;
        },
        KeyLeft => {
            updated_x = x - 1;
        },
        KeyRight => {
            updated_x = x + 1;
        },
        _ => ()
    }
    window.mvwin(updated_y, updated_x);
    window.mv(updated_y, updated_x);
    window.refresh();
}
