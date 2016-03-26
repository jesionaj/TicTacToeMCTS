#[derive(Copy,Clone,PartialEq)]
pub enum Player {
    None,
    One,
    Two
}

pub const BOARDSIZE: usize = 3;

pub struct Board 
{
    pub state: [[Player; BOARDSIZE]; BOARDSIZE],
    player: Player
}

impl Board
{
    pub fn new() -> Board
    {
        Board{ state: [[Player::None; BOARDSIZE]; BOARDSIZE], player: Player::One }
    }

    pub fn make_move_rc(&mut self, row: usize, col: usize) -> Player
    {
        self.state[row][col] = self.player;

        // Check for a winner and return the current player if they've won
        if self.is_winner()
        {
            return self.player
        }

        // Swap current player and return no winner
        if self.player == Player::One
        {
            self.player = Player::Two;
        }
        else
        {
            self.player = Player::One;
        }

        return Player::None;
    }

    pub fn make_move(&mut self, idx: usize) -> Player
    {
        return self.make_move_rc(idx / BOARDSIZE, idx % BOARDSIZE);
    }

    // Consider writing a function to check only changed row/col/diagional instead of whole board
    // Consider writing can_x_be_winner so we don't check each row and column each time
    // The above isn't important for size of 3 though
    pub fn is_winner(&self) -> bool
    {
        for idx in 0..BOARDSIZE
        {
            if self.is_row_winner(idx)
            {
                return true;
            }

            if self.is_col_winner(idx)
            {
                return true;
            }
        }

        return self.is_forward_diagional_winner() || self.is_backward_diagional_winner();
    }

    // Consider re-writing these methods. Create different iterators instead?
    fn is_row_winner(&self, row: usize) -> bool
    {
        let initial = self.state[row][0];

        for col in 1..BOARDSIZE
        {
            if self.state[row][col] == Player::None
            {
                return false;
            }

            if self.state[row][col] != initial
            {
                return false;
            }
        }

        return true;
    }


    fn is_col_winner(&self, col: usize) -> bool
    {
        let initial = self.state[0][col];

        for row in 1..BOARDSIZE
        {
            if self.state[row][col] == Player::None
            {
                return false;
            }

            if self.state[row][col] != initial
            {
                return false;
            }
        }

        return true;
    }

    fn is_forward_diagional_winner(&self) -> bool
    {
        let initial = self.state[0][0];

        for idx in 0..BOARDSIZE
        {
            if self.state[idx][idx] == Player::None
            {
                return false;
            }

            if self.state[idx][idx] != initial
            {
                return false;
            }
        }

        return true;
    }


    fn is_backward_diagional_winner(&self) -> bool
    {
        let initial = self.state[0][BOARDSIZE-1];

        for idx in 0..BOARDSIZE
        {
            if self.state[idx][BOARDSIZE-idx-1] == Player::None
            {
                return false;
            }

            if self.state[idx][BOARDSIZE-idx-1] != initial
            {
                return false;
            }
        }

        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_winner_zero() {
        let mut board = Board::new();

        assert!(!board.is_winner());
    }

    #[test]
    fn row_is_winner_zero() {
        let mut board = Board::new();

        assert!(!board.is_row_winner(0));
    }

    #[test]
    fn col_is_winner_zero() {
        let mut board = Board::new();

        assert!(!board.is_col_winner(0));
    }

    #[test]
    fn row_is_winner() {
        let mut board = Board::new();

        board.state[0][0] = 1;
        board.state[0][1] = 1;
        board.state[0][2] = 1;

        assert!(board.is_row_winner(0));
    }

    #[test]
    fn row_is_winner_neg() {
        let mut board = Board::new();

        board.state[0][0] = 1;
        board.state[0][1] = 2;
        board.state[0][2] = 1;

        assert!(!board.is_row_winner(0));
    }

    #[test]
    fn col_is_winner() {
        let mut board = Board::new();

        board.state[0][0] = 1;
        board.state[1][0] = 1;
        board.state[2][0] = 1;

        assert!(board.is_col_winner(0));
    }

    #[test]
    fn forward_diag_is_winner() {
        let mut board = Board::new();

        board.state[0][0] = 1;
        board.state[1][1] = 1;
        board.state[2][2] = 1;

        assert!(board.is_forward_diagional_winner());
    }

    #[test]
    fn backward_diag_is_winner_zero() {
        let mut board = Board::new();

        assert!(!board.is_backward_diagional_winner());
    }

    #[test]
    fn backward_diag_is_winner() {
        let mut board = Board::new();

        board.state[0][2] = 1;
        board.state[1][1] = 1;
        board.state[2][0] = 1;

        assert!(board.is_backward_diagional_winner());
    }
}