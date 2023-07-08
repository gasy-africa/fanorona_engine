mod bits;
mod bits_format;

use bits::Bits;

fn main() {
    println!("# Fanorona Engine! \n");
    let bits: Bits = Bits::new();
    println!(" - [ ] bits.initial_bot");
    bits_format::display(bits.initial_bot);
    println!(" - [ ] bits.initial_top");
    bits_format::display(bits.initial_top);
    println!(" - [ ] bits.top_row");
    bits_format::display(bits.top_row);
    println!(" - [ ] bits.bottom_row");
    bits_format::display(bits.bottom_row);
    println!(" - [ ] bits.left_col");
    bits_format::display(bits.left_col);
    println!(" - [ ] bits.right_col");
    bits_format::display(bits.right_col);
    println!(" - [ ] bits.diagonal");
    bits_format::display(bits.diagonal);
    println!(" - [ ] bits.on_board");
    bits_format::display(bits.on_board);
    println!(" - [ ] bits.center");
    bits_format::display(bits.center);

}
