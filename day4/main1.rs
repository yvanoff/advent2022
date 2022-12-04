use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut contained = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                let trimmed = ip.trim();
                let assignments : Vec<&str> = trimmed.split(&[',','-']).collect();
                if (assignments[0].parse::<i32>().unwrap() - assignments[2].parse::<i32>().unwrap())*
                    (assignments[1].parse::<i32>().unwrap() - assignments[3].parse::<i32>().unwrap()) <= 0 {
                        contained += 1;
                }
            }
        }
    }
    print!("Contained assignements: {}", contained);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
