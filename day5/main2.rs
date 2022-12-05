use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    struct Stack {
        nb: u32,
        cargo: Vec<char>,
    }

    if let Ok(lines) = read_lines("input") {
        let (instr, mut stacks): (Vec<_>, Vec<_>) = lines.partition(|n| n.as_ref().unwrap().chars().nth(0) == Some('m'));
        stacks.pop();
        let stacks_arrang = if let Some(Ok(stacks_arrang)) = stacks.pop() { stacks_arrang } else { todo!() };
        let mut s_a_trimmed : String = String::from(stacks_arrang.trim());
        let nb_stacks = s_a_trimmed.pop().unwrap().to_digit(10).unwrap();
        let mut stacks_rpz: Vec<Stack> = Vec::new();
        for i in 0..nb_stacks {
            stacks_rpz.push(Stack{nb: i, cargo: Vec::<char>::new()});
        }
        for line in stacks {
            if let Ok(ip) = line {
                for i in 0..nb_stacks {
                    let index_load = 4*i+1;
                    let char_to_add = ip.chars().nth(index_load as usize).unwrap();
                    if char_to_add != ' ' {
                        stacks_rpz[i as usize].cargo.push(char_to_add);
                    }
                }
            }
        }
        for line in instr {
            if let Ok(ip) = line {
                let line_split: Vec<&str> = ip.split(' ').collect();
                let nb_moves = line_split[1].parse::<u32>().unwrap();
                let from = line_split[3].parse::<u32>().unwrap() - 1;
                let dest = line_split[5].parse::<u32>().unwrap() - 1;
                let mut moved_crates: Vec<char> = Vec::new();
                for i in 0..nb_moves {
                    if !stacks_rpz[from as usize].cargo.is_empty() {
                        let moved_crate = stacks_rpz[from as usize].cargo.remove(0);
                        moved_crates.push(moved_crate);
                    }
                }
                for i in 0..nb_moves {
                    let moved_crate = moved_crates.pop().unwrap();
                    stacks_rpz[dest as usize].cargo.insert(0, moved_crate);
                }

            }
        }
        for i in stacks_rpz {
            if !i.cargo.is_empty() {
                println!("Top of the stack {} is: {}", i.nb, i.cargo[0]);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
