extern crate num_cpus;

use rayon::prelude::*;
use std::f64::consts::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::thread;

// Módulo Input-Output

pub fn read_info() -> (usize, i64) {
    print!("Threads to be used for Rayon Solution (2-4 if not sure): ");
    stdout().flush().unwrap();

    let mut threads = String::new();
    stdin().read_line(&mut threads).unwrap();
    let threads = threads.trim();
    let threads = threads.parse::<u32>().unwrap_or(1);

    print!("Iterations to be used for Spawn Solution (10000 at least): ");
    stdout().flush().unwrap();

    let mut iter = String::new();
    stdin().read_line(&mut iter).unwrap();
    let iter = iter.trim();
    let iter = iter.parse::<i64>().unwrap_or(10000);

    (threads as usize, iter)
}

// Métodos Solución Rayon

pub fn factorial(n: usize) -> f64 {
    const FACTORIALS: [f64; 17] = [
        1.0,              // 0!
        1.0,              // 1!
        2.0,              // 2!
        6.0,              // 3!
        24.0,             // 4!
        120.0,            // 5!
        720.0,            // 6!
        5040.0,           // 7!
        40320.0,          // 8!
        362880.0,         // 9!
        3628800.0,        // 10!
        39916800.0,       // 11!
        479001600.0,      // 12!
        6227020800.0,     // 13!
        87178291200.0,    // 14!
        1307674368000.0,  // 15!
        20922789888000.0, //16!
    ];

    FACTORIALS[n]
}

pub fn calc_rayon(threads: usize) -> f64 {
    let factor = (SQRT_2 * 2.0) / 9801.0;

    let sum = (0..=threads)
        .into_par_iter()
        .map(|i| {
            let k = i as f64;

            let numerator = factorial(i * 4) * (1103.0 + (26390.0 * k));
            let denominator = factorial(i).powf(4.0) * (396_f64).powf(4.0 * k);

            factor * numerator / denominator
        })
        .reduce(|| 0.0, |a, b| a + b);

    1.0 / sum as f64
}

// Métodos Solución Spawn

pub fn get_parallel_sum(k: i64, i: i64) -> f64 {
    let n = k as f64;
    let delta = 1.0 / n;
    let sum: f64 = (i..=k)
        .into_par_iter()
        .map(|i| {
            let f_i = i as f64;
            let x_i = delta * (f_i + 0.5);

            4.0 / (1.0 + x_i.powf(2.0))
        })
        .reduce(|| 0.0, |a, b| a + b);

    sum * delta
}

pub fn calc_spawn(iter: i64) -> f64 {
    let threads = num_cpus::get();

    let mut handles: Vec<thread::JoinHandle<f64>> = Vec::new();
    for i in 0..threads as i64 {
        handles.push(thread::spawn(move || {
            let number: f64 = get_parallel_sum(iter, i * (iter));
            number
        }));
    }

    let mut res: f64 = 0.0;
    for thread in handles.into_iter() {
        res += thread.join().unwrap();
    }

    res
}

// Testing

pub fn test() {
    let input = read_info();
    let rayon_res = calc_rayon(input.0);
    let spawn_res = calc_spawn(input.1);

    //Multithread con Rayon

    println!("---------------------------------");
    println!("RAYON SOLUTION\n");
    println!("Expected PI:  {:.60}", PI);
    println!("Calculed PI:  {:.60}", rayon_res);
    println!("Abs Err:      {:.60}", (PI - rayon_res).abs());
    println!("---------------------------------");

    //Multithread con spawn

    println!("SPAWN SOLUTION\n");
    println!("Expected PI:  {:.60}", PI);
    println!("Calculed PI:  {:.60}", spawn_res);
    println!("Abs Err:      {:.60}", (PI - spawn_res).abs());
    println!("--------------------------------");
}
