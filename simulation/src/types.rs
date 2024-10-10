pub const INF: f64 = f64::INFINITY;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Chain {
    pub timestamp: f64,
    pub work: f64,
    pub height: usize,
    pub arrival_time: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub timestamp: f64,
    pub work: f64,
}
