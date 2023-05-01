extern crate piston_window;
extern crate rand;

mod draw;
mod game;


use piston_window::types::Color;
use piston_window::*;
use crate::draw::to_coord;
use crate::game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main(){
    let (width, height)= (30, 30);
    let window: PistonWindow = WindowSettings::new("Snake", [to_coord(width), to_coord(height)])
                                .exit_on_esc(true)
                                .build()
                                .unwrap();
    let mut game: Game= Game::new(width, height);

    println!("hello world");
}
