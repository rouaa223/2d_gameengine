// src/engine/input.rs

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use std::rc::Rc;
use std::cell::RefCell;

pub struct InputHandler {
    up_key: Rc<RefCell<bool>>,
    down_key: Rc<RefCell<bool>>,
    left_key: Rc<RefCell<bool>>,
    right_key: Rc<RefCell<bool>>,
}

impl InputHandler {
    pub fn new() -> Self {
        let up_key = Rc::new(RefCell::new(false));
        let down_key = Rc::new(RefCell::new(false));
        let left_key = Rc::new(RefCell::new(false));
        let right_key = Rc::new(RefCell::new(false));

        let window = web_sys::window().expect("no global window");
        let document = window.document().expect("no document in this window");

        let up_key_clone = up_key.clone();
        let down_key_clone = down_key.clone();
        let left_key_clone = left_key.clone();
        let right_key_clone = right_key.clone();

        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            match event.key().as_str() {
                "ArrowUp" => *up_key_clone.borrow_mut() = true,
                "ArrowDown" => *down_key_clone.borrow_mut() = true,
                "ArrowLeft" => *left_key_clone.borrow_mut() = true,
                "ArrowRight" => *right_key_clone.borrow_mut() = true,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);
        document
        .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
        .unwrap();
    keydown_closure.forget();
    let up_key_clone = Rc::clone(&up_key);
    let down_key_clone = Rc::clone(&down_key);
    let left_key_clone = Rc::clone(&left_key);
    let right_key_clone = Rc::clone(&right_key);
    let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {

        match event.key().as_str() {
            "ArrowUp" => *up_key_clone.borrow_mut() = false,
            "ArrowDown" => *down_key_clone.borrow_mut() = false,
            "ArrowLeft" => *left_key_clone.borrow_mut() = false,
            "ArrowRight" => *right_key_clone.borrow_mut() = false,
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);
    document 
     .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
     .unwrap();
    keyup_closure.forget();

        Self {
            up_key,
            down_key,
            left_key,
            right_key,
        }
    }

    pub fn is_up_pressed(&self) -> bool {
        *self.up_key.borrow()
    }


    pub fn is_left_pressed(&self) -> bool {
        *self.left_key.borrow()
    }

    pub fn is_down_pressed(&self) -> bool {
        *self.down_key.borrow()
    }

    pub fn is_right_pressed(&self) -> bool {
        *self.right_key.borrow()
    }


}
