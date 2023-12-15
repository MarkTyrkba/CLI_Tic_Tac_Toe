pub(crate) mod field
{
use crossterm::{cursor, execute, terminal};

pub struct Field {
    pub field: Vec<Vec<char>>,
}

impl Field {
    pub fn new() -> Self {
        Field { field: vec![vec!['_'; 3]; 3] }
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: char) -> Result<(), &'static str>{
        if row < self.field.len() && col < self.field[0].len() {
            if self.field[row][col] == '_' {
                self.field[row][col] = player;
                Ok(())
            } else { Err("Invalid move: cell already occupied") }
        } else { Err("Invalid move: out of bounds") }
    }

    pub fn clear_console(&self) {
        execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    }

    pub fn is_over(&self) -> Option<char> {
        for row in &self.field {
            if row.iter().all(|&cell| cell == 'X')
                { return Some('X'); }
            else if row.iter().all(|&cell| cell == 'O')
                { return Some('O'); }
        }

        for col in 0..self.field[0].len() {
            if self.field.iter().all(|row| row[col] == 'X')
                { return Some('X'); }
            else if self.field.iter().all(|row| row[col] == 'O')
                { return Some('O'); }
        }

        if (0..self.field.len()).all(|i| self.field[i][i] == 'X')
            { return Some('X'); }
        else if (0..self.field.len()).all(|i| self.field[i][i] == 'O')
            { return Some('O'); }

        if (0..self.field.len()).all(|i| self.field[i][self.field.len() - 1 - i] == 'X')
            { return Some('X'); }
        else if (0..self.field.len()).all(|i| self.field[i][self.field.len() - 1 - i] == 'O')
            { return Some('O'); }

        if self.field.iter().any(|row| row.iter().any(|&cell| cell == '_')) { return None; }
        Some('D')
    }


    pub fn field_out(&self) {
        for row in &self.field {
            let row_str: String = row.iter().collect();
            println!("{row_str}");
        }
    }
}
}
