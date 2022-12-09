use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn apply(&self, pos: Position) -> Position {
        match self {
            Direction::Up => (pos.0, pos.1+1),
            Direction::Right => (pos.0+1, pos.1),
            Direction::Down => (pos.0, pos.1-1),
            Direction::Left => (pos.0-1, pos.1),
        }
    }
}

type Position = (i32, i32);

// Could easily clean this up and include both solutions but right now part a is commented out at
// bottom, no time today.

fn main() {
    let mut ins: Vec<Direction> = Vec::with_capacity(2000);
    include_str!("../input.txt").lines().for_each(|l| {
        let mut words = l.split_whitespace();
        let dir = match words.next().unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            _ => Direction::Left,
        };
        let amount = words.next().unwrap().parse().unwrap();
        for _ in 0..amount {
            ins.push(dir)
        }
    });
    let mut nine_visited = HashMap::new();

    let test = ins.iter().fold([(0,0); 10], |mut knots, dir| {
        knots[0] = dir.apply(knots[0]);
        for i in 1..10 {
            knots[i] = follow(knots[i], knots[i-1])
        }
        *nine_visited.entry(knots[9]).or_insert(0) += 1;
        knots
    });

    println!("{:?}", nine_visited.len());
}

fn follow(tail: Position, head: Position) -> Position {
    for x in -1..=1 {
        for y in -1..=1 {
            if (tail.0+x, tail.1+y) == head {
                return tail;
            }
        }
    }

    let diff = (head.0 - tail.0, head.1 - tail.1);
    (tail.0 + diff.0.signum(), tail.1 + diff.1.signum())
}
    // let mut head_visited = HashMap::new();
    // let mut tail_visited = HashMap::new();
    // head_visited.insert((0,0), 1);
    // tail_visited.insert((0,0), 1);
    // let mut head_visited = HashMap::new();
    // let mut tail_visited = HashMap::new();
    // head_visited.insert((0,0), 1);
    // tail_visited.insert((0,0), 1);

    // let test = ins.iter().fold(((0,0), (0,0)), |(head, tail), dir| {
    //     let head = dir.apply(head);
    //     let tail = follow(tail, head);
    //     *head_visited.entry(head).or_insert(0) += 1;
    //     *tail_visited.entry(tail).or_insert(0) += 1;
    //     (head, tail)
    // });
