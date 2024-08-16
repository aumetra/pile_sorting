use ::sorting::board::Board;
use ::sorting::program::BFS;

//40 000 boards fine, 200 000 slowdown
fn main() {
    let vec = vec![2, 5, 3, 4, 6, 1, 7];
    let board1 = Board::new(&vec, 4);
    let mut bfs1 = BFS::new(&board1, sorting::program::MoveChoice::Valid);
    while !bfs1.internal_step() {}
    println!(" Done, checking solution");
    match bfs1.get_full_solution() {
        Some(_) => println!("success!"),
        None => println!("failure"),
    }
}
