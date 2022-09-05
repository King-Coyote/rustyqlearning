use super::environment::Environment;
use super::learner::{Learner, EpsilonFunction,};

struct Yes;
struct No;

#[derive(Default)]
pub struct ScenarioBuilder<E, F, HasEnv, HasLearner, HasEps> {
    env: Option<dyn Environment>,
    learner: Option<Box<dyn Learner>>,
    epsilon: Option<dyn EpsilonFunction>,
}

impl<E, F, HasEnv, HasLearner, HasEps> ScenarioBuilder<E, F, HasEnv, HasLearner, HasEps>
where
    E: Environment,
    F: EpsilonFunction,
{
    pub fn new() -> Self<No, No, No> {
        ScenarioBuilder {

        }
    }

    pub fn build() -> Scenario {
        Scenario
    }
}

pub struct Scenario<E, F>
where
    E: Environment,
    F: EpsilonFunction,
{
    learner: Box<dyn Learner<E, F>>,
    env: E,
    epsilon: F,
}

impl<E, F> Scenario<E, F>
where
    E: Environment,
    F: EpsilonFunction,
{
    pub fn new(env: E, learner: impl Learner<E, F>, epsilon: F) -> Self {
        Scenario {
            learner: Box::new(learner),
            env,
            epsilon,
        }
    }

    pub fn train(&self, n: u32) {
        for n in 0..n {
            self.learner.learn(&self.env, &self.epsilon);
        }
    }

    pub fn simulate<C>(&self, cond: C)
    where C: FnMut() -> bool {
        
    }

    pub fn print(&self) {

    }
}