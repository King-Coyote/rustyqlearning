use std::env;
use learner::*;
use environment::*;

mod learner;
mod environment;

fn main() {
    let mut env = Environment::new(3, 2, 0, 0);
    let mut learner = TableLearner::using_env(&env);
    for _ in 0..5 {
        let mut local_env = env.clone();
        loop {
            let q = learner.learn(&mut local_env);
            match q {
                Some(v) => {
                    if v < 0.0 || v > 9.0 {
                        break;
                    }
                },
                None => break
            }
        }
    }
    learner.print();
    println!("Beginning simulation...");
    let mut simulated_env = env.clone();
    loop {
        let result = learner.simulate(&mut simulated_env);
        match result {
            Some((_, r)) => {
                if r < 0.0 || r > 9.0 {
                    break;
                }
            },
            None => break
        }
    }
}