use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle}; 
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color= [0.0, 0.0, 0.0, 1.0];
const FOOD_COLOR: Color= [0.8, 0.0, 0.0, 1.0];

pub struct Game {
    width: i32,
    height: i32,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    snake: Snake
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game { width, height, food_exists: false, food_x: 0, food_y: 0, snake: Snake::new(2,2) }
    }

    pub fn key_pressed(&mut self, key: Key){
        let dir = match key{
            Key::Up=> Some(Direction::Up),
            Key::Down=> Some(Direction::Down),
            Key::Left=> Some(Direction::Left),
            Key::Right=> Some(Direction::Right),
            _=> Some(self.snake.head_direction())
        };

        if dir.unwrap() == self.snake.head_direction().opposite(){
           return; 
        }

        self.update_snake();
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        self.snake.draw(con, g)
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width -1, 0, 1, self.height, con, g);

    }

    pub fn update(&mut self, delta_time: f64){
        if !self.food_exists{
            self.add_food();
        }
    }

    pub fn add_food(&mut self){
        let mut rng= thread_rng();
        let mut new_x= rng.gen_range(1, self.width-1);
        let mut new_y: i32= rng.gen_range(1, self.height-1);

        self.food_x= new_x;
        self.food_y= new_y;
        self.food_exists= true;
    }
}