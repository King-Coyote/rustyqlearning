use std::env;
use learner::*;
use environment::*;

mod learner;
mod environment;

fn main() {

    // how to train?? needs to be a choice between interactive and N
    // default to N for now
    // scenario.train(10);
    // let mut n = 0;
    // scenario.simulate(|| {
    //     n += 1;
    //     n >= 10
    // });
    // scenario.print();
    let mut env = Environment::new(3, 2, 0, 0);
    let mut learner = TableLearner::using_env(&env);
    for _ in 0..10 {
        let q = learner.learn(&mut env);
        match q {
            Some(v) => {
                if v < 0.0 {
                    break;
                }
            },
            None => break
        }
    }
    learner.print();
}