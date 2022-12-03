use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut prio_sum = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                let (first, second) = ip.split_at(ip.bytes().count()/2);
                for c in first.chars() {
                    if second.contains(c) {
                        let p = 
                            if c.is_lowercase() {
                                u32::from(c) - u32::from('a')+1
                            } else {
                                u32::from(c) - u32::from('A')+27
                            };
                        prio_sum += p;
                        break;
                    }
                }
            }
        }
    }
    print!("Final priority is: {}", prio_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
