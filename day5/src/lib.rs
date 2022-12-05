#[derive(Debug)]
pub struct Instruction {
    pub count: usize,
    pub from: usize,
    pub to: usize
}

impl Instruction {
    pub fn new(count: usize, from: usize, to: usize) -> Instruction {
        Instruction{count, from, to}
    }
}
