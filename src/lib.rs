pub mod avltree;
pub mod core;
pub mod error;
pub mod traits;
pub mod utils;

pub struct Kendall<T: PartialOrd> {
    x: Vec<T>,
    y: Vec<T>,
}

impl<T: PartialOrd + Clone> Kendall<T> {
    pub fn new(x: &[T], y: &[T]) -> Self {
        let x = x.to_vec();
        let y = y.to_vec();

        Self { x, y }
    }
}