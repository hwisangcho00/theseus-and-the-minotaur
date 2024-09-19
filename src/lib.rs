use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Lose,
    Continue,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardError {
    InvalidCharacter(char),
    InvalidSize,
    NoMinotaur,
    NoTheseus,
    NoGoal,
    MultipleMinotaur,
    MultipleTheseus,
    MultipleGoal,
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            BoardError::InvalidSize => write!(f, "Invalid size"),
            BoardError::NoMinotaur => write!(f, "No minotaur"),
            BoardError::NoTheseus => write!(f, "No theseus"),
            BoardError::NoGoal => write!(f, "No goal"),
            BoardError::MultipleMinotaur => write!(f, "Multiple minotaur"),
            BoardError::MultipleTheseus => write!(f, "Multiple theseus"),
            BoardError::MultipleGoal => write!(f, "Multiple goal"),
        }
    }
}
impl Error for BoardError {}

#[derive(Clone)]
pub enum BoardTile {
    Wall,
    Empty,
    Theseus,
    Minotaur,
    Goal,
}


#[derive(Clone)]
pub struct Game {
    grid: Vec<Vec<BoardTile>>,
    theseus_coordinate: (usize, usize),
    minotaur_coordinate: (usize, usize),
    goal_coordinate: (usize, usize),
}

impl Game {
    // TODO: replace the function body with your implementation
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        let mut grid = Vec::new();
        let mut theseus_coordinate = (0, 0);
        let mut minotaur_coordinate = (0, 0);
        let mut goal_coordinate = (0, 0);
        let mut row_num = 0;
        for line in board.lines() {
            let mut col_num = 0;
            let mut row = Vec::new();

            for c in line.chars() {
                match c {
                    ' ' => {row.push(BoardTile::Empty)}
                    'X' => {row.push(BoardTile::Wall)}
                    'M' => {
                        row.push(BoardTile::Minotaur); 
                        minotaur_coordinate = (row_num, col_num)
                    }
                    'T' => {
                        row.push(BoardTile::Theseus); 
                        theseus_coordinate = (row_num, col_num)
                    }
                    'G' => {row.push(BoardTile::Goal);
                        goal_coordinate = (row_num, col_num)}
                    c => {return Err(BoardError::InvalidCharacter(c))}
                }
                col_num += 1;
            }
            grid.push(row);
            row_num += 1;
        }


        return Ok(Game {
            grid: grid,
            theseus_coordinate: theseus_coordinate,
            minotaur_coordinate: minotaur_coordinate,
            goal_coordinate: goal_coordinate
        });
    }

    // TODO
    pub fn show(&self) {
        for row in self.grid.iter() {
            for tile in row.iter() {
                match tile {
                    BoardTile::Empty => {print!(" ")}
                    BoardTile::Goal => {print!("G")}
                    BoardTile::Minotaur => {print!("M")}
                    BoardTile::Theseus => {print!("T")}
                    BoardTile::Wall => {print!("█")}
                } 
            }
            println!();
        }

    }


    pub fn move_minotaur(&mut self, row: isize, col: isize) ->  bool{
        let m_row = self.minotaur_coordinate.0;
        let m_col = self.minotaur_coordinate.1;

        let new_row = (m_row as isize + row) as usize;
        let new_col = (m_col as isize + col) as usize;

        if !self.is_wall(new_row, new_col) {
          self.grid[m_row][m_col] = BoardTile::Empty;
          self.grid[new_row][new_col] = BoardTile::Minotaur;
          self.minotaur_coordinate = (new_row, new_col);
          return true;
        } 
        return false;
      }
    
    
    pub fn minotaur_move(&mut self) {
      let m_row = self.minotaur_coordinate.0;
      let m_col = self.minotaur_coordinate.1;
      let t_row = self.theseus_coordinate.0;
      let t_col = self.theseus_coordinate.1;

      if m_col < t_col {
        if self.move_minotaur(0, 1) {
          return;
        }
      }
      else if m_col > t_col {
        if self.move_minotaur(0, -1) {
          return;
        }
      }

      if m_row < t_row {
        if self.move_minotaur(1, 0) {
          return;
        }
      }
      else if m_row > t_row {
        if self.move_minotaur(-1, 0) {
          return;
        }
      }

      return;

    }


    pub fn move_theseus(&mut self, row: isize, col: isize) {
        let t_row = self.theseus_coordinate.0;
        let t_col = self.theseus_coordinate.1;

        let new_row = (t_row as isize + row) as usize;
        let new_col = (t_col as isize + col) as usize;

        if !self.is_wall(new_row, new_col) {
            self.grid[t_row][t_col] = BoardTile::Empty;
            self.grid[new_row][new_col] = BoardTile::Theseus;
            self.theseus_coordinate = (new_row, new_col);
        }
    }

    pub fn theseus_move(&mut self, command: Command) {
        match command {
            Command::Up => self.move_theseus(-1, 0),
            Command::Down => self.move_theseus(1, 0),
            Command::Left => self.move_theseus(0, -1),
            Command::Right => self.move_theseus(0, 1),
            Command::Skip => {}, // No movement on Skip
        }
    }

    // TODO: replace the function body with your implementation
    pub fn status(&self) -> GameStatus {
        if self.minotaur_coordinate == self.theseus_coordinate {
            return GameStatus::Lose;
        }
        if self.is_goal(self.theseus_coordinate.0, self.theseus_coordinate.1) {
            return GameStatus::Win;
        }
        GameStatus::Continue
    }
}

