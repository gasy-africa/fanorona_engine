mod bits;
mod bits_format;

use bits::Bits;

fn main() {
    println!("Fanorona Engine!");
    let bits: Bits = Bits::new();
    bits_format::display(bits.initial_top);
}
