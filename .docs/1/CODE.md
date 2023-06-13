```rust

fn find_next_set(&mut self) {
    let from = self.stored_from;
    let to = self.stored_to;
    let target = self.board.opponent_pieces;
    
    self.move_set_index += 1;

    match self.move_set_index {
        // CASES 0-7: CAPTURES
        0 => { // CAPTURE FORWARD VERTICALLY OR BACKWARD -VERTICALLY
            self.capture_type = CaptureType::CaptureForward;
            self.moves_v = (from & (to >> Bits::SHIFT_VERTICAL))
                | (to & (from >> Bits::SHIFT_VERTICAL));
            self.set = (self.moves_v & (target >> (2 * Bits::SHIFT_VERTICAL)));
            if self.set != 0 {
                self.shift = Bits::SHIFT_VERTICAL;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        1 => { // CAPTURE FORWARD HORIZONTALLY OR BACKWARD -HORIZONTALLY
            self.moves_h = (from & (to >> Bits::SHIFT_HORIZONTAL))
                | (to & (from >> Bits::SHIFT_HORIZONTAL));
            self.set = (self.moves_h & (target >> (2 * Bits::SHIFT_HORIZONTAL)));
            if self.set != 0 {
                self.shift = Bits::SHIFT_HORIZONTAL;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        2 => { // CAPTURE FORWARD SLANTLY OR BACKWARD -SLANTLY
            self.stored_from = (from & Bits::DIAGONAL);
            self.moves_s = (from & (to >> Bits::SHIFT_SLANT))
                | (to & (from >> Bits::SHIFT_SLANT));
            self.set = (self.moves_s & (target >> (2 * Bits::SHIFT_SLANT)));
            if self.set != 0 {
                self.shift = Bits::SHIFT_SLANT;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        3 => { // CAPTURE FORWARD BACKSLANTLY OR BACKWARD -BACKSLANTLY
            self.moves_b = (from & (to >> Bits::SHIFT_BACKSLANT))
                | (to & (from >> Bits::SHIFT_BACKSLANT));
            self.set = (self.moves_b & (target >> (2 * Bits::SHIFT_BACKSLANT)));
            if self.set != 0 {
                self.shift = Bits::SHIFT_BACKSLANT;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        4 => { // CAPTURE FORWARD -VERTICALLY OR BACKWARD VERTICALLY
            self.capture_type = CaptureType::CaptureBackward;
            self.set = (self.moves_v & (target << Bits::SHIFT_VERTICAL));
            if self.set != 0 {
                self.shift = Bits::SHIFT_VERTICAL;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        5 => { // CAPTURE FORWARD -HORIZONTALLY OR BACKWARD HORIZONTALLY
            self.set = (self.moves_h & (target << Bits::SHIFT_HORIZONTAL));
            if self.set != 0 {
                self.shift = Bits::SHIFT_HORIZONTAL;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        6 => { // CAPTURE FORWARD -SLANTLY OR BACKWARD SLANTLY
            self.set = (self.moves_s & (target << Bits::SHIFT_SLANT));
            if self.set != 0 {
                self.shift = Bits::SHIFT_SLANT;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        7 => { // CAPTURE FORWARD -BACKSLANTLY OR BACKWARD BACKSLANTLY
            self.set = (self.moves_b & (target << Bits::SHIFT_BACKSLANT));
            if self.set != 0 {
                self.shift = Bits::SHIFT_BACKSLANT;
                self.made_capture = true;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        // CASES 8-11: SHUFFLES
        8 => { // VERTICAL SHUFFLE OR PASS
            if self.board.mid_capture() { // illegal to shuffle?
                self.capture_type = CaptureType::Pass;
                self.move_set_index = 11; // set up so next call to findset runs into case 12
                self.set = 1; // return a set with one move in it
                return;
            } else if self.made_capture { // capture was forced?
                self.move_set_index = 11; // move to case 12, end of moves
                self.capture_type = CaptureType::NoMoreMoves;
                return;
            }
            self.capture_type = CaptureType::NoCapture;
            self.set = self.moves_v;
            if self.set != 0 {
                self.shift = Bits::SHIFT_VERTICAL;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        9 => { // HORIZONTAL SHUFFLE
            self.set = self.moves_h;
            if self.set != 0 {
                self.shift = Bits::SHIFT_HORIZONTAL;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        10 => { // SLANT SHUFFLE
            self.set = self.moves_s;
            if self.set != 0 {
                self.shift = Bits::SHIFT_SLANT;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        11 => { // BACKSLANT SHUFFLE
            self.set = self.moves_b;
            if self.set != 0 {
                self.shift = Bits::SHIFT_BACKSLANT;
                return;
            }
            self.move_set_index += 1; // fall into...
        },
        // CASE 12: RAN OUT OF SHUFFLES
        12 => {
            self.move_set_index -= 1; // stay in this case and always return zero
            self.capture_type = CaptureType::NoMoreMoves;
            return;
        },
        _ => {
            panic!("Invalid move set index");
        },
    }
}


```
