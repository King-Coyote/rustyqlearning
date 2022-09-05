mod scenario;
mod learner;
mod environment;

fn main() {
    // build the scenario
    // take scenario through N training steps
    // run the scenario until condition met?? yeh maybe
    let scenario = Scenario::new();
    // how to train?? needs to be a choice between interactive and N
    // default to N for now
    scenario.train(10);
    let mut n = 0;
    scenario.simulate(|| {
        n += 1;
        n >= 10
    });
    scenario.print();
}