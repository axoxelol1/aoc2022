use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};

// Code is so messy because I started with a HashMap graph but then realized I don't have time to
// implement dijkstra's algorithm so I switched to using a library halfway through. First time
// using a big crate so took some time to understand the docs and types and code is messy but good
// practice in reading docs.

// const WIDTH: usize = 8;
// const HEIGHT: usize = 5;

const WIDTH: usize = 179;
const HEIGHT: usize = 41;

fn main() {
    let mut heightmap = [[0u8; WIDTH]; HEIGHT];
    let mut start = (0, 0);
    let mut starts_b = vec![];
    let mut end = (0, 0);
    include_bytes!("../input.txt").iter()
        .filter(|x| **x != b'\n')
        .enumerate()
        .for_each(|(i, b)| {
            if *b == b'a' {
                starts_b.push((i / WIDTH, i % WIDTH))
            }
            if *b == b'S' {
                start = (i / WIDTH, i % WIDTH);
                heightmap[i / WIDTH][i % WIDTH] = 96;
            } else if *b == b'E' {
                end = (i / WIDTH, i % WIDTH);
                heightmap[i / WIDTH][i % WIDTH] = 123;
            } else {
                heightmap[i / WIDTH][i % WIDTH] = *b;
            }
        });
    
    let mut edges = vec![];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let curr = heightmap[i][j];
            if !(i == 0 || heightmap[i-1][j] as i32 - curr as i32 > 1) {
                edges.push(((i*WIDTH + j) as u32, ((i-1)*WIDTH+j) as u32));
            }
            if !(j == WIDTH-1 || heightmap[i][j+1] as i32 - curr as i32 > 1) {
                edges.push(((i*WIDTH + j) as u32, (i*WIDTH+j+1) as u32));
            }
            if !(i == HEIGHT-1 || heightmap[i+1][j] as i32 - curr as i32 > 1) {
                edges.push(((i*WIDTH + j) as u32, ((i+1)*WIDTH+j) as u32));
            }
            if !(j == 0 || heightmap[i][j-1] as i32 - curr as i32 > 1) {
                edges.push(((i*WIDTH + j) as u32, (i*WIDTH + j-1) as u32));
            }
        }
    }

    let graph = Graph::<(), ()>::from_edges(&edges);
    
    let test = dijkstra(&graph, NodeIndex::new(start.0*WIDTH + start.1), Some(NodeIndex::new(end.0*WIDTH + end.1)), |_| 1);
    println!("Part a: {:?}", test.get(&NodeIndex::new(end.0*WIDTH + end.1)));

    let mut costs = vec![];
    for start in starts_b {
        let test = dijkstra(&graph, NodeIndex::new(start.0*WIDTH + start.1), Some(NodeIndex::new(end.0*WIDTH + end.1)), |_| 1);
        match test.get(&NodeIndex::new(end.0*WIDTH + end.1)) {
            Some(cost) => costs.push(*cost),
            _ => {}
        }
    }
    println!("{:?}", costs.iter().min());
}


