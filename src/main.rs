use kendall::*;
use rand::{thread_rng, Rng};
use std::time::Instant;

const T: usize = 10000;

fn main() {
    let mut rng = thread_rng();
    /*
    let x = (0..T).map(|_| rng.gen()).collect::<Vec<f64>>();
    let y = (0..T).map(|_| rng.gen()).collect::<Vec<f64>>();

    let start = Instant::now();
    println!("== tau (definition) ==");
    println!("{:.4}, {}ms", tau(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== zB (definition) ==");
    println!("{:.4}, {}ms", zb(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== tau (avltree) ==");
    println!("{:.4}, {}ms", tau_avltree(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== zB (avltree) ==");
    println!("{:.4}, {}ms", zb_avltree(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    
    let x = (0..T).map(|_| if rng.gen::<f64>() < 0.5 { 0.0 } else { 1.0 }).collect::<Vec<f64>>();
    let y = (0..T).map(|_| if rng.gen::<f64>() < 0.5 { 0.0 } else { 1.0 }).collect::<Vec<f64>>();

    let start = Instant::now();
    println!("== tau (definition) ==");
    println!("{:.4}, {}ms", tau(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== zB (definition) ==");
    println!("{:.4}, {}ms", zb(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== tau (binary) ==");
    let xb = x.iter().map(|&xi| xi == 1.0).collect::<Vec<bool>>();
    let yb = y.iter().map(|&yi| yi == 1.0).collect::<Vec<bool>>();
    println!("{:.4}, {}ms", tau_binary(&xb, &yb).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== zB (binary) ==");
    let xb = x.iter().map(|&xi| xi == 1.0).collect::<Vec<bool>>();
    let yb = y.iter().map(|&yi| yi == 1.0).collect::<Vec<bool>>();
    println!("{:.4}, {}ms", zb_binary(&xb, &yb).unwrap(), (Instant::now() - start).as_millis());
    */

    let x = (0..T).map(|_| if rng.gen::<f64>() < 0.9 { 0.0 } else { rng.gen() }).collect::<Vec<f64>>();
    let y = (0..T).map(|_| if rng.gen::<f64>() < 0.9 { 0.0 } else { rng.gen() }).collect::<Vec<f64>>();

    let start = Instant::now();
    println!("== tau (definition) ==");
    println!("{:.4}, {}ms", tau(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== zB (definition) ==");
    println!("{:.4}, {}ms", zb(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== tau (zero) ==");
    println!("{:.4}, {}ms", tau_zero(&x, &y).unwrap(), (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("== zB (zero) ==");
    println!("{:.4}, {}ms", zb_zero(&x, &y).unwrap(), (Instant::now() - start).as_millis());
}
    