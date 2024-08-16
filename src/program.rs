#![allow(unused)]
// ### Questions ###
// Is there a trait for structs that can be created with New?
// Answered: What happens when a struct is moved: Depends
// If a struct has a field, is the field moved "with it?"
// -------------------------------------------------------
// Instresting perspecitve: you can see this as a single state machine and you want to find the
// smallest amount of signals to get it to move from one state to another
// ### TODO ###
// Implement "Solution"
use crate::validator::*;
use crate::{board, validator};
use crate::{board::*, Move};
use std::collections::HashSet;
pub trait Program: Iterator {
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
}
/// stuff all programs should contain

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
    found_boards: HashSet<Board>,
    next_boards: HashSet<Board>,
    current_boards: HashSet<Board>,
    step_counter: usize,
}
impl BFS {
    pub fn new(board: &Board, strategy: MoveChoice) -> Self {
        let mut bfs = BFS {
            strategy,
            name: "BFS".to_string(),
            starting_board: board.clone(),
            next_boards: HashSet::new(),
            current_boards: HashSet::new(),
            found_boards: HashSet::new(),
            step_counter: 0,
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
        //println!("{}", &self.current_boards.len());
        for board in &self.current_boards {
            for move_command in self.get_selected_moveset(board) {
                let mut newboard = board.clone();
                newboard.perform_move(move_command);
                //println!("newboard is {newboard}");
                if newboard.solved() {
                    self.found_boards.insert(newboard.clone());
                    println!("{}", &newboard);
                    self.current_boards.clear();
                    return true;
                }
                if !self.found_boards.contains(&newboard) {
                    println!("{}", &newboard);
                    //
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
        println!("step {}", self.step_counter);
        assert!(!self.current_boards.is_empty());
        false
    }
    pub fn get_full_solution(&self) -> Option<Solution> {
        let solution = validator::get_solution(&self.found_boards, &self.starting_board);
        if confirm_solution(&solution, &self.starting_board) {
            Some(solution)
        } else {
            None
        }
    }
}
