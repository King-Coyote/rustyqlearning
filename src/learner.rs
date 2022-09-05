use super::environment::Environment;

pub trait Learner<E, F> 
where 
    E: Environment,
    F: EpsilonFunction,
{
    fn learn(&self, env: &E, epsilon: &F);
}

pub trait EpsilonFunction {}
