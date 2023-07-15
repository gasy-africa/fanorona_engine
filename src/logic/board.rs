
mod logic {

    #[derive(Debug)]
    struct Board {
        pvs: bool,
        iid: bool,
        iid_ply: i32,
        iid_limit: i32,
        min_hash_depth: i32,
        ply: i32,
        forced_move_extension: i32,
        forced_capture_extension: i32,
        endgame_capture_extension: i32,
        forced_endgame_capture: i32,
        multiple_capture_extension: i32,
        early_pass_extension: i32,
        won_position: i32,
        decrementable: i32,
        ply_decrement: i32,
        hash: Hash,
        previous_position: Option<Rc<RefCell<Board>>>,
        my_pieces: i64,
        opponent_pieces: i64,
        already_visited: i64,
        sequence_number: i32,
        evaluation: i32,
        best_move: i64,
        principal_variation: Option<Rc<RefCell<Board>>>,
        child: Option<Rc<RefCell<Board>>>,
        move_generator: Option<MoveGenerator>,
        forced: bool,
        has_principal_variation: bool,
        eval_upper_bound: i32,
        eval_lower_bound: i32,
        eval_exact: i32,
        node_count: i64,
        leaf_count: i64,
        collect_extra_statistics: bool,
        endgame_eval_count: i64,
        board_cons_count: i64,
        move_gen_cons_count: i64,
        pv_change_count: i64,
    }

    impl Board {
        fn new() -> Board {
            Board {
                pvs: true,
                iid: true,
                iid_ply: 25,
                iid_limit: 55,
                min_hash_depth: 15,
                ply: 10,
                forced_move_extension: 5,
                forced_capture_extension: 10,
                endgame_capture_extension: 5,
                forced_endgame_capture: 10,
                multiple_capture_extension: 7,
                early_pass_extension: 10,
                won_position: 10000,
                decrementable: 5000,
                ply_decrement: 1,
                hash: Hash::new(32768),
                previous_position: None,
                my_pieces: 0,
                opponent_pieces: 0,
                already_visited: 0,
                sequence_number: 0,
                evaluation: 0,
                best_move: -1,
                principal_variation: None,
                child: None,
                move_generator: None,
                forced: false,
                has_principal_variation: false,
                eval_upper_bound: 0,
                eval_lower_bound: 1,
                eval_exact: 2,
                node_count: 0,
                leaf_count: 0,
                collect_extra_statistics: false,
                endgame_eval_count: 0,
                board_cons_count: 0,
                move_gen_cons_count: 0,
                pv_change_count: 0,
            }
        }

        fn white_to_move(&self) -> bool {
            (self.my_pieces & Bits::IS_WHITE) != 0
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
            (self.my_pieces & Bits::ON_BOARD) == 0
                || (self.opponent_pieces & Bits::ON_BOARD) == 0
                || !(MoveGenerator::new(*self)).has_more_elements()
        }

        fn white_wins(&self) -> bool {
            !(self.white_to_move() ^ ((self.opponent_pieces & Bits::ON_BOARD) == 0))
        }

        fn set_child(&mut self, mv: i64) {
            if self.child.is_none() {
                self.child = Some(Rc::new(RefCell::new(Board::new())));
                if self.collect_extra_statistics {
                    self.board_cons_count += 1;
                }
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

}