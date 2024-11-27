use web_time::SystemTime;
use rand::seq::IteratorRandom;
use rand::thread_rng;

#[derive(Debug,Clone, Copy, PartialEq)]
pub enum GameStatus { Win, GameOver, Running }

/// Settings for the Minesweeper game.
pub struct Settings { 
    /// Width of the game board.
    pub dx: usize, 
    /// Height of the game board.
    pub dy: usize, 
    /// Number of mines on the game board.
    pub mine_count: usize 
}
#[allow(dead_code)]
pub const BEGINNER_SETTINGS: Settings = Settings { dx: 8, dy: 8, mine_count: 10 };
#[allow(dead_code)]
pub const INTERMEDIATE_SETTINGS: Settings = Settings { dx: 16, dy: 16, mine_count: 40 };
#[allow(dead_code)]
pub const EXPERT_SETTINGS: Settings = Settings { dx: 30, dy: 16, mine_count: 99 };

/// Represents a single field on the Minesweeper game board.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Field {
    /// Indicates if the field is a mine.
    is_mine: bool,
    /// Indicates if the field has been revealed.
    is_revealed: bool,
    /// Indicates if the field is flagged.
    is_flagged: bool,
    /// The number of mines adjacent to this field.
    adjacent_mines: usize,
}

impl Field {
    /// Checks if the field is a mine.
    ///
    /// If the field has yet not been revealed, the result will always be `false` to hide this information.
    pub fn is_mine(&self) -> bool {
        if !self.is_revealed {
            return false;
        }
        return self.is_mine;
    }

    pub fn is_revealed(&self) -> bool {
        return self.is_revealed;
    }

    pub fn is_flagged(&self) -> bool {
        return self.is_flagged;
    }

    /// Returns the number of adjacent mines if field is revealed, otherwise zero. 
    pub fn adjacent_mines(&self) -> usize {
        if !self.is_revealed {
            return 0;
        }
        return self.adjacent_mines;
    }
}

/// Represents the Minesweeper game.
pub struct Minesweeper {
    status: GameStatus,
    /// Time the game (re)started.
    start: SystemTime,
    /// Width of the game board.
    dx: usize,
    /// Height of the game board.
    dy: usize,
    /// Number of mines on the game board.
    mine_count: usize,
    /// Number of fields that have been revealed to determine a win.
    revealed_count: usize,
    /// Number of flags.
    flagged_count: usize,
    /// Number of fields that are mines and flagged.
    flagged_mines_count: usize,
    /// 2D vector representing the game board.
    board: Vec<Vec<Field>>,
}

pub fn new(set: Settings) -> Minesweeper {
    let mut m = Minesweeper {
        status: GameStatus::Running,
        start: SystemTime::now(),
        dx: set.dx,
        dy: set.dy,
        mine_count: set.mine_count,
        revealed_count: 0,
        flagged_count: 0,
        flagged_mines_count: 0,
        board: vec![vec![Field::default(); set.dx as usize]; set.dy as usize],
    };

    m.restart();

    return m;
}

impl Minesweeper {
    pub fn status(&self) -> GameStatus {
        return self.status;
    }

    pub fn mine_count(&self) -> usize {
        return self.mine_count;
    }

    pub fn board_clone(&self) -> Vec<Vec<Field>> {
        return self.board.clone();
    }

    pub fn flagged_count(&self) -> usize {
        return self.flagged_count;
    }

    /// Returns the number of seconds the game has been running.
    ///
    /// This method keeps running even if the game status != Running.
    pub fn seconds_running(&self) -> usize {
        return SystemTime::now().duration_since(self.start).unwrap().as_secs() as usize;
    }

    /// (Re)starts the game.
    ///
    /// This method resets the game status to `Running`, sets the start time to the current time,
    /// resets the revealed field count, and reinitializes the game board with new mines.
    /// Initial game settings are left untouched.
    pub fn restart(&mut self) {
        self.status = GameStatus::Running;
        self.start = SystemTime::now();
        self.revealed_count = 0;
        self.flagged_count = 0;
        self.flagged_mines_count = 0;

        // reset fields
        self.board = vec![vec![Field::default(); self.dx as usize]; self.dy as usize];

        // generate mines
        let mines = (0..self.dx * self.dy).choose_multiple(&mut thread_rng(), self.mine_count as usize);
        for i in mines {
            let row = i / self.dx;
            let col = i % self.dx;
            self.board[row as usize][col as usize].is_mine = true;
        }

        // calculate adjacent_mines for every field
        for y in 0..self.dy as usize {
            for x in 0..self.dx as usize {
                if self.board[y][x].is_mine {
                    // upper left
                    if y > 0 && x > 0 { self.board[y-1][x-1].adjacent_mines += 1; }
                    // upper
                    if y > 0 { self.board[y-1][x].adjacent_mines += 1; }
                    // upper right
                    if y > 0 && x < self.dx-1 { self.board[y-1][x+1].adjacent_mines += 1; }
                    // left
                    if x > 0 { self.board[y][x-1].adjacent_mines += 1; }
                    // right
                    if x < self.dx-1 { self.board[y][x+1].adjacent_mines += 1; }
                    // lower left
                    if y < self.dy-1 && x > 0 { self.board[y+1][x-1].adjacent_mines += 1; }
                    // lower
                    if y < self.dy-1 { self.board[y+1][x].adjacent_mines += 1; }
                    // lower right
                    if y < self.dy-1 && x < self.dx-1 { self.board[y+1][x+1].adjacent_mines += 1; }
                }
            }
        }

        // debug print board
        self.print_board();
    }

