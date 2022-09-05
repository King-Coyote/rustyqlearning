use num::clamp;
use super::Action;

pub struct Environment {
    state: PositionState,
    width: u32,
    height: u32
}

impl Environment {
    pub fn state(&self) -> usize {
        self.get_state_index()
    }

    pub fn advance_state(&mut self, a: Action) -> usize {
        use Action::*;
        let x = self.state.x;
        let y = self.state.y;
        let mut new = match a {
            UP => PositionState {x, y: y + 1},
            DOWN => PositionState { x, y: y - 1 },
            LEFT => PositionState { x: x - 1, y },
            RIGHT => PositionState { x: x + 1, y },
        };
        new.x = clamp(new.x, 0, self.width);
        new.y = clamp(new.y, 0, self.height);
        self.state = new;
        self.state()
    }

    pub fn reward_from_action(&self, action: Action) -> f32 {
        1.0
    }

    pub fn get_state_index(&self) -> usize {
        (self.state.y * self.width + self.state.x) as usize
    }

    pub fn get_statespace_length(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Clone)]
pub struct PositionState {
    x: u32,
    y: u32,
}

impl PositionState {
    fn to_index(&self, width: u32) -> usize {
        (self.y * width + self.x) as usize
    }
}