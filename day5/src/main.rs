use std::time::Instant;
use day5::Instruction;

const WIDTH: usize = 9;

fn main() {
    let before = Instant::now();
    let mut crates: Vec<Vec<char>> = vec![vec![]; WIDTH+1];

    let (starting_crates, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();

    // Parse starting positions
    for line in starting_crates.lines().rev() {
        let line = line.as_bytes();
        for (char_index, crate_nr) in (1..=1+4*(WIDTH-1)).step_by(4).zip(1..=WIDTH) {
            if line[char_index].is_ascii_uppercase() {
                crates[crate_nr].push(line[char_index] as char)
            }
        }
    }

    // Parse instructions
    let instructions: Vec<Instruction> = instructions.lines().map(|l| {
        let words: Vec<&str> = l.split_whitespace().collect();
        let count = words[1].parse().unwrap();
        let from = words[3].parse().unwrap();
        let to = words[5].parse().unwrap();
        Instruction::new(count, from, to)
    }).collect();

    part_a(crates.clone(), &instructions);
    part_b(crates.clone(), &instructions);

    println!("Time elapsed {:.2?}", before.elapsed());
}

fn part_a(mut crates: Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    for &Instruction {count, from, to} in instructions.iter() {
        for _ in 0..count {
            let c = crates[from].pop().unwrap();
            crates[to].push(c);
        }
    }

    print!("Part a: ");
    for c in crates.iter().skip(1) {
        print!("{}", c[c.len()-1]);
    }
    println!();
}
fn part_b(mut crates: Vec<Vec<char>>, instructions: &Vec<Instruction>) {
    for &Instruction {count, from, to} in instructions.iter() {
        let mut items = vec![];
        for _ in 0..count {
            items.push(crates[from].pop().unwrap());
        }
        items.reverse();
        crates[to].append(&mut items);
    }
    
    print!("Part b: ");
    for c in crates.iter().skip(1) {
        print!("{}", c[c.len()-1]);
    }
    println!();
}
