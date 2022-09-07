use rand::random;

pub trait EpsilonFunction {
    fn select(&mut self) -> ActionStrategy;
}

#[derive(Clone)]
pub struct SimpleEpsilon;

impl EpsilonFunction for SimpleEpsilon {
    fn select(&mut self) -> ActionStrategy {
        ActionStrategy::Explore
    }
}

#[derive(Debug)]
pub enum ActionStrategy {
    Explore,
    Exploit
}

#[derive(Clone)]
pub struct DecayEpsilon {
    e: f32,
    rate: f32,
}

impl DecayEpsilon {
    pub fn new(rate: f32) -> Self {
        Self {
            e: 1.0,
            rate
        }
    }
}

impl EpsilonFunction for DecayEpsilon {
    fn select(&mut self) -> ActionStrategy {
        let strat = match random::<f32>() {
            r if r < self.e => ActionStrategy::Explore,
            _ => ActionStrategy::Exploit
        };
        self.e -= self.rate;
        strat
    }
}