use std::f64::consts::PI;

use crate::{
    game::game::Game,
    window::window::{MINIMAP_SCALING, WINDOW_HEIGHT, WINDOW_WIDTH},
};

#[derive(Clone, Copy)]
pub enum TurnDirection {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

#[derive(Clone, Copy)]
pub enum WalkDirection {
    Neutral = 0,
    Forward = 1,
    Backward = -1,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub turn_direction: TurnDirection,
    pub walk_direction: WalkDirection,
    pub rotation_angle: f64,
    pub walk_speed: f64,
    pub turn_speed: f64,
    pub minimap_scale: f64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: WINDOW_WIDTH as f64 / 2.0,
            y: WINDOW_HEIGHT as f64 / 2.0,
            width: 1.0,
            height: 1.0,
            turn_direction: TurnDirection::Neutral,
            walk_direction: WalkDirection::Neutral,
            rotation_angle: PI / 2.0,
            walk_speed: 100.0,
            turn_speed: 45.0 * (PI / 180.0),
            minimap_scale: MINIMAP_SCALING,
        }
    }
}

impl Player {
    fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    pub fn set_walk_direction(&mut self, direction: &str) -> Result<(), String> {
        match direction.to_lowercase().as_str() {
            "neutral" => self.walk_direction = WalkDirection::Neutral,
            "forward" => self.walk_direction = WalkDirection::Forward,
            "backward" => self.walk_direction = WalkDirection::Backward,
            _ => return Err(format!("Unknown walk direction {}", direction)),
        }
        Ok(())
    }

    pub fn set_turn_direction(&mut self, direction: &str) -> Result<(), String> {
        match direction.to_lowercase().as_str() {
            "neutral" => self.turn_direction = TurnDirection::Neutral,
            "left" => self.turn_direction = TurnDirection::Left,
            "right" => self.turn_direction = TurnDirection::Right,
            _ => return Err(format!("Unknown turn direction {}", direction)),
        }
        Ok(())
    }

    pub fn move_player(&mut self, delta: f64, game: &Game) {
        self.rotation_angle += match self.turn_direction {
            TurnDirection::Neutral => 0.0,
            TurnDirection::Left => -self.turn_speed * delta,
            TurnDirection::Right => self.turn_speed * delta,
        };

        let distance = match self.walk_direction {
            WalkDirection::Neutral => 0.0,
            WalkDirection::Forward => self.walk_speed * delta,
            WalkDirection::Backward => -self.walk_speed * delta,
        };

        let new_x = self.x + self.rotation_angle.cos() * distance;
        let new_y = self.y + self.rotation_angle.sin() * distance;

        if !game.is_coordinate_solid(new_x, new_y) {
            self.set_x(new_x);
            self.set_y(new_y);
        }
    }
}
