pub struct Bits {
    pub is_white: u64, // 1L << 62
    captured: u64, // 1L << 63; sign
                // bit speeds
                // tests
    pub initial_bot: u64,
    pub initial_top: u64,
    pub top_row: u64,
    pub bottom_row: u64,
    pub left_col: u64,
    pub right_col: u64,
    pub diagonal: u64,
    pub on_board: u64,
    pub center: u64,

    pub shift_vertical: u32, // 10
    pub shift_horizontal: u32, // 1
    pub shift_slant: u32, // 11
    pub shift_backslant: u32, // 9
                                    
    ones: u64,
    twos: u64,
    fours: u32,
}

pub const BITS: Bits = Bits {
    // BINARY Annotation
    // 5 rows (numbers) 9 columns (letters)
    // C(capt) W(White) x, numbers(Empty)    
    //             CWxxxxxxxxxxxx5abcdefghi4abcdefghi3abcdefghi2abcdefghi1abcdefghi
    is_white:    0b0100000000000000000000000000000000000000000000000000000000000000,
    captured:    0b1000000000000000000000000000000000000000000000000000000000000000, 
    initial_bot: 0b0000000000000000000000000000000000001010010101111111110111111111,
    initial_top: 0b0000000000000001111111110111111111010100101000000000000000000000,
    top_row:     0b0000000000000001111111110000000000000000000000000000000000000000,
    bottom_row:  0b0000000000000000000000000000000000000000000000000000000111111111,
    left_col:    0b0000000000000001000000000100000000010000000001000000000100000000,
    right_col:   0b0000000000000000000000010000000001000000000100000000010000000001,
    diagonal:    0b0000000000000001010101010010101010010101010100101010100101010101,
    on_board:    0b0000000000000001111111110111111111011111111101111111110111111111,
    center:      0b0000000000000000000000000000000000000111110000000000000000000000,
                                    
    ones:        0b0101010101010101010101010101010101010101010101010101010101010101,
    twos:        0b0011001100110011001100110011001100110011001100110011001100110011,
    fours:       0b00001111000011110000111100001111,

    shift_vertical: 0b1010, // 10
    shift_horizontal: 0b1, // 1
    shift_slant: 0b1011, // 11
    shift_backslant: 0b1001, // 9

};

impl Bits {

    pub fn new() -> Self {
        BITS
    }

    fn count(&self, set: u64) -> u32 {
        let mut set = set;
        set -= (set >> 1) & self.ones;
        set = (set & self.twos) + ((set >> 0b10) & self.twos);
        let mut result = (set + (set >> 4)) & 0b100000;
        result += result >> 8;
        result += result >> 16;
        result += result >> 32;
        result as u32 & 0x0000007F
    }

    fn at(row: i32, col: i32) -> u64 {
        1u64 << (10 * (4 - row) + (8 - col))
    }
    
    pub fn last_bit(bitboard: u64) -> u64 {
        bitboard & (!bitboard + 1)
    }
        
}
