#[derive(Debug, PartialEq)]
enum Operation {
    Addx(i32),
    Noop,
}

// Want to clean this up as well but assignmenet is due soon, maybe in the future.

fn main() {
    let instructions: Vec<Operation> = include_str!("../input.txt").lines().map(|ins| {
        match &ins[..4] {
            "addx" => Operation::Addx(ins.split_once(" ").unwrap().1.parse().unwrap()),
            _ => Operation::Noop
        }
    }).collect();

    let (_, _, strengths) = instructions.iter().fold((1, 0, vec![]), |(mut x, mut cycle, mut strengths), operation| {
        cycle += 1;
        if cycle % 40 == 20 {
            strengths.push(x * cycle);
        }
        if let Operation::Addx(v) = operation {
            cycle += 1;
            if cycle % 40 == 20 {
                strengths.push(x * cycle);
            }
            x += v;
        }
        (x, cycle, strengths)
    });
    println!("Part a: {:?}", strengths.iter().sum::<i32>());

    let (_, _, crt) = instructions.iter().fold((1, 0, vec![]), |(mut x, mut cycle, mut crt), operation| {
        cycle += 1;
        if [x-1, x, x+1].contains(&(cycle % 40 - 1)) {
            crt.push('#');
        } else {
            crt.push('.');
        }
        if let Operation::Addx(v) = operation {
            cycle += 1;
            if [x-1, x, x+1].contains(&(cycle % 40 - 1)) {
                crt.push('#');
            } else {
                crt.push('.');
            }
            x += v;
        }
        (x, cycle, crt)
    });

    println!("\nPart b");
    for i in 0..6 {
        for j in 0..40 {
            print!("{}", crt[i*40 + j]);
        }
        println!();
    }
}
