
mod bits;
mod bits_format;

use bits::bits::Bits;

#[derive(Debug, PartialEq, Default)]
struct ElementSet {
    move_set_index: u32,
    shift: u32,
    capture_type: Capture,
    made_capture: bool,
    set: u64,
}

#[derive(Debug, PartialEq)]
enum Capture {
    FORWARD,
    BACKWARD,
    NO,
    PASS,
    NO_MORE_MOVES,
}

impl Default for Capture {
    fn default() -> Self {
        Capture::FORWARD
    }
}

fn find_next_set(
    ingress_move_set_index: u32,
    stored_from: u64,
    stored_to: u64,
    opponent_pieces: u64,
    es: ElementSet,
) -> ElementSet {
    let from = stored_from;
    let to = stored_to;

    let target = opponent_pieces;

    let move_set_index: u32 = ingress_move_set_index + 1;

    let bits: bits::bits::Bits = Bits::new();

    match move_set_index {
        0 => {
            let shift = bits.shift_vertical;
            let capture_type = Capture::FORWARD;
            let capture_type2 = Capture::FORWARD;
            let moves_v = (from & (to >> shift)) | (to & (from >> shift));
            let set = moves_v & (target >> (2 * shift));
            if set != 0 {
                ElementSet {
                    move_set_index: move_set_index,
                    shift,
                    capture_type,
                    made_capture: true,
                    set,
                };
            } else {
                return find_next_set(
                    move_set_index + 1,
                    stored_from,
                    stored_to,
                    opponent_pieces,
                    es,
                );
            }
            ElementSet {
                move_set_index: move_set_index + 1,
                shift,
                capture_type: capture_type2,
                made_capture: true,
                set,
            }

        }
        _ => es,
    }
}

fn next_element(
    ingress_move_set_index: u32,
    ingress_set: u64,
    shift: i32,
    capture: Capture,
    stored_from: u64,
    stored_to: u64,
    opponent_pieces: u64,
) -> (u64, u64) {
    if ingress_set == 0 {
        find_next_set(
            ingress_move_set_index,
            stored_from,
            stored_to,
            opponent_pieces,
            ElementSet::default()
        );
    }

    let mut bit = Bits::last_bit(ingress_set);
    let set = ingress_set ^ bit;
    match capture {
        Capture::FORWARD => {
            let mut retval = bit | (bit << shift);
            bit <<= 2 * shift;
            while bit & opponent_pieces != 0 {
                retval |= bit;
                bit <<= shift;
            }
            (set, retval)
        }
        Capture::BACKWARD => {
            let mut retval1 = bit | (bit << shift);
            bit >>= shift;
            while bit & opponent_pieces != 0 {
                retval1 |= bit;
                bit >>= shift;
            }
            (set, retval1)
        }
        Capture::NO => (set, bit | (bit << shift)),
        Capture::PASS => (set, 0),
        Capture::NO_MORE_MOVES => (set, u64::MAX),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_next_set() {
        let ingress_move_set_index = 0;
        let stored_from = 123;
        let stored_to = 456;
        let opponent_pieces: u64 = 789;
        let es = ElementSet {
            move_set_index: 1,
            shift: 2,
            capture_type: Capture::FORWARD,
            made_capture: true,
            set: 10,
        };

        let result = find_next_set(ingress_move_set_index, stored_from, stored_to, opponent_pieces, es);

        let expected = ElementSet {
            move_set_index: 2,
            shift: 2,
            capture_type: Capture::FORWARD,
            made_capture: true,
            set: 20,
        };

        println!(" - [ ] bits.find_next_set\n");
        bits_format::display(stored_from);    

        assert_eq!(result, expected);
    }

    #[test]
    fn test_next_element() {
        let ingress_move_set_index = 0;
        let ingress_set = 123;
        let shift = 2;
        let capture = Capture::FORWARD;
        let stored_from = 456;
        let stored_to = 789;
        let opponent_pieces = 10;

        let result = next_element(ingress_move_set_index, ingress_set, shift, capture, stored_from, stored_to, opponent_pieces);

        let expected = (123, 456);

        assert_eq!(result, expected);
    }
}


