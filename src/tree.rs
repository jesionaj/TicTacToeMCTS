pub enum Result
{
    P1Win,
    P2Win,
    Draw
}

pub struct Node
{
    pub children: [Option<Box<Node>>; 9],
    pub p1_win_count: usize,
    pub p2_win_count: usize,
    pub draw_count: usize
}

impl Node
{
    pub fn create_empty_node() -> Node
    {
        Node{ children: [None, None, None, None, None, None, None, None, None], p1_win_count: 0, p2_win_count: 0, draw_count: 0 }
    }

    pub fn create_child(&mut self, idx: usize)
    {
        if self.children[idx].is_none()
        {
            let new_node = Node::create_empty_node();
            self.children[idx] = Some(Box::new(new_node));
        }
    }

    pub fn add_move_list(&mut self, move_list: &mut Vec<usize>, result: Result)
    {
        match result
        {
            Result::P1Win => { self.p1_win_count += 1},
            Result::P2Win => { self.p2_win_count += 1},
            Result::Draw =>  { self.draw_count   += 1}
        }

        if move_list.len() > 0
        {
            let next = move_list.pop().unwrap();

            if self.children[next].is_none()
            {
                self.create_child(next);
            }

            match self.children[next]
            {
                Some(ref mut x) => { x.add_move_list(move_list, result) },
                None => {},
            }
        }

    }

    pub fn print_wins(&self)
    {
        println!("P1 Wins: {} P2 Wins: {} Draws: {} - Total {}", self.p1_win_count, self.p2_win_count, self.draw_count, self.p1_win_count + self.p2_win_count + self.draw_count);
    }
}