    /// Reveals the field at the given coordinates.
    ///
    /// This method reveals the field at the specified coordinates. 
    /// If the field is a mine, the game is over.
    /// If the field is a zero, it will reveal all adjacent non-mines.
    /// Checks if the game is won.
    pub fn reveal(&mut self, x: usize, y: usize) {
        if self.status != GameStatus::Running {
            return;
        }

        let f = &mut self.board[y][x];
        if f.is_revealed || f.is_flagged {
            return
        }
        
        if f.is_mine { // mine => game over
            f.is_revealed = true;
            self.revealed_count += 1;
            self.status = GameStatus::GameOver;
        } else if f.adjacent_mines == 0 { // reveal adjacent zeros
            self.reveal_zeros(x, y);
        } else { // reveal this field
            f.is_revealed = true;
            self.revealed_count += 1;
        }

        self.check_win();

        self.print_board();
    }

    /// Reveals all connected fields with zero adjacent mines starting from the given coordinates.
    ///
    /// This method recursively reveals fields with zero adjacent mines, stopping when it encounters
    /// a field with non-zero adjacent mines or a field that has already been revealed.
    fn reveal_zeros(&mut self, x: usize, y: usize) {
        let f = &mut self.board[y][x];

        if f.is_revealed || f.is_flagged { // trivial cases
            return
        }

        f.is_revealed = true;
        self.revealed_count += 1;
        if f.adjacent_mines != 0 { // trivial case: field is not a zero
            return
        }

        // upper left
        if y > 0 && x > 0 { self.reveal_zeros(x-1, y-1); }
        // upper
        if y > 0 { self.reveal_zeros(x, y-1); }
        // upper right
        if y > 0 && x < self.dx-1 { self.reveal_zeros(x+1, y-1); }
        // left
        if x > 0 { self.reveal_zeros(x-1, y); }
        // right
        if x < self.dx-1 { self.reveal_zeros(x+1, y); }
        // lower left
        if y < self.dy-1 && x > 0 { self.reveal_zeros(x-1, y+1); }
        // lower
        if y < self.dy-1 { self.reveal_zeros(x, y+1); }
        // lower right
        if y < self.dy-1 && x < self.dx-1 { self.reveal_zeros(x+1, y+1); }
    }

    /// Flags or unflags the field at the given coordinates.
    ///
    /// This method flags or unflags the field at the specified coordinates. 
    /// Flagged fields cant be revealed.
    /// If the field is a mine, it counts towards winning.
    /// Checks if the game is won.
    pub fn flag(&mut self, x: usize, y: usize) {
        if self.status != GameStatus::Running {
            return;
        }

        let f = &mut self.board[y][x];
        if f.is_revealed { // trivial case: field is already revealed
            return;
        }

        if f.is_flagged { // unflag
            f.is_flagged = false;

            self.flagged_count -= 1;
            if f.is_mine {
                self.flagged_mines_count -= 1;
            }
        } else { // flag
            f.is_flagged = true;

            self.flagged_count += 1;
            if f.is_mine {
                self.flagged_mines_count += 1;
            }
        }

        self.check_win();

        self.print_board();
    }

    // Checks if the game is won.
    fn check_win(&mut self) {
        // determine if all fields except mines are revealed
        if (self.dx*self.dy) == (self.revealed_count+self.mine_count) {
            self.status = GameStatus::Win;
        }
        if self.flagged_mines_count == self.mine_count {
            self.status = GameStatus::Win;
        }
    }

    /// Prints board to stdout for debugging.
    ///
    /// * revealed fields are prefixed with 'r'
    /// * mines are displayed as 'X'
    /// * non-mines are displayed as digit
    pub fn print_board(&self) {
        let dx = self.dx*3+1;

        println!("|{:?}", self.status);
        println!("|mc: {:?}", self.mine_count);
        println!("|fc: {:?}", self.flagged_count);
        println!("|fmc: {:?}", self.flagged_mines_count);
        println!("|{}|", String::from("-").repeat(dx));
        for row in self.board.iter() {
            print!("| ");
            for col in row.iter() {
                if col.is_revealed {
                    print!("r")
                } else {
                    print!(" ")
                }
                if col.is_mine {
                    print!("X ");
                } else {
                    print!("{} ", col.adjacent_mines);
                }
            }
            println!("|");
        }
        println!("|{}|", String::from("-").repeat(dx));
        println!();
    }
}
