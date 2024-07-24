// src/engine/input.rs

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;
use std::rc::Rc;
use std::cell::RefCell;

pub struct InputHandler {
    up_key: Rc<RefCell<bool>>,
    down_key: Rc<RefCell<bool>>,
}

impl InputHandler {
    pub fn new() -> Self {
        let up_key = Rc::new(RefCell::new(false));
        let down_key = Rc::new(RefCell::new(false));

        // Set up key event listeners
        let window = web_sys::window().expect("should have a window in this context");
        let up_key_clone = up_key.clone();
        let down_key_clone = down_key.clone();

        // Keydown event listener
        let keydown_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            match event.key().as_str() {
                "ArrowUp" => *up_key_clone.borrow_mut() = true,
                "ArrowDown" => *down_key_clone.borrow_mut() = true,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("keydown", keydown_closure.as_ref().unchecked_ref())
            .unwrap();
        keydown_closure.forget();

        // Keyup event listener
        let up_key_clone = up_key.clone();
        let down_key_clone = down_key.clone();
        let keyup_closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            match event.key().as_str() {
                "ArrowUp" => *up_key_clone.borrow_mut() = false,
                "ArrowDown" => *down_key_clone.borrow_mut() = false,
                _ => {}
            }
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("keyup", keyup_closure.as_ref().unchecked_ref())
            .unwrap();
        keyup_closure.forget();

        Self { up_key, down_key }
    }

    pub fn is_up_pressed(&self) -> bool {
        *self.up_key.borrow()
    }

    pub fn is_down_pressed(&self) -> bool {
        *self.down_key.borrow()
    }
}
