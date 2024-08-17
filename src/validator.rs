use crate::board::*;
use crate::program::MoveChoice;
use crate::Move;
use std::collections::hash_map::*;
use std::collections::hash_set::*;
pub type Solution = Vec<Move>;
const VALIDATOR_SHOULD_PRINT: bool = true;
pub fn get_solution(
    set: &HashSet<Board, crate::OwnHasher>,
    starting_board: &Board,
    strategy_used: &MoveChoice,
) -> Solution {
    let nbr_piles = starting_board.piles.len();
    let mut board_sequence_inverted: Vec<Board> = Vec::new();
    let solution_board = Board::new_solved_board(nbr_piles);
    let mut board_option: Option<Board> = set.get(&solution_board).cloned();
    'outer: loop {
        match board_option {
            Some(board) => {
                board_sequence_inverted.push(board.clone());
                let a = board.revert();
                if Option::is_some(&a) {
                    board_option = set.get(&a.unwrap()).cloned();
                } else {
                    break 'outer;
                }
            }
            None => {
                break 'outer;
            }
        }
    }
    board_sequence_inverted.reverse();
    assert!(board_sequence_inverted.last().unwrap().solved());
    assert!(
        board_sequence_inverted[0] == *starting_board,
        "expected starting board {}, but got {} \n 
        move strategy: {:?}",
        starting_board,
        board_sequence_inverted[0],
        strategy_used
    );
    board_seq_to_move(&board_sequence_inverted)
}

pub fn board_seq_to_move(vec: &Vec<Board>) -> Solution {
    let mut vec = vec.to_owned();
    assert!(vec[0].last_move.is_none());
    vec.remove(0);
    vec.into_iter().map(|x| x.last_move.unwrap()).collect()
}

pub fn confirm_solution(solution: &Solution, starting_board: &Board) -> bool {
    let mut board = starting_board.clone();
    if VALIDATOR_SHOULD_PRINT {
        println!("starting board validation for: {}", &board);
    }
    for abs_move_command in solution {
        let rel_move = board.abs_to_rel_move(*abs_move_command);
        board.perform_move(rel_move, "confirming_solution");
        //board.perform_move_unchecked(rel_move);

        //board.abs_to_rel_translator = starting_board.abs_to_rel_translator.clone();
        if VALIDATOR_SHOULD_PRINT {
            println!("{}", &board);
        }
    }
    board.solved()
}

fn get_all_solutions(_map: &HashMap<Board, Vec<Move>>) -> Vec<Solution> {
    unimplemented!();
}
