use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut max = [0,0,0];
    let mut curr_i = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                let trimmed = ip.trim();
                if trimmed.is_empty() {
                    if curr_i > max[0] {
                        max[0] = curr_i;
                    }
                    max.sort();
                    curr_i = 0;
                } else {
                    curr_i += ip.parse::<u32>().unwrap();
                }
            }
        }
    }
    print!("Max is: {}", max[0]+max[1]+max[2]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
