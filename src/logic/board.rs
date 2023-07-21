use crate::bits::bits::BITS;

use crate::logic::engine::MoveGenerator;

#[derive(Debug)]
pub struct Board {
    my_pieces: u64,
    opponent_pieces: u64,
    already_visited: u64,
    sequence_number: u32,
    child: Option<Box<Board>>,
}

impl Board {
    fn new() -> Board {
        Board {
            my_pieces: 0,
            opponent_pieces: 0,
            already_visited: 0,
            sequence_number: 0,
            child: None
        }
    }

    fn white_to_move(&self) -> bool {
        (self.my_pieces & BITS.is_white) != 0
    }

    fn mid_capture(&self) -> bool {
        self.my_pieces < 0
    }

    fn was_pass(&self) -> bool {
        (self.my_pieces ^ self.opponent_pieces) < 0
    }

    fn was_shuffle(&self) -> bool {
        self.opponent_pieces >= 0
    }

    fn game_over(&self) -> bool {
        (self.my_pieces & BITS.on_board) == 0
            || (self.opponent_pieces & BITS.on_board) == 0
            || !(MoveGenerator::new(*self)).has_more_elements()
    }

    fn white_wins(&self) -> bool {
        !(self.white_to_move() ^ ((self.opponent_pieces & BITS.on_board) == 0))
    }

    fn set_child(&mut self, mv: i64) {
        if self.child.is_none() {
            self.child = Some(Rc::new(RefCell::new(Board::new())));
            // if self.collect_extra_statistics {
                // self.board_cons_count += 1;
            // }
        }

        let captures = self.opponent_pieces & mv;
        if captures != 0 {
            let child = self.child.as_mut().unwrap();
            let mut child_ref = child.borrow_mut();
            child_ref.opponent_pieces = (self.opponent_pieces ^ captures) | Bits::CAPTURED;
            let mv = mv ^ captures;
            child_ref.already_visited = self.already_visited | mv;
            child_ref.my_pieces = (self.my_pieces ^ mv) | Bits::CAPTURED;
        } else {
            let child = self.child.as_mut().unwrap();
            let mut child_ref = child.borrow_mut();
            child_ref.opponent_pieces = self.my_pieces ^ mv;
            child_ref.my_pieces = self.opponent_pieces & !Bits::CAPTURED;
            child_ref.already_visited = 0;
        }

        child.best_move = -1;
    }
}
