use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle}; 

const BORDER_COLOR: Color= [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    width: i32,
    height: i32
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game { width, height }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width -1, 0, 1, self.height, con, g);

    }

    pub fn update(&mut self, delta_time: f64){

    }
}