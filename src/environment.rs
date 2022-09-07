use num::clamp;
use rand::random;
use super::EpsilonFunction;

pub trait Environment {
    fn state(&self) -> usize;
    fn take_action(&mut self, a: usize) -> Option<(usize, f32)>;
    fn get_statespace_length(&self) -> usize;
    fn get_actionspace_length(&self) -> usize;
}

#[derive(Clone)]
pub struct PositionalEnvironment {
    state: PositionState,
    width: i32,
    height: i32,
    rewards: Vec<Vec<f32>>,
}

impl PositionalEnvironment {
    pub fn new(width: i32, height: i32, x: i32, y: i32) -> Self {
        // hardcoded for now
        let rewards = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, -10.0, 10.0],
        ];

        PositionalEnvironment {
            state: PositionState { x, y },
            width,
            height,
            rewards
        }
    }

    fn next_state_from_action(&self, action: MoveAction) -> PositionState {
        use MoveAction::*;
        let x = self.state.x;
        let y = self.state.y;
        let mut new = match action {
            UP => PositionState {x, y: y - 1},
            DOWN => PositionState { x, y: y + 1 },
            LEFT => PositionState { x: x - 1, y },
            RIGHT => PositionState { x: x + 1, y },
        };
        new.x = clamp(new.x, 0, self.width - 1);
        new.y = clamp(new.y, 0, self.height - 1);
        new
    }

    fn get_action_index(&self, action: MoveAction) -> usize {
        use MoveAction::*;
        match action {
            UP => 0,
            DOWN => 1,
            LEFT => 2,
            RIGHT => 3,
        }
    }

    pub fn take_reward_from_action(&mut self, action: MoveAction) -> Option<f32> {
        let PositionState {x, y} = self.next_state_from_action(action);
        let reward = self.rewards.get_mut(y as usize)?.get_mut(x as usize).copied();
        self.rewards[y as usize][x as usize] = -1.0;
        reward
    }

    pub fn get_state_index(&self) -> usize {
        (self.state.y * self.width + self.state.x) as usize
    }

    fn pos_action_from_index(a: usize) -> MoveAction {
        use MoveAction::*;
        match a {
            0 => UP,
            1 => DOWN,
            2 => LEFT,
            3 => RIGHT,
            _ => panic!("invalid action index provided to env")
        }
    }
}

impl Environment for PositionalEnvironment {
    fn state(&self) -> usize {
        self.get_state_index()
    }

    fn take_action(&mut self, a: usize) -> Option<(usize, f32)> {
        let action = PositionalEnvironment::pos_action_from_index(a);
        let reward = self.take_reward_from_action(action)?;
        self.state = self.next_state_from_action(action);
        Some((self.state(), reward))
    }

    fn get_statespace_length(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn get_actionspace_length(&self) -> usize {
        4
    }

}

#[derive(Clone)]
pub struct PositionState {
    x: i32,
    y: i32,
}

impl PositionState {
    fn to_index(&self, width: i32) -> i32 {
        self.y * width + self.x
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MoveAction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn random_action() -> MoveAction {
    use MoveAction::*;
    match random::<f32>() {
        r if r < 0.25 => UP,
        r if r > 0.25 && r < 0.5 => DOWN,
        r if r > 0.5 && r < 0.75 => LEFT,
        _ => RIGHT
    }
}