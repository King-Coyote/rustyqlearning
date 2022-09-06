use num::clamp;
use super::Action;

#[derive(Clone)]
pub struct Environment {
    state: PositionState,
    width: i32,
    height: i32,
    rewards: Vec<Vec<f32>>,
}

impl Environment {
    pub fn new(width: i32, height: i32, x: i32, y: i32) -> Self {
        // hardcoded for now
        let rewards = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, -10.0, 10.0],
        ];

        Environment {
            state: PositionState { x, y },
            width,
            height,
            rewards
        }
    }

    pub fn state(&self) -> usize {
        self.get_state_index()
    }

    pub fn take_action(&mut self, action: Action) -> Option<(f32, usize)> {
        let reward = self.take_reward_from_action(action)?;
        self.state = self.next_state_from_action(action);
        Some((reward, self.state()))
    }

    fn next_state_from_action(&self, action: Action) -> PositionState {
        use Action::*;
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

    pub fn take_reward_from_action(&mut self, action: Action) -> Option<f32> {
        let PositionState {x, y} = self.next_state_from_action(action);
        let reward = self.rewards.get_mut(y as usize)?.get_mut(x as usize).copied();
        self.rewards[y as usize][x as usize] = -1.0;
        reward
    }

    pub fn get_state_index(&self) -> usize {
        (self.state.y * self.width + self.state.x) as usize
    }

    pub fn get_statespace_length(&self) -> usize {
        (self.width * self.height) as usize
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