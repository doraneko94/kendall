use std::fmt;
use std::hash::Hash;

pub trait KendallElement: PartialOrd + Copy + Eq + Hash + fmt::Debug + fmt::Display {}
impl<T: PartialOrd + Copy + Eq + Hash + fmt::Debug + fmt::Display> KendallElement for T {}