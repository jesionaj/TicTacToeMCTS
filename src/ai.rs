use tree;
use board::Player;
use std;

// The AI's strategy is to minimize the number of wins the other player can achieve
pub fn run_ai(root: &tree::Node)
{
    let mut current_player = Player::One;
    let mut current_node = root;

    for _ in 0..9
    {
        let mut best_wins = 0;
        let mut next_node = std::usize::MAX;

        // Check each child node, choose the one that minimize's opponent's wins
        for idx in 0..9
        {
            match current_node.children[idx]
            {
                Some(ref x) => {
                    print!("{}: ", idx);
                    x.print_wins();

                    let half_draw = x.draw_count >> 1;

                    if current_player == Player::One
                    {
                        if x.p1_win_count + half_draw > best_wins
                        {
                            next_node = idx;
                            best_wins = x.p1_win_count + half_draw;
                        }
                    }
                    else
                    {
                        if x.p2_win_count + half_draw > best_wins
                        {
                            next_node = idx;
                            best_wins = x.p2_win_count + half_draw;
                        }
                    }
                },
                None => { },
            }
        }

        if next_node != std::usize::MAX
        {
            println!("Next node chosen: {}", next_node);

            // Set current node to next child
            match current_node.children[next_node]
            {
                Some(ref x) => {
                    current_node = x;
                },
                None => { },
            }

            // Swap current player
            if current_player == Player::One
            {
                current_player = Player::Two;
            }
            else
            {
                current_player = Player::One;
            }
        }
        else
        {
            // This should never happen except at the last node, optimal play results in a draw
            break;
        }
    }
}