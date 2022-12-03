use std::time::Instant;

fn main() {
    let before = Instant::now();
    let input = include_bytes!("../input.txt");

    let answer1 = &input[..input.len()-1].split(|&x| x == b'\n').map(|sack| {
        let comp1 = &sack[..sack.len()/2];
        let comp2 = &sack[sack.len()/2..];
        byte_to_prio(*comp1.iter().find(|x| comp2.contains(x)).unwrap())
    }).sum::<u32>();

    let sacks: Vec<&[u8]> = input[..input.len()-1].split(|&x| x == b'\n').collect();

    let mut badges: Vec<u8> = vec![];
    'outer: for i in (0..sacks.len()-2).step_by(3) {
        for l in sacks[i] {
            if sacks[i+1].contains(l) && sacks[i+2].contains(l) {
                badges.push(*l);
                continue 'outer;
            }
        }
    }

    println!("Part 1: {answer1}");
    println!("Part 2: {}", badges.iter().map(|&x| byte_to_prio(x)).sum::<u32>());
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn byte_to_prio(byte: u8) -> u32 {
    if 65 <= byte && byte <= 90 {
        (byte - 38) as u32
    } else {
        (byte - 96) as u32
    }
}
