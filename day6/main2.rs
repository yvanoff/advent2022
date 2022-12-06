use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut char_pos = 0;
    let mut validated = String::new();
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                for c in ip.chars() {
                    if (!validated.contains(c)) && (validated.len() == 13) {
                        char_pos += 1;
                        break;
                    } else {
                        char_pos += 1;
                        if validated.contains(c) {
                            let mut pop_c = validated.remove(0);
                            while pop_c != c {
                                pop_c = validated.remove(0);
                            }
                        }
                        validated.push(c);
                        if validated.len() > 13 {
                            validated.remove(0);
                        }
                    }
                }
            }
        }
    }
    println!("We are looking for char: {}", char_pos);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
