use std::time::Instant;

fn main() {
    let before = Instant::now();
    let input = include_str!("../input.txt");

    let mut cals: Vec<u32> = input.split("\n\n")
        .map(|elf| elf.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();

    cals.sort();

    println!("Part a: {}", cals[cals.len()-1]);
    println!("Part b: {}", cals[cals.len()-1] + cals[cals.len()-2] + cals[cals.len()-3]);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
