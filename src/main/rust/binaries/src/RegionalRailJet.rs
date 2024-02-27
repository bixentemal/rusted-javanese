pub mod vmutils;

use crate::vmutils::{create_vm, prepare_static_method, actual_call};
use std::time::{Instant};
use crate::vmutils::jni_inc::{jclass, jmethodID, JNIEnv};

fn main() {
    let (_, env) = create_vm("-Djava.class.path=target/rusted-javanese-1.0-SNAPSHOT.jar");
    let cl_method = prepare_static_method(env, "com/rusty/JavaJungleJuice", "gen", "(II)I");

    let start_time = Instant::now();
    let number_of_trials = 100_000;
    let number_of_iterations = 100_000;
    let error = calculate_error(number_of_trials, number_of_iterations, env, cl_method.0, cl_method.1);
    let end_time = Instant::now();
    let execution_time = end_time - start_time;
    let nbJniTransition = number_of_trials as u64 * number_of_iterations as u64;
    println!("Rust -> Rust -> Java : res = {} Total execution time for JNI transitions (R->J) = {} \t: {:?}ms", error, nbJniTransition, execution_time.as_millis());
}

fn calculate_error(number_of_trials: i32, number_of_iterations: i32, env: *mut JNIEnv, class: jclass, method: jmethodID) -> i32 {
    let mut error = 0;
    for _ in 0..number_of_trials {
        error += 50 - calc(number_of_iterations, env, class, method);
    }
    error
}

fn calc(n: i32, env: *mut JNIEnv, class: jclass, method: jmethodID) -> i32 {
    let mut sum = 0;
    for i in 0..n {
        let r = gen(n, i, env, class, method);
        sum += r;
    }
    sum / n
}

fn gen(m: i32, round: i32, env: *mut JNIEnv,  class: jclass, method: jmethodID) -> i32 {
    actual_call(env, class, method, m, round)
}