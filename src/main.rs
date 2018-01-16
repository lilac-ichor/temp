mod editor;

extern crate termion;

use termion::event::{Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use editor::{Editor, Direction};

use std::io::{Write, stdout, stdin};
use std::fmt;

fn main() {

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut editor = Editor::new();
    
    let mut events = stdin.events();

    loop {
        if let Some(Ok(event)) = events.next() {
            if let termion::event::Event::Key(key) = event {
                match key {
                    Key::Char('q') => break,
                    Key::Char('\n') => {
                        editor.insert("\n");
                        editor.move_cursor(Direction::Right);
                    },
                    Key::Char(c) => {
                        editor.insert(&c.to_string());
                        editor.move_cursor(Direction::Right);
                    },
                    Key::Up => editor.move_cursor(Direction::Up),
                    Key::Down => editor.move_cursor(Direction::Down),
                    Key::Left => editor.move_cursor(Direction::Left),
                    Key::Right => editor.move_cursor(Direction::Right),
                    Key::Backspace => {
                        editor.remove_char();
                        editor.move_cursor(Direction::Left);
                    },
                    _ => (),
                }
                print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                for line in editor.buffer.lines() {
                    print!("{}\r", line);
                }
                print!("\r\n{:?}\r\n", editor.cursor);
                for line in editor.buffer.lines() {
                    print!("{:?}\r\n", line);
                }
                print!("{:?}\r\n", editor.buffer);
                print!("{}\r\n", editor.buffer.len_lines());
                print!("{}", termion::cursor::Goto((editor.cursor.x + 1) as u16, (editor.cursor.y + 1) as u16));
                stdout.flush().unwrap();
            }
        }
    }

}
