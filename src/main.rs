use std::fmt;
use std::io::{self, stdout, Write};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::{Hide, Show, MoveTo},
    event::{read, Event, KeyCode},
    style::Print,
};
use rand::prelude::SliceRandom;
use rand::Rng;

const SIZE: usize = 4;

#[derive(Clone, Copy, PartialEq)]
struct Tile(Option<u32>);

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(value) => write!(f, "{:4}", value),
            None => write!(f, "    "),
        }
    }
}

struct Board([[Tile; SIZE]; SIZE]);

impl Board {
    fn new() -> Self {
        let mut board = Board([[Tile(None); SIZE]; SIZE]);
        board.add_random_tile();
        board.add_random_tile();
        board
    }

    fn add_random_tile(&mut self) {
        let mut rng = rand::thread_rng();
        let empty_tiles: Vec<(usize, usize)> = self.0.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate()
                .filter(|(_, tile)| tile.0.is_none())
                .map(move |(j, _)| (i, j)))
            .collect();

        if let Some(&(i, j)) = empty_tiles.choose(&mut rng) {
            self.0[i][j] = Tile(Some(if rng.gen_bool(0.9) { 2 } else { 4 }));
        }
    }

    fn move_tiles(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => self.move_up(),
            KeyCode::Left => self.move_left(),
            KeyCode::Down => self.move_down(),
            KeyCode::Right => self.move_right(),
            _ => false,
        }
    }

    fn move_up(&mut self) -> bool {
        let mut moved = false;
        for col in 0..SIZE {
            let mut merged = false;
            for row in 1..SIZE {
                if let Some(value) = self.0[row][col].0 {
                    let mut new_row = row;
                    while new_row > 0 && self.0[new_row - 1][col].0.is_none() {
                        new_row -= 1;
                    }
                    if new_row > 0 && !merged && self.0[new_row - 1][col].0 == Some(value) {
                        self.0[new_row - 1][col].0 = Some(value * 2);
                        self.0[row][col].0 = None;
                        merged = true;
                        moved = true;
                    } else if new_row != row {
                        self.0[new_row][col].0 = Some(value);
                        self.0[row][col].0 = None;
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for row in 0..SIZE {
            let mut merged = false;
            for col in 1..SIZE {
                if let Some(value) = self.0[row][col].0 {
                    let mut new_col = col;
                    while new_col > 0 && self.0[row][new_col - 1].0.is_none() {
                        new_col -= 1;
                    }
                    if new_col > 0 && !merged && self.0[row][new_col - 1].0 == Some(value) {
                        self.0[row][new_col - 1].0 = Some(value * 2);
                        self.0[row][col].0 = None;
                        merged = true;
                        moved = true;
                    } else if new_col != col {
                        self.0[row][new_col].0 = Some(value);
                        self.0[row][col].0 = None;
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    fn move_down(&mut self) -> bool {
        let mut moved = false;
        for col in 0..SIZE {
            let mut merged = false;
            for row in (0..SIZE - 1).rev() {
                if let Some(value) = self.0[row][col].0 {
                    let mut new_row = row;
                    while new_row < SIZE - 1 && self.0[new_row + 1][col].0.is_none() {
                        new_row += 1;
                    }
                    if new_row < SIZE - 1 && !merged && self.0[new_row + 1][col].0 == Some(value) {
                        self.0[new_row + 1][col].0 = Some(value * 2);
                        self.0[row][col].0 = None;
                        merged = true;
                        moved = true;
                    } else if new_row != row {
                        self.0[new_row][col].0 = Some(value);
                        self.0[row][col].0 = None;
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        let mut moved = false;
        for row in 0..SIZE {
            let mut merged = false;
            for col in (0..SIZE - 1).rev() {
                if let Some(value) = self.0[row][col].0 {
                    let mut new_col = col;
                    while new_col < SIZE - 1 && self.0[row][new_col + 1].0.is_none() {
                        new_col += 1;
                    }
                    if new_col < SIZE - 1 && !merged && self.0[row][new_col + 1].0 == Some(value) {
                        self.0[row][new_col + 1].0 = Some(value * 2);
                        self.0[row][col].0 = None;
                        merged = true;
                        moved = true;
                    } else if new_col != col {
                        self.0[row][new_col].0 = Some(value);
                        self.0[row][col].0 = None;
                        moved = true;
                    }
                }
            }
        }
        moved
    }

    fn is_game_over(&self) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.0[row][col].0.is_none() {
                    return false;
                }
                if (row < SIZE - 1 && self.0[row][col] == self.0[row + 1][col])
                    || (col < SIZE - 1 && self.0[row][col] == self.0[row][col + 1])
                {
                    return false;
                }
            }
        }
        true
    }

    fn print(&self) -> io::Result<()> {
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print("2048 Game\n"),
            Print("┌────┬────┬────┬────┐\n")
        )?;

        for (i, row) in self.0.iter().enumerate() {
            for tile in row {
                execute!(stdout(), Print(format!("│{}", tile)))?;
            }
            execute!(stdout(), Print("│\n"))?;
            if i < SIZE - 1 {
                execute!(stdout(), Print("├────┼────┼────┼────┤\n"))?;
            }
        }

        execute!(
            stdout(),
            Print("└────┴────┴────┴────┘\n"),
            Print("Use arrow keys to move. Press 'q' to quit.\n")
        )?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut board = Board::new();

    execute!(stdout(), Hide)?;

    loop {
        board.print()?;
        stdout().flush()?;

        if board.is_game_over() {
            execute!(stdout(), Print("Game Over!\n"))?;
            break;
        }

        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    if board.move_tiles(key_event.code) {
                        board.add_random_tile();
                    }
                }
                _ => {}
            }
        }
    }

    execute!(stdout(), Show)?;
    Ok(())
}
