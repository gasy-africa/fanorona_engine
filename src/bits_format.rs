use std::fmt;

struct DisplayRow(Vec<char>);
    
impl fmt::Display for DisplayRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let new_vector: String = self.0
            .iter()
            .enumerate()
            .flat_map(|(_, &element)| {
                match element {
                    '1' => vec!['|',':','o',':'],
                    '0' => vec!['|',':','x',':'],
                      _ => vec!['|',' ',' ',' '],
                }
            })
            .collect();

        write!(f, "{}|", new_vector)
    }
}

fn fill64(bin: &str) -> String {
    format!("{:0>64}L", bin)
}

fn display_emoji(number: usize) -> &'static str {
    match number {
        0 => ":zero:",
        1 => ":one:",
        2 => ":two:",
        3 => ":three:",
        4 => ":four:",
        5 => ":five:",
        6 => ":six:",
        7 => ":seven:",
        8 => ":eight:",
        9 => ":nine:",
        _ => "",
    }
}

pub fn display(bin: u64) -> Vec<Vec<char>> {
    display_string(&fill64(&format!("{:b}", bin)))
}

fn display_string(bin: &str) -> Vec<Vec<char>> {
    let control = bin.chars().take(4).collect::<Vec<_>>();
    let one = bin.chars().skip(55).take(9).collect::<Vec<_>>();
    let two = bin.chars().skip(45).take(9).collect::<Vec<_>>();
    let three = bin.chars().skip(35).take(9).collect::<Vec<_>>();
    let four = bin.chars().skip(25).take(9).collect::<Vec<_>>();
    let five = bin.chars().skip(15).take(9).collect::<Vec<_>>();

    let mut board = Vec::new();
    board.push(control);
    board.push(one);
    board.push(two);
    board.push(three);
    board.push(four);
    board.push(five);

    println!("|`~`     |`A`|`B`|`C`|`D`|`E`|`F`|`G`|`H`|`I`|");
    println!("|--------|---|---|---|---|---|---|---|---|---|");
    for i in (1..=5).rev() {
        let row = format!("|{:7} {}", display_emoji(i), DisplayRow(board[i].clone()));

        println!("{}", row);
    }

    board
}
