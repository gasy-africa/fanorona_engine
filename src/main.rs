
mod bits;
use bits::bits::Bits;
use bits::bits_format;

fn main() {
    println!("# Fanorona Engine! \n");
    let bits: Bits = Bits::new();
    println!(" - [ ] bits.initial_bot\n");
    bits_format::display(bits.initial_bot);
    println!(" - [ ] bits.initial_top\n");
    bits_format::display(bits.initial_top);
    println!(" - [ ] bits.top_row\n");
    bits_format::display(bits.top_row);
    println!(" - [ ] bits.bottom_row\n");
    bits_format::display(bits.bottom_row);
    println!(" - [ ] bits.left_col\n");
    bits_format::display(bits.left_col);
    println!(" - [ ] bits.right_col\n");
    bits_format::display(bits.right_col);
    println!(" - [ ] bits.diagonal\n");
    bits_format::display(bits.diagonal);
    println!(" - [ ] bits.on_board\n");
    bits_format::display(bits.on_board);
    println!(" - [ ] bits.center\n");
    bits_format::display(bits.center);

}
