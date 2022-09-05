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
    let learner = TableLearner::new(10, 10);
    learner.print();
}