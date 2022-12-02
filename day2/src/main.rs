use std::time::Instant;

// Terrible split and even worse code, please don't read this file

fn main() {
    let before = Instant::now();
    let input: Vec<(&str, &str)> = include_str!("../input.txt")
        .lines().map(|l| {
            let mut x = l.split_whitespace();
            (x.next().unwrap(), x.next().unwrap())
        }).collect();

    let input: Vec<(u32, u32)> = input.iter().map(|x| {
        (letter_to_num(x.0), letter_to_num(x.1))
    }).collect();

    println!("{:?}", input.iter().map(|x| x.1 + win_score(x.1, x.0)).sum::<u32>());
    println!("{:?}", input.iter().map(|x| win_score2(x.1, x.0)).sum::<u32>());

    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn letter_to_num(letter: &str) -> u32 {
    match letter {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 99
    }
}

fn win_score(my: u32, enemy: u32) -> u32 {
    if my == enemy {
        3
    } else if (my, enemy) == (1, 3) || (my, enemy) == (2, 1) || (my, enemy) == (3,2) {
        6
    } else {
        0
    }
}

fn win_score2(my: u32, enemy: u32) -> u32 {
    let chosen = match my {
        1 => lose(enemy),
        2 => enemy,
        3 => win(enemy),
        _ => 99
    };
    if chosen == 99 {
        panic!();
    }
    chosen + if chosen == enemy {
        3
    } else if (chosen, enemy) == (1, 3) || (chosen, enemy) == (2, 1) || (chosen, enemy) == (3,2) {
        6
    } else {
        0
    }
}

fn lose(enemy:u32) -> u32{
    match enemy {
        1 => 3,
        2 => 1,
        _ => 2,
    }
}

fn win(enemy:u32) -> u32{
    match enemy {
        1 => 2,
        2 => 3,
        _ => 1,
    }
}
