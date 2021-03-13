use num_traits::Float;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::error::KendallError;
use crate::traits::KendallElement;
use crate::utils::check_array_len;

pub fn calc_tau<T: Float>(
    n: usize, (nc, nd): (usize, usize),
    n12: Option<(usize, usize)>,
) -> Result<T, KendallError> {
    if n < 2 {
        Err(KendallError::ShortArray{ length: n })
    } else {
        let n0 = n * (n - 1) / 2;
        let deli_opt = match n12 {
            Some((n1, n2)) => {
                if n0 <= n1 || n0 <= n2 {
                    None
                } else {
                    Some(T::from((n0 - n1) * (n0 - n2)).unwrap().sqrt())
                }
            }
            None => Some(T::from(n0).unwrap())
        };
        match deli_opt {
            Some(deli) => Ok((T::from(nc).unwrap() - T::from(nd).unwrap()) / deli),
            None => Err(KendallError::AllElementsAreTheSame)
        }
    }
}

pub fn calc_n_tie<T: KendallElement>(t: &HashMap<T, usize>) -> usize {
    t.iter().map(|(_, &v)| {
        if v <= 1 {
            0
        } else {
            v * (v - 1) / 2
        }
    }).sum()
}

pub fn kendall_on2<T>(x: &[T], y: &[T])
-> Result<(usize, (usize, usize), Option<(usize, usize)>), KendallError>
where
    T: KendallElement,
{
    let n = match check_array_len(x, y) {
        Ok(num) => num,
        Err(e) => { return Err(e); }
    };
    let (mut nc, mut nd) = (0, 0);
    let (mut t, mut u) = (HashMap::new(), HashMap::new());
    for i in 0..n {
        let (xi, yi) = (x[i], y[i]);
        let count = t.entry(xi).or_insert(0);
        *count += 1;
        let count = u.entry(yi).or_insert(0);
        *count += 1;
        for j in i+1..n {
            let (xj, yj) = (x[j], y[j]);
            if (xi < xj && yi < yj) || (xi > xj && yi > yj) {
                nc += 1;
            } else if (xi > xj && yi < yj) || (xi < xj && yi > yj) {
                nd += 1;
            }
        }
    }
    let n12 = (calc_n_tie(&t), calc_n_tie(&u));
    Ok((n, (nc, nd), Some(n12)))
}

pub fn kendall_binary<T>(x: &[T], y: &[T])
-> Result<(usize, (usize, usize), Option<(usize, usize)>), KendallError>
where
    T: KendallElement,
{
    let n = match check_array_len(x, y) {
        Ok(num) => num,
        Err(e) => { return Err(e); }
    };
    let x_elem: HashSet<&T> = HashSet::from_iter(x.iter());
    let y_elem: HashSet<&T> = HashSet::from_iter(y.iter());
    let elem = x_elem.union(&y_elem).map(|&&e| e).collect::<Vec<T>>();
    if elem.len() != 2 {
        return Err(KendallError::NotBinary { n: elem.len() });
    }
    let ea = elem[0];
    let (mut a, mut s) = (0, 0);
    let (mut ax, mut ay) = (0, 0);
    let (mut dx, mut dy) = (0, 0);
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        if xi == ea {
            ax += 1;
            if yi == ea {
                ay += 1;
                a += 1;
            } else {
                dx += 1;
            }
        } else {
            if yi == ea {
                ay += 1;
                dy += 1;
            } else {
                s += 1;
            }
        }
    }
    let ncd =(a * s, dx * dy);
    let n1 = if ax == 0 || ax == n {
        0
    } else {
        (ax * (ax - 1) + (n - ax) * (n - ax - 1)) / 2
    };
    let n2 = if ay == 0 || ay == n {
        0
    } else {
        (ay * (ay - 1) + (n - ay) * (n - ay - 1)) / 2
    };
    Ok((n, ncd, Some((n1, n2))))
}