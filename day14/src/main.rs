use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(Debug, PartialEq)]
enum Cell {
    Rock,
    Sand,
}

type Point = (i32, i32);

// Part a is included as a comment, part b solution is really ugly just adding a floor between x
// -1000 and 1000 and hoping it is enough, fast split of about 4min though. Essay due tomorrow so
// probably won't clean up.

fn main() {
    let mut cells: HashMap<Point, Cell> = HashMap::new();

    for line in include_str!("../input.txt").lines() {
        let coords: Vec<Point> = line.split(" -> ").map(|tup| {
            let (x,y) = tup.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }).collect();

        for pair in coords.windows(2) {
            for rock in get_all_rock(pair[0], pair[1]) {
                cells.insert(rock, Cell::Rock);
            }
        }
    }

    let lowest_y = cells.iter().max_by_key(|((_,y), _)| y).unwrap().0.1;

    for x in -1000..1000 {
        cells.insert((x, lowest_y+2), Cell::Rock);
    }

    // part a
    // 'outer: loop {
    //     let mut curr = (500,0);
    //     loop {
    //         if curr.1 > lowest_y {
    //             break 'outer;
    //         }
    //         if !cells.contains_key(&(curr.0, curr.1+1)) {
    //             curr = (curr.0, curr.1+1);
    //         } else if !cells.contains_key(&(curr.0-1, curr.1+1)) {
    //             curr = (curr.0-1, curr.1+1);
    //         } else if !cells.contains_key(&(curr.0+1, curr.1+1)) {
    //             curr = (curr.0+1, curr.1+1);
    //         } else {
    //             cells.insert(curr, Cell::Sand);
    //             break;
    //         }
    //     }
    // }
    //
    // part b
    'outer: loop {
        let mut curr = (500,0);
        loop {
            if cells.contains_key(&(500, 0)) {
                break 'outer;
            }
            if !cells.contains_key(&(curr.0, curr.1+1)) {
                curr = (curr.0, curr.1+1);
            } else if !cells.contains_key(&(curr.0-1, curr.1+1)) {
                curr = (curr.0-1, curr.1+1);
            } else if !cells.contains_key(&(curr.0+1, curr.1+1)) {
                curr = (curr.0+1, curr.1+1);
            } else {
                cells.insert(curr, Cell::Sand);
                break;
            }
        }
    }

    println!("{}", cells.values().filter(|x| **x == Cell::Sand).count());
}

fn get_all_rock(p1: Point, p2: Point) -> Vec<Point> {
    if p1.0 == p2.0 {
        (min(p1.1, p2.1)..=max(p1.1, p2.1)).map(|y| (p1.0, y)).collect()
    } else {
        (min(p1.0, p2.0)..=max(p1.0, p2.0)).map(|x| (x, p1.1)).collect()
    }
}
