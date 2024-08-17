#![allow(unused)]
const SHOULD_PRINT_FOUND_BOARDS: bool = false;
const SHOULD_PRINT_STEP_COUNTER: bool = false;
use crate::validator::*;
use crate::{board, validator};
use crate::{board::*, Move};
use std::collections::HashSet;
/*pub trait Program: Iterator {
    fn starting_state(&self) -> &Board;
    fn done(&self) -> bool;
    /// Runs the program until a new Move has been reached, must change result of done method when
    /// applicable
    fn step(&mut self) -> Option<Move>;
    /// Runs the program to completion.
    /// Will return new moves made only
    fn run(&mut self) -> Vec<Move> {
        let mut vec: Vec<Move> = Vec::<Move>::new();
        while (!self.done()) {
            if let Some(value) = self.step() {
                vec.push(value)
            }
        }
        vec
    }
    /// Returns all moves so far
    fn progress(&self) -> &Vec<Move>;
} */

#[derive(Debug)]
pub enum MoveChoice {
    Valid,
    Good,
    Unconfirmed,
}

pub struct BFS {
    strategy: MoveChoice,
    name: String,
    starting_board: Board,
    found_boards: HashSet<Board, crate::OwnHasher>,
    next_boards: HashSet<Board, crate::OwnHasher>,
    current_boards: HashSet<Board, crate::OwnHasher>,
    step_counter: usize,
    solved_board: Option<Board>,
}
impl BFS {
    pub fn new(board: &Board, strategy: MoveChoice) -> Self {
        let mut bfs = BFS {
            strategy,
            name: "BFS".to_string(),
            starting_board: board.clone(),
            next_boards: HashSet::default(),
            current_boards: HashSet::default(),
            found_boards: HashSet::default(),
            step_counter: 0,
            solved_board: None,
        };
        bfs.found_boards.insert(bfs.starting_board.clone());
        bfs.current_boards.insert(bfs.starting_board.clone());
        bfs
    }
    fn get_selected_moveset(&self, board: &Board) -> Vec<Move> {
        match &self.strategy {
            MoveChoice::Valid => board.valid_moves_rel(),
            MoveChoice::Good => board.good_moves_rel(),
            MoveChoice::Unconfirmed => board.unconfirmed_validity_moves_rel(),
        }
    }
    pub fn internal_step(&mut self) -> bool {
        for board in &self.current_boards {
            for move_command in self.get_selected_moveset(board) {
                let mut newboard = board.clone();
                newboard.perform_move(
                    move_command,
                    &format!("bfs with srategy {:?}", self.strategy),
                );
                if newboard.solved() {
                    self.found_boards.insert(newboard.clone());
                    if SHOULD_PRINT_FOUND_BOARDS {
                        println!("{}", &newboard);
                    }
                    self.solved_board = Some(newboard.clone());
                    self.current_boards.clear();
                    return true;
                }
                if !self.found_boards.contains(&newboard) {
                    if SHOULD_PRINT_FOUND_BOARDS {
                        println!("{}", &newboard)
                    };
                    self.next_boards.insert(newboard.clone());
                    self.found_boards.insert(newboard);
                }
            }
        }
        assert!(!self.current_boards.is_empty());
        assert!(!self.next_boards.is_empty());
        self.current_boards.clear();
        self.current_boards = self.next_boards.clone();
        self.next_boards.clear();
        self.step_counter += 1;
        if SHOULD_PRINT_STEP_COUNTER {
            println!("step {}", self.step_counter);
        }
        assert!(!self.current_boards.is_empty());
        false
    }
    pub fn get_full_solution(&self) -> Option<Solution> {
        let solution =
            validator::get_solution(&self.found_boards, &self.starting_board, &self.strategy);
        if confirm_solution(&solution, &self.starting_board) {
            Some(solution)
        } else {
            None
        }
    }
    pub fn solution_lenght(&mut self) -> usize {
        while !self.internal_step() {}
        let solution =
            validator::get_solution(&self.found_boards, &self.starting_board, &self.strategy);
        solution.len()
    }
    pub fn solve(&mut self) -> Option<Solution> {
        while !self.internal_step() {}
        self.get_full_solution()
    }
}

#[cfg(test)]
mod test {
    use crate::vector_util;

    use super::*;
    #[test]
    fn compare_all() {
        let mut all_board: Vec<Board> = Vec::new();
        for pile in vector_util::all_sequences(5) {
            all_board.push(Board::new(&pile, 4));
        }
        all_board.push(Board::new(&vec![2, 5, 3, 4, 6, 1, 7], 4));
        for board in all_board {
            best_solution_not_exluded(&board);
        }
    }

    fn best_solution_not_exluded(board: &Board) {
        let mut bfs_good = BFS::new(&board, MoveChoice::Good);
        let mut bfs_valid = BFS::new(&board, MoveChoice::Valid);
        let mut bfs_unconfirmed = BFS::new(&board, MoveChoice::Unconfirmed);
        let valid_len = bfs_valid
            .solve()
            .expect("There is always a valid solution")
            .len();
        let good_len = bfs_good
            .solve()
            .expect("There should always be a good solution")
            .len();
        let unconfirmed_len = bfs_unconfirmed
            .solve()
            .expect("Unconfirmed method failed")
            .len();

        assert!(
            valid_len == good_len && good_len == unconfirmed_len,
            "valid: {}, good: {}, unconfirmed: {}",
            valid_len,
            good_len,
            unconfirmed_len
        );
    }
    #[test]
    fn confirming_bug() {
        let pile = vec![1, 5, 2, 3, 4];
        let nbr_piles = 4;
        let strategy = MoveChoice::Good;
        let board = Board::new(&pile, nbr_piles);
        let mut bfs = BFS::new(&board, strategy);
        let potential_solution = bfs.solve();
        assert!(bfs.solved_board.unwrap() == Board::new_solved_board(4))
    }

    proptest::proptest! {
        #[test]
        fn break_stuff(seed: [u8; 16]) {
            crate::SEED.lock().unwrap().replace(seed);

            let pile = vec![1, 5, 2, 3, 4];
            let nbr_piles = 4;
            let strategy = MoveChoice::Good;
            let board = Board::new(&pile, nbr_piles);
            let mut bfs = BFS::new(&board, strategy);
            let potential_solution = bfs.solve();
            assert!(bfs.solved_board.unwrap() == Board::new_solved_board(4));
        }
    }
}
