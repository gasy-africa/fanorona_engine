

struct Bits {
    is_white: u64, // 1L << 62
    captured: u64, // 1L << 63; sign
                   // bit speeds
                   // tests
}

const BITS_STRUCT: Bits = Bits {
    is_white: 0b0100000000000000000000000000000000000000000000000000000000000000,
    captured: 0b1000000000000000000000000000000000000000000000000000000000000000, 
};

impl Bits {

	fn count(set: u64) -> u32 {
		// set -= (set >>> 1) & ONES;
		// set = (set & TWOS) + ((set >>> 0b10) & TWOS);
		// int result = (int) set + (int) (set >>> 0b100000);
		// (((result & FOURS) + ((result >>> 0b100) & FOURS)) * 0b1000000010000000100000001) >>> 0b11000;
        0
	}    
    
}

