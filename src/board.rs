//  ##### TODO #######
//
//
// Look into the possibility of using mem::swap for replacing values
// Consider tracking higest and lowest card for each pile
//
// ######
use crate::vector_util;
use crate::Move;
use core::fmt;
use core::panic;
use std::fmt::*;
use std::hash::*;
use std::{u8, usize};

static SOLUTION_PILE: [u8; 2] = [2, 1];

#[derive(Debug, Clone)]
/// Representation of a full set of cardpiles.
/// Piles are always sorted in order of the value of the bottom card, highest to lowest.
pub struct Board {
    pub piles: Vec<Vec<u8>>,
    abs_to_rel_translator: Vec<usize>,
    pub nbr_cards: usize,
    highest_card_is_on_bottom: bool,
    has_solution_pile: bool,
    pos_of_highest_card: usize,
    last_move: Option<Move>,
}
/// Hashing is based on relative pile positions
impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for pile in self.relative_piles() {
            pile.hash(state);
        }
    }
}
impl Eq for Board {}
impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.relative_piles() == other.relative_piles()
    }
}

/// Displays the Board based on relative pile position. A board will look similar to:
/// ```<[5][4][1 2 3]_ _>``` when printed in the terminal
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pile_ids: Vec<usize> = Vec::new();

        for i in 0..self.piles.len() {
            pile_ids.push(i);
        }
        pile_ids.iter_mut().for_each(|x| *x = self.rel_to_abs(*x));
        write!(f, "({})", self.nbr_cards)?;
        write!(f, "<")?;
        for i in pile_ids {
            let mut pile = self.piles[i].clone();
            if !pile.is_empty() {
                write!(f, "[")?;

                let last = pile.pop();
                for card in pile.iter() {
                    write!(f, "{} ", card)?;
                }
                if Option::is_some(&last) {
                    write!(f, "{}", last.unwrap())?;
                }
                write!(f, "]")?;
            } else {
                write!(f, " _")?;
            }
        }
        write!(f, ">")
    }
}

impl Board {
    /// Creates a new Board, with all cards placed in the 0th pile.
    pub fn new(pile: &[u8], nbr_piles: usize) -> Board {
        assert!(pile.len() > 2);
        assert!(nbr_piles > 2);
        assert!(!vector_util::contains_zero(&pile.to_vec()));
        assert!(vector_util::correct_sequence(&pile.to_vec()));
        let mut new_piles = Vec::new();
        let mut new_position_translator = Vec::new();
        let mut new_nbr_cards = pile.len();
        let mut new_highest_card_is_on_bottom = false;

        new_piles.push(pile.to_owned());
        for _ in 1..nbr_piles {
            new_piles.push(Vec::<u8>::new());
        }

        for i in 0..nbr_piles {
            new_position_translator.push(i);
        }

        if pile.len() == pile[0].into() {
            new_highest_card_is_on_bottom = true;

            while (new_piles[0][0] == new_piles[0][1] + 1)
                && (new_piles[0][1] == new_piles[0][2] + 1)
            {
                new_piles[0].remove(0);
                new_nbr_cards -= 1;
                if new_nbr_cards == 2 {
                    break;
                }
            }
        }
        let mut board = Board {
            piles: new_piles,
            abs_to_rel_translator: new_position_translator,
            nbr_cards: new_nbr_cards,
            highest_card_is_on_bottom: new_highest_card_is_on_bottom,
            has_solution_pile: false,
            pos_of_highest_card: 0,
            last_move: None,
        };
        board.update_indexes();
        board
    }
    /// Gives all moves(absolute) that may be performed that yields a valid state,
    /// performing any other move will cause a panic.
    fn valid_moves_abs(&self) -> Vec<Move> {
        let mut non_empty_piles = Vec::<usize>::new();
        let mut empty_piles = Vec::<usize>::new();

        for (i, el) in self.piles.iter().enumerate() {
            if el.is_empty() {
                empty_piles.push(i);
            } else {
                non_empty_piles.push(i);
            }
        }

        let valid_from = non_empty_piles.clone();
        let mut valid_to = non_empty_piles.clone();
        if !empty_piles.is_empty() {
            valid_to.push(*empty_piles.first().unwrap());
        }
        let mut valid_moves = Vec::<Move>::new();
        for from in &valid_from {
            for to in &valid_to {
                valid_moves.push([*from, *to])
            }
        }

        // doesn't take from empty pile
        // doesn't put in empty pile except first one
        // doesn't take from one pile and put into same
        valid_moves
    }
    fn relative_piles(&self) -> Vec<Vec<u8>> {
        let mut piles_in_rel_order = Vec::new();
        let mut pile_ids: Vec<usize> = Vec::new();

        for i in 0..self.piles.len() - 1 {
            pile_ids.push(i);
        }
        pile_ids.iter_mut().for_each(|x| *x = self.rel_to_abs(*x));
        for i in pile_ids {
            piles_in_rel_order.push(self.piles[i].clone());
        }
        piles_in_rel_order
    }

