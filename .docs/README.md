

> From Scala to Rust

```scala
	private final void findNextSet() {
		long from = storedFrom;
		long to = storedTo;
		long target = board.opponentPieces;
		switch (++moveSetIndex) {
		// CASES 0-7: CAPTURES
		case 0: // CAPTURE FORWARD VERTICALLY OR BACKWARD -VERTICALLY
			captureType = CAPTURE_FORWARD;
			movesV = (from & (to >>> Bits.SHIFT_VERTICAL))
					| (to & (from >>> Bits.SHIFT_VERTICAL));
            set = (movesV & (target >>> 2 * Bits.SHIFT_VERTICAL));
			if ( set != 0) {
				shift = Bits.SHIFT_VERTICAL;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 1: // CAPTURE FORWARD HORIZONTALLY OR BACKWARD -HORIZONTALLY
			movesH = (from & (to >>> Bits.SHIFT_HORIZONTAL))
					| (to & (from >>> Bits.SHIFT_HORIZONTAL));
            set = (movesH & (target >>> 2 * Bits.SHIFT_HORIZONTAL));
			if ( set != 0) {
				shift = Bits.SHIFT_HORIZONTAL;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 2: // CAPTURE FORWARD SLANTLY OR BACKWARD -SLANTLY
			storedFrom = (from &= Bits.DIAGONAL);
			movesS = (from & (to >>> Bits.SHIFT_SLANT))
					| (to & (from >>> Bits.SHIFT_SLANT));
            set = (movesS & (target >>> 2 * Bits.SHIFT_SLANT));
			if ( set != 0) {
				shift = Bits.SHIFT_SLANT;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 3: // CAPTURE FORWARD BACKSLANTLY OR BACKWARD -BACKSLANTLY
			movesB = (from & (to >>> Bits.SHIFT_BACKSLANT))
					| (to & (from >>> Bits.SHIFT_BACKSLANT));
            set = (movesB & (target >>> 2 * Bits.SHIFT_BACKSLANT));
			if ( set != 0) {
				shift = Bits.SHIFT_BACKSLANT;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 4: // CAPTURE FORWARD -VERTICALLY OR BACKWARD VERTICALLY
			captureType = CAPTURE_BACKWARD;
            set = (movesV & (target << Bits.SHIFT_VERTICAL));
			if ( set != 0) {
				shift = Bits.SHIFT_VERTICAL;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 5: // CAPTURE FORWARD -HORIZONTALLY OR BACKWARD HORIZONTALLY
            set = (movesH & (target << Bits.SHIFT_HORIZONTAL));
			if ( set != 0) {
				shift = Bits.SHIFT_HORIZONTAL;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 6: // CAPTURE FORWARD -SLANTLY OR BACKWARD SLANTLY
            set = (movesS & (target << Bits.SHIFT_SLANT));
			if ( set != 0) {
				shift = Bits.SHIFT_SLANT;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...
		case 7: // CAPTURE FORWARD -BACKSLANTLY OR BACKWARD BACKSLANTLY
            set = (movesB & (target << Bits.SHIFT_BACKSLANT));
			if ( set != 0) {
				shift = Bits.SHIFT_BACKSLANT;
				madeCapture = true;
				return;
			}
			++moveSetIndex; // fall into...

			// CASES 8-11: SHUFFLES
		case 8: // VERTICAL SHUFFLE OR PASS
			if (board.midCapture()) { // illegal to shuffle?
				captureType = PASS;
				moveSetIndex = 11; // set up so next call to findset runs into
									// case 12
				set = 1; // return a set with one move in it
				return;
			} else if (madeCapture) { // capture was forced?
				moveSetIndex = 11; // move to case 12, end of moves
				captureType = NO_MORE_MOVES;
				return;
			}
			captureType = NO_CAPTURE;
            set = movesV;
			if ( set != 0) {
				shift = Bits.SHIFT_VERTICAL;
				return;
			}
			++moveSetIndex; // fall into...
		case 9: // HORIZONTAL SHUFFLE
            set = movesH;
			if ( set != 0) {
				shift = Bits.SHIFT_HORIZONTAL;
				return;
			}
			++moveSetIndex; // fall into...
		case 10: // SLANT SHUFFLE
            set = movesS;
			if ( set != 0) {
				shift = Bits.SHIFT_SLANT;
				return;
			}
			++moveSetIndex; // fall into...
		case 11: // BACKSLANT SHUFFLE
            set = movesB;
			if ( set != 0) {
				shift = Bits.SHIFT_BACKSLANT;
				return;
			}
			++moveSetIndex; // fall into...

			// CASE 12: RAN OUT OF SHUFFLES
		case 12:
			moveSetIndex--; // stay in this case and always return zero
			captureType = NO_MORE_MOVES;
			return;

		default:
			throw new IllegalArgumentException();
		}
	}
```
