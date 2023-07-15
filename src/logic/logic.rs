mod logic {
    pub mod engine {
        // use crate::logic::board::Bits;
        // use crate::logic::board::Board;

        pub struct MoveGenerator {
            board: Board,
            stored_from: i64,
            stored_to: i64,
            moves_v: i64,
            moves_h: i64,
            moves_s: i64,
            moves_b: i64,
            move_set_index: i32,
            set: i64,
            capture_type: i32,
            made_capture: bool,
            shift: i32,
        }

        impl MoveGenerator {
            const CAPTURE_FORWARD: i32 = 0;
            const CAPTURE_BACKWARD: i32 = 1;
            const NO_CAPTURE: i32 = 2;
            const PASS: i32 = 3;
            const NO_MORE_MOVES: i32 = 4;

            pub fn new(board: Board) -> MoveGenerator {
                MoveGenerator {
                    board,
                    stored_from: 0,
                    stored_to: 0,
                    moves_v: 0,
                    moves_h: 0,
                    moves_s: 0,
                    moves_b: 0,
                    move_set_index: -1,
                    set: 0,
                    capture_type: MoveGenerator::CAPTURE_FORWARD,
                    made_capture: false,
                    shift: 0,
                }
            }

            fn find_next_set(&mut self) {
                let from = self.stored_from;
                let to = self.stored_to;
                let target = self.board.opponent_pieces;

                self.move_set_index += 1;

                match self.move_set_index {
                    0 => {
                        self.capture_type = MoveGenerator::CAPTURE_FORWARD;
                        self.moves_v = (from & (to >> Bits::SHIFT_VERTICAL))
                            | (to & (from >> Bits::SHIFT_VERTICAL));
                        self.set = self.moves_v & (target >> (2 * Bits::SHIFT_VERTICAL));

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_VERTICAL;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    1 => {
                        self.moves_h = (from & (to >> Bits::SHIFT_HORIZONTAL))
                            | (to & (from >> Bits::SHIFT_HORIZONTAL));
                        self.set = self.moves_h & (target >> (2 * Bits::SHIFT_HORIZONTAL));

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_HORIZONTAL;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    2 => {
                        self.stored_from = from & Bits::DIAGONAL;
                        self.moves_s = (from & (to >> Bits::SHIFT_SLANT))
                            | (to & (from >> Bits::SHIFT_SLANT));
                        self.set = self.moves_s & (target >> (2 * Bits::SHIFT_SLANT));

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_SLANT;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    3 => {
                        self.moves_b = (from & (to >> Bits::SHIFT_BACKSLANT))
                            | (to & (from >> Bits::SHIFT_BACKSLANT));
                        self.set = self.moves_b & (target >> (2 * Bits::SHIFT_BACKSLANT));

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_BACKSLANT;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    4 => {
                        self.capture_type = MoveGenerator::CAPTURE_BACKWARD;
                        self.set = self.moves_v & (target << Bits::SHIFT_VERTICAL);

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_VERTICAL;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    5 => {
                        self.set = self.moves_h & (target << Bits::SHIFT_HORIZONTAL);

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_HORIZONTAL;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    6 => {
                        self.set = self.moves_s & (target << Bits::SHIFT_SLANT);

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_SLANT;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    7 => {
                        self.set = self.moves_b & (target << Bits::SHIFT_BACKSLANT);

                        if self.set != 0 {
                            self.shift = Bits::SHIFT_BACKSLANT;
                            self.made_capture = true;
                            return;
                        }

                        self.move_set_index += 1;
                    }
                    8 => {
                        if self.board.mid_capture() {
                            self.capture_type = MoveGenerator::PASS;
                            self.move_set_index = 11;
                            self.set = 1;
                            return;
                        } else if self.made_capture {
                            self.move_set_index = 11;
                            self.capture_type = MoveGenerator::NO_MORE_MOVES;
                            return;
                        }
                        self.capture_type = MoveGenerator::NO_CAPTURE;
                        self.set = self.moves_v;
                        if self.set != 0 {
                            self.shift = Bits::SHIFT_VERTICAL;
                            return;
                        }
                        self.move_set_index += 1;
                    }
                    9 => {
                        self.set = self.moves_h;
                        if self.set != 0 {
                            self.shift = Bits::SHIFT_HORIZONTAL;
                            return;
                        }
                        self.move_set_index += 1;
                    }
                    10 => {
                        self.set = self.moves_s;
                        if self.set != 0 {
                            self.shift = Bits::SHIFT_SLANT;
                            return;
                        }
                        self.move_set_index += 1;
                    }
                    11 => {
                        self.set = self.moves_b;
                        if self.set != 0 {
                            self.shift = Bits::SHIFT_BACKSLANT;
                            return;
                        }
                        self.move_set_index += 1;
                    }
                    12 => {
                        self.move_set_index -= 1;
                        self.capture_type = MoveGenerator::NO_MORE_MOVES;
                        return;
                    }
                    _ => panic!("Invalid move set index"),
                }
            }

            pub fn has_capture(&mut self) -> bool {
                if self.set == 0 {
                    find_next_set();
                }
                self.capture_type == MoveGenerator::CAPTURE_FORWARD || self.capture_type == MoveGenerator::CAPTURE_BACKWARD
            }

            pub fn reset(&mut self, b: Board) {
                self.board = b;
                self.move_set_index = -1;
                self.set = 0;
                self.made_capture = false;

                // Find positions we can move from and to.
                // At start of move they are just occupied and empty positions, but in
                // midCapture we restrict from to the piece just moved and to to places it can
                // legally go.
                let my_pieces = b.my_pieces;

                // Narrow down target positions for midcapture moves to avoid our
                // previous positions
                if b.mid_capture() {
                    let mov = my_pieces ^ b.previous_position.my_pieces;
                    self.stored_from = my_pieces & mov; // Only allow same piece to move

                    let mov_shifted_vertical = mov << Bits::SHIFT_VERTICAL;
                    let mov_shifted_horizontal = mov << Bits::SHIFT_HORIZONTAL;
                    let mov_shifted_slant = mov << Bits::SHIFT_SLANT;
                    let mov_shifted_backslant = mov << Bits::SHIFT_BACKSLANT;

                    let mov = if (mov & mov_shifted_vertical) != 0 {
                        mov_shifted_vertical | (mov >> Bits::SHIFT_VERTICAL)
                    } else if (mov & mov_shifted_horizontal) != 0 {
                        mov_shifted_horizontal | (mov >> Bits::SHIFT_HORIZONTAL)
                    } else if (mov & mov_shifted_slant) != 0 {
                        mov_shifted_slant | (mov >> Bits::SHIFT_SLANT)
                    } else {
                        mov_shifted_backslant | (mov >> Bits::SHIFT_BACKSLANT)
                    };

                    self.stored_to = Bits::ON_BOARD
                        & !(move | my_pieces | b.opponent_pieces | b.already_visited);
                } else {
                    self.stored_from = my_pieces;
                    self.stored_to = Bits::ON_BOARD & !(my_pieces | b.opponent_pieces);
                }
            }

            pub fn has_more_elements(&mut self) -> bool {
                if self.set == 0 {
                    self.find_next_set();
                }
                self.capture_type != MoveGenerator::NO_MORE_MOVES
            }

            pub fn next_element(&mut self) -> i64 {
                if self.set == 0 {
                    self.find_next_set();
                }
                let bit = Bits::last_bit(self.set);
                self.set ^= bit;

                match self.capture_type {
                    MoveGenerator::CAPTURE_FORWARD => {
                        let mut retval = bit | (bit << self.shift);
                        let mut bit_shifted = bit << (2 * self.shift);
                        while (bit_shifted & self.board.opponent_pieces) != 0 {
                            retval |= bit_shifted;
                            bit_shifted <<= self.shift;
                        }
                        retval
                    }
                    MoveGenerator::CAPTURE_BACKWARD => {
                        let mut retval = bit | (bit << self.shift);
                        let mut bit_shifted = bit >> self.shift;
                        while (bit_shifted & self.board.opponent_pieces) != 0 {
                            retval |= bit_shifted;
                            bit_shifted >>= self.shift;
                        }
                        retval
                    }
                    MoveGenerator::NO_CAPTURE => bit | (bit << self.shift),
                    MoveGenerator::PASS => 0,
                    MoveGenerator::NO_MORE_MOVES => -1,
                    _ => panic!("Invalid capture type"),
                }
            }

            // Find an arbitrary move in a position
            pub fn arbitrary_move(board: Board) -> Board {
                let mut i = MoveGenerator::arbitrary_move_index; // remember which move we want
                MoveGenerator::arbitrary_move_index += 1; // and next time take the following move instead

                let mut move_generator = MoveGenerator::new(board);
                while move_generator.has_more_elements() {
                    let mv = move_generator.next_element();
                    if i < 0 {
                        return Board::new(board, mv);
                    }
                    i -= 1;
                }
                MoveGenerator::arbitrary_move_index = 1; // ran out, had to return 0'th, return 1'st next time
                Board::new(board, MoveGenerator::new(board).next_element())
            }
        }
    }
}