    pub fn valid_moves_rel(&self) -> Vec<Move> {
        let mut moves = self.valid_moves_abs();
        moves.iter_mut().for_each(|x| *x = self.abs_to_rel_move(*x));
        moves
    }

    /// Returns all moves(relative) that may lead to a better solution.
    pub fn good_moves_rel(&self) -> Vec<Move> {
        let next_card_needed = self.nbr_cards - self.piles[self.pos_of_highest_card].len();
        if self.solved() {
            return vec![];
        }
        let mut valid_moves = self.valid_moves_abs();
        valid_moves.retain(|x| self.not_last_move(x)); // you never need to undo the last move.                             //
        valid_moves.retain(|x| x[0] != x[1]); /* picking up and putting down a card in the same
                                              // place is meaningless */

        if self.has_solution_pile {
            for (i, pile) in self.piles.iter().enumerate() {
                let last = pile.last();
                if last.is_some_and(|x| usize::from(*x) == next_card_needed) {
                    let move_command = [i, self.pos_of_highest_card];
                    return vec![self.abs_to_rel_move(move_command)];
                }
            }
            valid_moves.retain(|x| x[0] != self.pos_of_highest_card); // never remove card from solutionpile
        } else {
        }
        if !self.has_solution_pile {
            for (i, pile) in self.piles.iter().enumerate() {
                if pile.is_empty() {
                    if usize::from(*self.piles[self.pos_of_highest_card].last().unwrap())
                        == self.nbr_cards
                    {
                        return vec![self.abs_to_rel_move([self.pos_of_highest_card, i])];
                    }
                }
            }
        }
        valid_moves.retain(|x| x[1] != self.pos_of_highest_card);
        /* Speculated but not implemented: doesn't put bad cards on solutionpile.
        not sure if there are cases where such a reshuffle is required or not */
        //assert!(!valid_moves.is_empty());
        valid_moves
            .iter_mut()
            .for_each(|x| *x = self.abs_to_rel_move(*x));

        valid_moves
    }
    fn not_last_move(&self, move_command: &Move) -> bool {
        if Option::is_none(&self.last_move) {
            return true;
        }
        let last_move = self.last_move.unwrap();
        if move_command[0] == last_move[1] {
            return false;
        }
        true
    }

    /// Performs a move. Move instructions are "relative".
    pub fn perform_move(&mut self, move_command: Move) {
        // seperate into move and place logic?

        let from_rel = move_command[0];
        let to_rel = move_command[1];
        let from_abs = self.rel_to_abs(from_rel);
        let to_abs = self.rel_to_abs(to_rel);
        let card = *self.piles[from_abs].last().unwrap();
        let moved_higest_card = usize::from(card) == self.nbr_cards;
        let moved_on_top_of_highest_card = to_abs == self.pos_of_highest_card;
        let had_solution_pile = self.has_solution_pile;
        let card_diff = self.nbr_cards - usize::from(card);
        let should_go_on_top =
            (usize::wrapping_sub(self.piles[self.pos_of_highest_card].len(), card_diff)) == 0;
        let shrink = (card_diff == 2) && should_go_on_top;

        self.last_move = Some([from_abs, to_abs]);

        assert!(
            self.valid_moves_rel().contains(&move_command),
            "move command {:?}, wasn't contained in valid commands: {:?} (rel) || {:?} (abs), \n 
            current board is {}",
            move_command,
            self.valid_moves_rel(),
            self.valid_moves_abs(),
            &self,
        );

        if moved_higest_card {
            self.pos_of_highest_card = to_abs;
            if self.piles[to_abs].is_empty() {
                self.highest_card_is_on_bottom = true;
                self.has_solution_pile = true;
            } else {
                self.highest_card_is_on_bottom = false;
                self.has_solution_pile = false;
            }
        }
        if moved_on_top_of_highest_card {
            if had_solution_pile && should_go_on_top {
                if shrink {
                    self.piles[to_abs].remove(0);
                    self.nbr_cards -= 1;
                }
            } else {
                self.has_solution_pile = false;
            }
        }

        self.piles[from_abs].pop().unwrap();
        self.piles[to_abs].push(card);
        self.update_indexes();
    }
    /// A solved pile will be identical to a pile with the cards \[2,1\] in one pile and no other
    /// cards.
    pub fn solved(&self) -> bool {
        let potential_solution_pile = &self.piles[self.pos_of_highest_card];
        potential_solution_pile == &SOLUTION_PILE
    }
    fn abs_to_rel(&self, abs_val: usize) -> usize {
        self.abs_to_rel_translator[abs_val]
    }
    fn abs_to_rel_move(&self, abs_move: Move) -> Move {
        [self.abs_to_rel(abs_move[0]), self.abs_to_rel(abs_move[1])]
    }

