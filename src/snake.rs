use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color= [0.00, 0.8, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn opposite (&self) ->{
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right;
            Direction::Right => Direction::Left
        }
    }
}

#[derive(debug, clone)]
struct Block {
    x: i32,
    y: i32, 
}

pub struct snake {
    
}