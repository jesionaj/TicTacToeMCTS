extern crate rand;
use rand::{thread_rng, Rng};

mod board;
mod tree;
mod ai;

fn main()
{
    let num_simulations = 100000000;
    let mut root = tree::Node::create_empty_node();
    let mut rng = thread_rng();

    for _ in 0..num_simulations
    {
        // Game board
        let mut board = board::Board::new();

        // A list of the moves left in the game
        let mut all_moves = vec![0, 1, 2, 3, 4, 5, 6, 7, 8]; // No range intializers with vec!

        // A list of the moves made
        let mut move_list: Vec<usize> = Vec::new();

        // The game result and current player
        let mut result = tree::Result::Draw;

        for _ in 0..9
        {
            // Choose a random move from all moves left            
            let n = rng.gen_range(0, all_moves.len());
            let current_move = all_moves[n];

            all_moves.remove(n);
            move_list.push(current_move);

            // Update the board state and end if there is a winner
            let winner = board.make_move(current_move);

            if winner == board::Player::One
            {
                result = tree::Result::P1Win;
                break;
            }
            else if winner == board::Player::Two
            {
                result = tree::Result::P2Win;
                break;
            }
        }
        // We're done. If there is no winner, the result is the default draw.

        // Move list has the latest move at the head, reverse it
        move_list.reverse();
        root.add_move_list(&mut move_list, result);
    }

    root.print_wins();

    ai::run_ai(&root);

}

