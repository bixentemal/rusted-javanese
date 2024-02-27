use std::time::{Instant};

fn main() {
    let start_time = Instant::now();
    let number_of_trials = 100_000;
    let number_of_iterations = 100_000;
    let error = calculate_error(number_of_trials, number_of_iterations);
    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    let nbJniTransition = 0;
    println!("Rust -> Rust -> Rust : res = {} Total execution time for JNI transitions (    ) = {} \t\t: {:?}ms", error, nbJniTransition, execution_time.as_millis());
}

fn calculate_error(number_of_trials: i32, number_of_iterations: i32) -> i32{
    let mut error = 0;
    for _ in 0..number_of_trials {
        error += 50 - calc(number_of_iterations);
    }
    error
}

fn calc(n: i32) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        let r = gen(n, i);
        sum += r;
    }
    sum / n
}

fn gen(m: i32, round: i32) -> i32 {
    (m * round) % 10
}
