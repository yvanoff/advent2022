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
                let ass_int : Vec<i32> = assignments.iter().map(|x| x.parse::<i32>().unwrap()).collect();
                let result = ass_int.iter().filter(|x| **x >= ass_int[2] && **x <= ass_int[3]).collect::<Vec<_>>();
                if result.len() > 2 {
                    contained += 1;
                }
                if ass_int[2] > ass_int[0] && ass_int[3] < ass_int[1] {
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
