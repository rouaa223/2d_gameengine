// src/lib.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
use std::rc::Rc;
use std::cell::RefCell;
use crate::engine::game::Game;
use crate::games::pong::Pong;

mod engine;
mod games;

#[wasm_bindgen]
pub fn start() {
    let window = window().expect("should have a window in this context");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id("pong_canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let mut pong = Pong::new(context, canvas);

    let window_clone = window.clone();
    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        pong.update(0.016); // Assuming 60 FPS, so delta_time is about 16ms
        pong.render();

        // Request the next animation frame
        window_clone
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }) as Box<dyn FnMut()>));

    window
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
