mod game;
use game::field;

use std::{io, time::Duration};
use crossterm::{
    cursor,
    execute,
    event::{self, KeyCode, KeyEvent},
    terminal,
};

enum GameError {
    InvalidMove(String),
    OutOfBounds,
}

struct Game {
    field: field::field::Field,
    cursor_row: usize,
    cursor_col: usize,
    player: char,
}

impl Game {
    fn new() -> Game {
        Game {
            field: field::field::Field::new(),
            cursor_row: 0,
            cursor_col: 0,
            player: 'X',
        }
    }

    fn switch_player(&mut self) {
        self.player = match self.player {
            'X' => 'O',
            'O' => 'X',
            _ => 'X',
        };
    }

    fn make_move(&mut self) {
        match self.field.make_move(self.cursor_row, self.cursor_col, self.player) {
            Ok(()) => {
                self.switch_player();
                self.field.clear_console();
                self.field.field_out();
                match self.field.is_over() {
                    Some('D') => println!("DRAW"),
                    Some(winner) => println!("Winner's a: {}", winner),
                    None => {}
                }
            }
            Err(err) => self.print_error(err),
        }
    }

    fn handle_key_event(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc => std::process::exit(0),
            KeyCode::Enter => { self.make_move(); }
            KeyCode::Up if self.cursor_row > 0 => self.cursor_row -= 1,
            KeyCode::Down if self.cursor_row < 2 => self.cursor_row += 1,
            KeyCode::Left if self.cursor_col > 0 => self.cursor_col -= 1,
            KeyCode::Right if self.cursor_col < 2 => self.cursor_col += 1,
            _ => {}
        }
    }

    fn run(&mut self) {
        loop {
            execute!(io::stdout(),cursor::MoveTo(self.cursor_col as u16, self.cursor_row as u16))
                .unwrap();

            if event::poll(Duration::from_millis(100)).unwrap() {
                if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                    self.handle_key_event(code);
                    event::read().unwrap();
                }
            }
        }
    }

    fn print_error(&self, err: GameError) {
        match err {
            GameError::InvalidMove(message) => {
                match self.cursor_row {
                    0 => println!("\n\n\nError: {}", message),
                    1 => println!("\n\nError: {}", message),
                    2 => println!("\nError: {}", message),
                    _ => println!("Error: {}", message),
                }
            }
            GameError::OutOfBounds => { println!("Error: Invalid move - out of bounds"); }
        }
    }
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut game = Game::new();
    game.field.clear_console();
    game.field.field_out();
    game.run();
    terminal::disable_raw_mode().unwrap();
}
