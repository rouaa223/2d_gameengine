use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use std::rc::Rc;
use std::cell::RefCell;
use crate::engine::game_loop::GameLoop;
use crate::games::pacman::PacmanGame;
//use crate::games::pong::Pong

mod engine;
mod games;

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("there is a window");
    let document = window.document().expect("should have a document in this window");
    let canvas = document 
     .get_element_by_id("game_canvas")
     .unwrap()
     .dyn_into::<HtmlCanvasElement>()
     .unwrap();
    
    let context = canvas
     .get_context("2d")
     .unwrap()
     .unwrap()
     .dyn_into::<CanvasRenderingContext2d>()
     .unwrap();
    let game = Rc::new(RefCell::new(PacmanGame::new(context, canvas)));
    let game_loop = GameLoop::new(game);
    game_loop.start();


}