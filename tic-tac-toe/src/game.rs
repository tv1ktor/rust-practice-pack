use rand;
use std::{io, u32};

type Board = Vec<Vec<String>>;

enum Turn {
    Player,
    Bot,
}

pub struct Game {
    board: Board,
    current_turn: Turn,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![
                vec!["1".to_string(), "2".to_string(), "3".to_string()],
                vec!["4".to_string(), "5".to_string(), "6".to_string()],
                vec!["7".to_string(), "8".to_string(), "9".to_string()],
            ],
            current_turn: Turn::Player,
        }
    }

    pub fn play_game(&mut self) {
        let mut finished = false;

        while !finished {
            self.turn();

            if self.game_is_won() {
                self.print_board();

                match self.current_turn {
                    Turn::Player => println!("You won!"),
                    Turn::Bot => println!("You lost!"),
                }

                self.reset();

                finished = Self::player_is_finished();
            }

            self.current_turn = self.get_next_turn();
        }
    }

    fn turn(&mut self) {
        self.print_board();

        let (token, valid_move) = match self.current_turn {
            Turn::Player => ("X".to_string(), self.get_player_move()),
            Turn::Bot => ("O".to_string(), self.get_bot_move()),
        };

        let (row, column) = Self::move_to_board_location(valid_move);

        self.board[row][column] = token;
    }

    fn get_player_move(&self) -> u32 {
        loop {
            let mut input = String::new();

            println!("\nPlease enter your move (an integer between 1 and 9: ");

            match io::stdin().read_line(&mut input) {
                Err(_) => println!("Failed to read player's input."),
                Ok(_) => match self.validate_input(&input) {
                    Err(err) => println!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }

    fn validate_input(&self, player_input: &String) -> Result<u32, String> {
        match player_input.trim().parse::<u32>() {
            Err(_) => Err(String::from("Please enter valid number!")),
            Ok(num) => {
                if self.is_valid_move(num) {
                    Ok(num)
                } else {
                    Err(String::from("Please input a number of empty box"))
                }
            }
        }
    }

    fn is_valid_move(&self, unchecked_move: u32) -> bool {
        match unchecked_move {
            1..=9 => {
                let temp_location = Self::move_to_board_location(unchecked_move);

                match self.board[temp_location.0][temp_location.1].as_str() {
                    "X" | "O" => false,
                    _ => true,
                }
            }
            _ => false,
        }
    }

    fn move_to_board_location(position: u32) -> (usize, usize) {
        let row = (position - 1) / 3;
        let col = (position - 1) % 3;

        (row as usize, col as usize)
    }

    //  Prints the game board
    //
    //  +---+---+---+
    //  | 1 | 2 | 3 |
    //  +---+---+---+
    //  | 4 | 5 | 6 |
    //  +---+---+---+
    //  | 7 | 8 | 9 |
    //  +---+---+---+
    //
    fn print_board(&self) {
        let separator = "+---+---+---+";

        println!("\n{}", separator);

        for row in &self.board {
            println!("| {} |\n{}", row.join(" | "), separator);
        }

        println!("\n");
    }

    fn get_bot_move(&self) -> u32 {
        let mut bot_move: u32 = rand::random::<u32>() % 9 + 1;

        while !self.is_valid_move(bot_move) {
            bot_move = rand::random::<u32>() % 9 + 1;
        }

        println!("Bot make move at position {}", bot_move);

        bot_move
    }

    fn game_is_won(&self) -> bool {
        let mut all_same_col = false;
        let mut all_same_row = false;

        for index in 0..3 {
            all_same_col |= self.board[index][0] == self.board[index][1]
                && self.board[index][1] == self.board[index][2];
            all_same_row |= self.board[0][index] == self.board[1][index]
                && self.board[1][index] == self.board[2][index];
        }

        let all_same_diag_1 =
            self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2];
        let all_same_diag_2 =
            self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0];

        all_same_col || all_same_row || all_same_diag_1 || all_same_diag_2
    }

    fn player_is_finished() -> bool {
        let mut response = String::new();

        println!("Are you finished playing (y/n)?:");

        match io::stdin().read_line(&mut response) {
            Ok(_) => {
                let temp_response = response.to_lowercase();

                temp_response.trim() == "y" || temp_response.trim() == "yes"
            }
            Err(_) => false,
        }
    }

    fn reset(&mut self) {
        self.current_turn = Turn::Player;
        self.board = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
    }

    fn get_next_turn(&mut self) -> Turn {
        match self.current_turn {
            Turn::Bot => Turn::Player,
            Turn::Player => Turn::Bot,
        }
    }
}
