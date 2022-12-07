use std::time::Instant;
use std::collections::HashMap;

// Originally wanted to define a data structure with rc and stuff to practice but no time.
fn main() {
    let before = Instant::now();
    let input = include_str!("../input.txt");
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut working_dir = vec![];

    // Slution names the root folder / (so //a is a folder, ugly) and `cd /` does not
    // technically work but does not appear in the input after the first row so ¯\_(ツ)_/¯
    for line in input.lines() {
        if line == "$ cd .." {
            working_dir.pop();
        }
        else if &line[..4] == "$ cd" {
            let name = line.split_whitespace().last().unwrap();
            working_dir.push(name);
        } 
        else if line == "$ ls"{
            //Nothing
        }
        else if &line[..3] != "dir" { //It is a file
            let size: u32 = line.split_whitespace().next().unwrap().parse::<u32>().unwrap();
            for i in 0..working_dir.len() {
                let path = working_dir[..i+1].join("/");
                *dir_sizes.entry(path).or_insert_with(|| 0) += size;
            }
        }
    }

    println!("Part a {:?}", dir_sizes.values().filter(|&&size| size <= 100000).sum::<u32>());

    let used_space = dir_sizes.get(working_dir[0]).unwrap();
    println!("Part b {:?}", dir_sizes.values().filter(|&&size| used_space - size <= 40000000).min().unwrap());
    println!("Time elapsed: {:?}", before.elapsed());
}


