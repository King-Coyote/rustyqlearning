use rand::{self, Rng};
use crate::environment::Environment;

pub const LEARNING_RATE: f32 = 0.1;
pub const DISCOUNT_RATE: f32 = 0.99;

pub trait Learner {
    fn learn(&mut self, env: &mut dyn Environment, epsilon: &mut dyn EpsilonFunction) -> Option<f32>;
    fn simulate(&self, env: &mut dyn Environment) -> Option<(String, f32)>;
    fn print(&self);
}

pub struct TableLearner {
    qualities: Vec<Vec<f32>>,
    a_length: usize,
    s_length: usize,
}

impl TableLearner {
    pub fn using_env(env: &dyn Environment) -> Self {
        let a_length = env.get_actionspace_length();
        let s_length = env.get_statespace_length();
        let qualities = (0..s_length)
            .map(|_| vec![0.0; a_length])
            .collect();
        TableLearner { qualities, a_length, s_length }
    }

    fn get_q(&self, s: usize, a: usize) -> Option<f32> {
        self.qualities.get(s)?.get(a).copied()
    }

    fn set_q(&mut self, s: usize, a: usize, val: f32) -> Option<()> {
        *self.qualities.get_mut(s)?.get_mut(a)? = val;
        Some(())
    }

    fn max_from_state(&self, s: usize) -> f32 {
        self.qualities.get(s).unwrap()
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max)
    }

    fn best_action_for_state(&self, s: usize) -> Option<usize> {
        let mut max = f32::NEG_INFINITY;
        let action_index = self.qualities.get(s)?
            .iter()
            .enumerate()
            .fold(0, |acc, (i, q)| {
                if *q > max {
                    max = *q;
                    return i;
                }
                acc
            });
        Some(action_index)
    }
}

impl Learner for TableLearner {
    fn learn(&mut self, env: &mut dyn Environment, epsilon: &mut dyn EpsilonFunction) -> Option<f32> {
        // needs to assign a value to a cell at coords
        // to do this it needs to know what state to get
        // choose action with some epsilon (init 1.0)
        // do action, observe reward
        // set quality according to algorithm
        let current_s = env.state();
        let mut rng = rand::thread_rng();
        let a = match epsilon.select() {
            ActionStrategy::Explore => rng.gen_range(0..self.a_length),
            ActionStrategy::Exploit => self.best_action_for_state(current_s)?,
        };
        let (next_s, reward) = env.take_action(a)?;
        let current_q = self.get_q(current_s, a)?;
        let max_next_action = self.max_from_state(next_s);
        let new_q = current_q + LEARNING_RATE * (reward + DISCOUNT_RATE * max_next_action - current_q);
        self.set_q(current_s, a, new_q);

        Some(new_q)
    }

    fn simulate(&self, env: &mut dyn Environment) -> Option<(String, f32)> {
        let current_s = env.state();
        let a = self.best_action_for_state(current_s)?;
        let (state, reward) = env.take_action(a)?;
        println!("Took action index {:?}; received reward {}", a, reward);
        Some(("".to_owned(), reward))
    }

    fn print(&self) {
        for row in self.qualities.iter() {
            for cell in row.iter() {
                print!("{}\t", *cell)
            }
            println!();
        }
    }
}

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