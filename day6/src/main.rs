use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let before = Instant::now();
    let input = include_bytes!("../input.txt");
    let mut set = HashSet::new();

    //part a
    for (char_count, window) in input.windows(4).enumerate() {
        set.clear();
        if window.iter().all(|b| set.insert(b)) {
            println!("Part a: {}", char_count + 4);
            break;
        }
    }

    //part b
    for (char_count, window) in input.windows(14).enumerate() {
        set.clear();
        if window.iter().all(|b| set.insert(b)) {
            println!("Part a: {}", char_count + 14);
            break;
        }
    }

    println!("{:.2?}", before.elapsed());
}
