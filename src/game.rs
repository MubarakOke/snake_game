use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle}; 

const BORDER_COLOR: Color= [0.0, 0.0, 0.0, 1.0];

pub struct Game {
    width: i32,
    height: i32
}