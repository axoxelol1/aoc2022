fn main() {
    let pairs: Vec<((u32, u32), (u32, u32))> = 
        include_str!("../input.txt")
        .lines().map(|l| {
            let (first, second) = l.split_once(',').unwrap();
            let (s1, e1) = first.split_once('-').unwrap();
            let (s2, e2) = second.split_once('-').unwrap();
            ((s1.parse().unwrap(), e1.parse().unwrap()), (s2.parse().unwrap(), e2.parse().unwrap()))
        }).collect();

    println!("Part a: {}", pairs.iter().filter(|(f, s)| fully_contains(f, s)).count());
    println!("Part b: {}", pairs.iter().filter(|(f, s)| overlaps(f, s)).count());

}

fn fully_contains(first: &(u32, u32), second: &(u32, u32)) -> bool {
    (first.0 <= second.0 && second.1 <= first.1) ||
    (second.0 <= first.0 && first.1 <= second.1)
}

fn overlaps(first: &(u32, u32), second: &(u32, u32)) -> bool {
    first.0 <= second.1 && second.0 <= first.1
}
