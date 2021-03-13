use crate::error::KendallError;
use crate::traits::KendallElement;

pub fn check_array_len<T: KendallElement>(
    x: &[T], y: &[T]
) -> Result<usize, KendallError> {
    let n = x.len();
    if n != y.len() {
        Err(KendallError::DifferentLength { left: n, right: y.len() })
    } else {
        Ok(n)
    }
}