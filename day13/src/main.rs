use std::cmp::Ordering;

use serde_json::Value;
use serde_json::Value::{Array, Number};

// I have never used serde or serde_json before so it took quite some time to understand and I was
// fighting the type system a bit. Could probably just have done my own parsing faster but seemeed
// like a good oppertunity to learn a bit about a commonly used crate.
fn main() {
    let pairs: Vec<(Value, Value)> = include_str!("../input.txt").split("\n\n").map(|pair| {
        let mut lines = pair.lines();
        let first: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        let second: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
        (first, second)
    }).collect();

    println!("Sum part a: {}", pairs.iter().enumerate().filter(|(_, pair)| cmp_packets(&pair.0, &pair.1) == Ordering::Less).map(|(i, _)| i+1).sum::<usize>());

    let mut packets: Vec<Value> = include_str!("../input.txt").lines().filter(|&l| l != "").map(|line| {
        serde_json::from_str(line).unwrap()
    }).collect();

    let div_pack_1: Value = serde_json::from_str("[[2]]").unwrap();
    let div_pack_2: Value = serde_json::from_str("[[6]]").unwrap();
    packets.push(div_pack_1.clone());
    packets.push(div_pack_2.clone());
    packets.sort_by(|a, b| cmp_packets(a, b));
    let pos1 = packets.iter().position(|x| x == &div_pack_1).unwrap() + 1;
    let pos2 = packets.iter().position(|x| x == &div_pack_2).unwrap() + 1;
    
    println!("Part b: {}", pos1*pos2);
}

fn cmp_packets(first: &Value, second: &Value) -> Ordering {
    match (first, second) {
        (Number(num1), Number(num2)) => num1.as_u64().unwrap().cmp(&num2.as_u64().unwrap()),
        (Number(_), Array(_)) => cmp_packets(&Array(vec![first.clone()]), second),
        (Array(_), Number(_)) => cmp_packets(first, &Array(vec![second.clone()])),
        (Array(arr1), Array(arr2)) => {
            let mut i = 0;
            loop {
                if i >= arr1.len() && i >= arr2.len() {
                    break Ordering::Equal;
                }
                if i >= arr1.len() {
                    break Ordering::Less; //left ran out
                }
                if i >= arr2.len() {
                    break Ordering::Greater; //right ran out
                }

                let order = cmp_packets(&arr1[i], &arr2[i]);
                if order.is_ne() {
                    break order;
                }
                
                i += 1;
            }
        }
        _ => unreachable!("Should always be number or arrays only"),
    }
}
