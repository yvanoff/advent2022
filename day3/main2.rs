use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut prio_sum = 0;
    if let Ok(mut lines) = read_lines("input") {
        while let Some(line) = lines.next() {
            if let Ok(ip) = line {
                let line2 = if let Some(line2) = lines.next() { line2 } else { todo!() };
                let line3 = if let Some(line3) = lines.next() { line3 } else { todo!() };
                let ip2 = if let Ok(ip2) = line2 { ip2 } else { todo!() };
                let ip3 = if let Ok(ip3) = line3 { ip3 } else { todo!() };
                for c in ip.chars() {
                    if ip2.contains(c) && ip3.contains(c) {
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
