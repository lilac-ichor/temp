extern crate ropey;

use self::ropey::Rope;

use termion::event::{Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin};
use std::fmt;

use std::cmp;


type Buffer = Rope;

pub struct Editor {
    pub cursor: Position,
    pub buffer: Buffer
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Editor {
    pub fn new() -> Editor {
        Editor { cursor: Position::new(0, 0), buffer: Buffer::new() }
    }

    // Safe
    pub fn move_cursor(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.cursor.y = self.cursor.y.saturating_sub(1);
            },
            Direction::Down => {
                let y = self.cursor.y.saturating_add(1);
                self.cursor.y = cmp::min(y, self.buffer.len_lines() - 1);
            },
            Direction::Left => {
                if self.cursor.x == 0 {
                    self.move_cursor(Direction::Up);
                } else {
                    self.cursor.x -= 1;
                }
            },
            Direction::Right => {
                let x = self.cursor.x.saturating_add(1);
                let max_x = self.line_length(self.cursor.y);
                if x > max_x {
                    self.move_cursor(Direction::Down);
                    self.cursor.x = 0;
                } else {
                    self.cursor.x = x;
                }
            }
        }
    }

    fn line_length(&self, line_idx: usize) -> usize {
        let mut raw_length = self.buffer.line(line_idx).len_chars();

        if line_idx + 1 == self.buffer.len_lines() {
            raw_length;
        } else {
            raw_length -= 1;
        }

        raw_length
    }

    // Safe -- rework to be smart
    pub fn set_cursor(&mut self, position: &Position) {
        self.cursor.x = position.x;
        self.cursor.y = position.y;
    }

    // Safe
    pub fn insert(&mut self, text: &str) -> Position {
        let char_idx = self.cursor_to_char_idx();
        self.buffer.insert(char_idx, text);

        self.char_idx_to_position(char_idx + text.len())
    }

    pub fn remove_char(&mut self) {
        let char_idx = self.cursor_to_char_idx();
        self.buffer.remove(char_idx-1..char_idx);
    }

    // Safe?
    pub fn cursor_to_char_idx(&self) -> usize {
        self.position_to_char_idx(&self.cursor)
    }

    // Safe? might be missing minus one 
    pub fn position_to_char_idx(&self, position: &Position) -> usize {
        let y = cmp::min(position.y, self.buffer.len_lines());
        let min_char_idx = self.buffer.line_to_char(y);
        let max_char_idx = self.buffer.line_to_char(y + 1);

        cmp::min(min_char_idx + self.cursor.x, max_char_idx)
    }

    // Safe
    pub fn char_idx_to_position(&self, char_idx: usize) -> Position {
        let char_idx = cmp::min(char_idx, self.buffer.len_lines());
        let y = self.buffer.char_to_line(char_idx);
        let min_char_idx = self.buffer.line_to_char(y);

        Position::new(char_idx - min_char_idx, y)
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position{ x, y }
    }
}