    fn rel_to_abs(&self, rel_val: usize) -> usize {
        for (i, el) in self.abs_to_rel_translator.iter().enumerate() {
            if el == &rel_val {
                return i;
            }
        }
        panic!();
    }
    fn rel_to_abs_move(&self, rel_move: Move) -> Move {
        [self.rel_to_abs(rel_move[0]), self.rel_to_abs(rel_move[1])]
    }

    fn update_indexes(&mut self) {
        let mut non_empty_piles = Vec::<usize>::new();
        let mut empty_piles = Vec::<usize>::new();

        for (i, el) in self.piles.iter().enumerate() {
            if el.is_empty() {
                empty_piles.push(i);
            } else {
                non_empty_piles.push(i);
            }
        }
        non_empty_piles.sort_by(|a, b| self.piles[*b][0].cmp(&self.piles[*a][0]));
        let mut counter = 0;

        for pile in &non_empty_piles {
            self.abs_to_rel_translator[*pile] = counter;
            counter += 1;
        }
        for pile in empty_piles {
            self.abs_to_rel_translator[pile] = counter;
            counter += 1;
        }
        // order rel based on highest card
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn new_board() {
        {
            let input = [4, 3, 2, 1];
            let expected = SOLUTION_PILE;

            let board: Board = Board::new(&input, 4);
            assert_eq!(board.piles[0], expected);
            assert!(board.solved());
        }
        {
            let input = [1, 2, 3, 4];
            let expected = [1, 2, 3, 4];
            let board: Board = Board::new(&input, 4);
            assert_eq!(board.piles[0], expected);
            assert!(!board.solved());
        }
        {
            let input = [8, 7, 6, 5, 1, 2, 3, 4];
            let expected = [6, 5, 1, 2, 3, 4];

            let board = Board::new(&input, 7);
            assert_eq!(board.piles[0], expected);
            assert!(!board.solved())
        }
    }
    #[test]
    #[should_panic]
    fn too_short() {
        Board::new(&[2, 1], 4);
    }
    #[test]
    #[should_panic]
    fn contains_zero() {
        Board::new(&[3, 4, 0, 2, 1], 4);
    }

    #[test]
    #[should_panic]
    fn contains_gap() {
        Board::new(&[1, 2, 3, 5], 4);
    }
    #[test]
    #[should_panic]
    fn starts_at_wrong_index() {
        Board::new(&[2, 3, 4, 6, 5], 4);
    }

    fn get_hash<T>(obj: &T) -> u64
    where
        T: Hash,
    {
        let mut hasher = fxhash::FxHasher::default();
        obj.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hash_test() {
        //TODO: use a hashmap to double check this
        let mut board1 = Board::new(&vec![1, 2, 3, 4], 4);
        let mut board2 = Board::new(&vec![1, 2, 4, 3], 4);

        assert_ne!(get_hash(&board1), get_hash(&board2));

        board1.perform_move([0, 1]); //[4][1,2,3]
        board2.perform_move([0, 1]); //[3][1,2,4]

        assert_ne!(get_hash(&board1), get_hash(&board2));

        board1.perform_move([1, 2]); //[4][3][1,2]
        board2.perform_move([1, 2]); //[4][3][1,2]

        assert_eq!(get_hash(&board1), get_hash(&board2));
    }
    #[test]
    fn display_test() {
        let mut board1 = Board::new(&vec![1, 2, 3, 4], 4);
        let mut board2 = Board::new(&vec![1, 2, 4, 3], 4);

        assert_ne!(format!("{}", board1), format!("{}", board2));
        println!("{board1} != {board2} ");

        board1.perform_move([0, 1]); //[4][1,2,3]
        board2.perform_move([0, 1]); //[3][1,2,4]

        assert_ne!(format!("{}", board1), format!("{}", board2));
        println!("{board1} != {board2} ");

        board1.perform_move([1, 2]); //[4][3][1,2]
        board2.perform_move([1, 2]); //[4][3][1,2]

        assert_eq!(format!("{}", board1), format!("{}", board2));
        println!("{board1} == {board2} ");
    }
}
