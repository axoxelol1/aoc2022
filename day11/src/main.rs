use std::time::Instant;

#[derive(Debug, Clone)]
enum Operation {
    Square,
    Add(u64),
    Mul(u64)
}

impl Operation {
    fn from_input(input: &str) -> Self {
        if &input[19..] == "old * old" {
            Operation::Square
        } else if input.contains("+") {
            Operation::Add((&input[25..]).parse().unwrap())
        } else {
            Operation::Mul((&input[25..]).parse().unwrap())
        }
    }
    fn apply(&self, num: u64) -> u64 {
        match self {
            Operation::Square => num * num,
            Operation::Add(x) => num + x,
            Operation::Mul(x) => num * x,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    iftrue: usize,
    iffalse: usize,
    inspects: u64,
}

impl Monkey {
    fn parse_monkeys(input: &str) -> Vec<Monkey> {
        input.split("\n\n").map(|monkey| {
            let lines: Vec<&str> = monkey.lines().collect();
            let items: Vec<u64> = (&lines[1][18..]).split(", ").map(|x| x.parse().unwrap()).collect();
            let operation = Operation::from_input(lines[2]);
            let divisor: u64 = (&lines[3][21..]).parse().unwrap();
            let iftrue: usize = (&lines[4][29..]).parse().unwrap();
            let iffalse: usize = (&lines[5][30..]).parse().unwrap();
            Monkey{items, operation, divisor, iftrue, iffalse, inspects: 0}
        }).collect()
    }
}


fn main() {
    let before = Instant::now();
    let monkeys: Vec<Monkey> = Monkey::parse_monkeys(include_str!("../input.txt"));

    parta(monkeys.clone());
    partb(monkeys.clone());
    println!("Time elapsed: {:.2?}", before.elapsed());
}

fn parta(mut monkeys: Vec<Monkey>) {
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop().unwrap();
                monkeys[i].inspects += 1;
                let new_worry = monkeys[i].operation.apply(item) / 3;
                let new_monkey = if new_worry % monkeys[i].divisor == 0 {
                    monkeys[i].iftrue
                } else {
                    monkeys[i].iffalse
                };
                monkeys[new_monkey].items.push(new_worry)
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspects);
    println!("Part a: {}", monkeys[monkeys.len()-1].inspects * monkeys[monkeys.len()-2].inspects);
}

fn partb(mut monkeys: Vec<Monkey>) {
    let cm = monkeys.iter().fold(1, |curr, m| curr*m.divisor);

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop().unwrap();
                monkeys[i].inspects += 1;
                let new_worry = monkeys[i].operation.apply(item) % cm;
                let new_monkey = if new_worry % monkeys[i].divisor == 0 {
                    monkeys[i].iftrue
                } else {
                    monkeys[i].iffalse
                };
                monkeys[new_monkey].items.push(new_worry)
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspects);
    println!("Part b: {}", monkeys[monkeys.len()-1].inspects * monkeys[monkeys.len()-2].inspects);
}
