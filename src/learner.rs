use rand::random;
use super::environment::Environment;

pub const LEARNING_RATE: f32 = 0.1;
pub const DISCOUNT_RATE: f32 = 0.99;

pub trait Learner {
    fn learn(&mut self, env: &mut Environment) -> Option<f32>;
    fn simulate(&self, env: &mut Environment) -> Option<(Action, f32)>;
    fn print(&self);
}

pub struct TableLearner {
    qualities: Vec<Vec<f32>>,
}

impl TableLearner {
    pub fn using_env(env: &Environment) -> Self {
        let width = 4; // MAGIC!
        let height = env.get_statespace_length();
        let qualities = (0..height)
            .map(|_| vec![0.0; width])
            .collect();
        TableLearner { qualities }
    }

    fn get_q(&self, s: usize, a: usize) -> Option<f32> {
        self.qualities.get(s)?.get(a).copied()
    }

    fn set_q(&mut self, s: usize, a: usize, val: f32) -> Option<()> {
        *self.qualities.get_mut(s)?.get_mut(a)? = val;
        Some(())
    }

    fn get_action_index(&self, a: Action) -> usize {
        use Action::*;
        match a {
            UP => 0,
            DOWN => 1,
            LEFT => 2,
            RIGHT => 3,
        }
    }

    fn max_from_state(&self, s: usize) -> f32 {
        self.qualities.get(s).unwrap()
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max)
    }

    fn best_action_for_state(&self, s: usize) -> Option<Action> {
        use Action::*;
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
        let action = match action_index {
            0 => UP,
            1 => DOWN,
            2 => LEFT,
            3 => RIGHT,
            _ => panic!("invalid action index supplied to get best action"),
        };
        Some(action)
    }
}

impl Learner for TableLearner {
    fn learn(&mut self, env: &mut Environment) -> Option<f32> {
        // needs to assign a value to a cell at coords
        // to do this it needs to know what state to get
        // choose action with some epsilon (init 1.0)
        // do action, observe reward
        // set quality according to algorithm
        let action = random_action();
        let a = self.get_action_index(action);

        let current_s = env.state();
        let current_q = self.get_q(current_s, a)?;
        let (reward, next_s) = env.take_action(action)?;
        let max_next_action = self.max_from_state(next_s);
        let new_q = current_q + LEARNING_RATE * (reward + DISCOUNT_RATE * max_next_action - current_q);
        self.set_q(current_s, a, new_q);

        Some(new_q)
    }

    fn simulate(&self, env: &mut Environment) -> Option<(Action, f32)> {
        let current_s = env.state();
        let action = self.best_action_for_state(current_s)?;
        let (reward, _) = env.take_action(action)?;
        println!("Took action {:?}; received reward {}", action, reward);
        Some((action, reward))
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

fn random_action() -> Action {
    use Action::*;
    match random::<f32>() {
        r if r < 0.25 => UP,
        r if r > 0.25 && r < 0.5 => DOWN,
        r if r > 0.5 && r < 0.75 => LEFT,
        _ => RIGHT
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub trait EpsilonFunction {}
