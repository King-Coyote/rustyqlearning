use num::clamp;
use super::Action;

pub struct Environment {
    state: PositionState,
    width: i32,
    height: i32
}

impl Environment {
    pub fn new(width: i32, height: i32, x: i32, y: i32) -> Self {
        Environment {
            state: PositionState { x, y },
            width,
            height
        }
    }

    pub fn state(&self) -> usize {
        self.get_state_index()
    }

    pub fn advance_state(&mut self, action: Action) -> usize {
        self.state = self.next_state_from_action(action);
        self.state()
    }

    pub fn next_state_from_action(&self, action: Action) -> PositionState {
        use Action::*;
        let x = self.state.x;
        let y = self.state.y;
        let mut new = match action {
            UP => PositionState {x, y: y - 1},
            DOWN => PositionState { x, y: y + 1 },
            LEFT => PositionState { x: x - 1, y },
            RIGHT => PositionState { x: x + 1, y },
        };
        new.x = clamp(new.x, 0, self.width);
        new.y = clamp(new.y, 0, self.height);
        new
    }

    pub fn reward_from_action(&self, action: Action) -> f32 {
        // hardcoded for now
        let new_state = self.next_state_from_action(action);
        match (new_state.x, new_state.y) {
            (0, 0) => 0.0,
            (1, 0) => 1.0,
            (2, 0) => 0.0,
            (0, 1) => 0.0,
            (1, 1) => -10.0,
            (2, 1) => 10.0,
            _ => panic!("invalid state reached!")
        }
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