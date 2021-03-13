use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq, Clone)]
pub struct AvlNode<T: PartialOrd> {
    value: T,
    smaller: usize,
    left: AvlTree<T>,
    right: AvlTree<T>,
}

pub type AvlTree<T> = Option<Box<AvlNode<T>>>;

#[derive(Debug, PartialEq, Clone)]
pub struct AvlTreeSet<T: PartialOrd> {
    root: AvlTree<T>,
}

impl<T: PartialOrd> AvlTreeSet<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl<T: PartialOrd> AvlTreeSet<T> {
    pub fn insert(&mut self, value: T) -> usize {
        let mut count = 0;
        let mut current_tree = &mut self.root;

        while let Some(current_node) = current_tree {
            let flg = if current_node.value < value {
                true
            } else if current_node.value > value {
                false
            } else {
                let ud = Uniform::new(0.0, 1.0);
                if ud.sample(&mut rand::thread_rng()) < 0.5 {
                    true
                } else {
                    false
                }
            };
            if flg {
                count += current_node.smaller + 1;
                current_tree = &mut current_node.right;
            } else {
                current_node.smaller += 1;
                current_tree = &mut current_node.left;
            }
        }

        *current_tree = Some(Box::new(AvlNode {
            value,
            smaller: 0,
            left: None,
            right: None,
        }));

        count
    }
}