impl Game {

    fn return_boardtile(&self, row: usize, col: usize) -> Option<BoardTile> {
        match self.grid.get(row) {
            Some(r) =>
                match r.get(col) {
                    Some(c) => Some(c.clone()),
                    None => None
                },
            None => None
        }
    }

    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        match self.return_boardtile(row, col) {
            Some(c) =>
                match c {
                    BoardTile::Theseus => true,
                    _ => false,
                },
            None => false
        }
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        match self.return_boardtile(row, col) {
            Some(c) =>
                match c {
                    BoardTile::Minotaur => true,
                    _ => false,
                },
            None => false
        }
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        match self.return_boardtile(row, col) {
            Some(c) =>
                match c {
                    BoardTile::Wall => true,
                    _ => false,
                },
            None => false
        }
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        return self.goal_coordinate == (row, col);
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        match self.return_boardtile(row, col) {
            Some(c) =>
                match c {
                    BoardTile::Empty => true,
                    _ => false,
                },
            None => false
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Move one tile up
    Up,
    /// Move one tile down
    Down,
    /// Move one tile left
    Left,
    /// Move one tile right
    Right,
    /// Don't move at all
    Skip,
}

//  To get a command from the user, you can use the following code:
//  ```
//  let line = stdin.lines().next().unwrap().unwrap();
//  ```
//  This will read a line from the user and store it in the `buffer` string.
//  
//  Unfortunately, since stdin is line-buffered, everytime you enter a command while playing the
//  game you will have to press "enter" afterwards to send a new line.
//
//  While using the arrow keys to take inputs would be natural, it can be difficult to handle arrow
//  keys in a way that works on all devices. Therefore, it's recommended that you either use "w",
//  "a", "s", and "d" to take input, or else the words "up", "down", "left", "right". You can take
//  input however you like, so long as you document it here in a comment and it is reasonable to
//  use as a player.
pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    let line = stdin.lines().next().unwrap().unwrap().trim().to_string();

    return match line.as_str() {
        "w" => {Some(Command::Up)}
        "up" => {Some(Command::Up)}
        "a" => {Some(Command::Left)}
        "left" => {Some(Command::Left)}
        "s" => {Some(Command::Down)}
        "down" => {Some(Command::Down)}
        "d" =>{Some(Command::Right)}
        "right" =>{Some(Command::Right)}
        " " => {Some(Command::Skip)}
        "skip" => {Some(Command::Skip)}
        _ => {None}
    }
}
