use kendall::core::*;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let ud = Uniform::new(0.0, 1.0);
    let x = (0..100).map(|_| {
        if ud.sample(&mut rand::thread_rng()) < 0.5 {
            "A"
        } else {
            "B"
        }
    }).collect::<Vec<&str>>();
    let y = (0..100).map(|_| {
        if ud.sample(&mut rand::thread_rng()) < 0.5 {
            "A"
        } else {
            "B"
        }
    }).collect::<Vec<&str>>();

    let (no, ncdo, n12o) = kendall_on2(&x, &y).unwrap();
    let (nb, ncdb, n12b) = kendall_binary(&x, &y).unwrap();
    assert_eq!(no, nb);
    assert_eq!(ncdo, ncdb);
    assert_eq!(n12o, n12b);
}