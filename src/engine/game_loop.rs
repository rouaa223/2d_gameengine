use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Window;
use std::rc::Rc;
use std::cell::RefCell;
use crate::engine::game::Game;

pub struct GameLoop {
    game: Rc<RefCell<dyn Game>>,
    window: Window,
}

impl GameLoop {
    pub fn new(game: Rc<RefCell<dyn Game>>) -> Self {
        let window = web_sys::window().expect("there is no global window");
        Self {
            game,
            window,
        }
    }

    pub fn start(&self) {
        
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let game_clone = self.game.clone();
        let window_clone = self.window.clone();

        // Wrap the callback function
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut game = game_clone.borrow_mut();
            game.update(0.016);
            game.render();

            // Request the next frame
            window_clone
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("request_animation_frame failed");
        }) as Box<dyn FnMut()>));

        // Request the first frame
        self.window
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("request_animation_frame failed");
    }
}
