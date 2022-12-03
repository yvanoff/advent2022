use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                let play = u32::from(ip.chars().nth(2).unwrap()) - u32::from('X');
                let opponent = u32::from(ip.chars().nth(0).unwrap())-u32::from('A')+1;
                let result = 
                    if play == 1 {
                        opponent as i32
                    }
                    else if play == 0 {
                        (((opponent as i32)-4)%3)+3
                    } else {
                        ((opponent as i32)%3)+1
                    };
                score = score + 3*(play as i32) + result;
            }
        }
    }
    print!("Final score is: {}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
