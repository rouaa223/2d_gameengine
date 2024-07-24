use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
pub struct GameLoop {
    callback: Option<Closure<dyn FnMut()>>,
}

impl GameLoop {
    pub fn new(mut callback: F) -> Self
    where 
    F: FnMut() + 'static,
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
        Self { callback: Some(closure)}
    }
    pub fn start (&mut self) {
        let window =  web_sys::window().expect("window in this arean");
        if let Some(callback) = self.callback.take() {
            let cb = callback.clone();
            let g = Rc::new(RefCell::new(None));
            let r = g.clone();

            *r.borrow_mut() = Some(Closure::wrap(Box::new(move|| {
                cb();
                if let Some(f) = g.borrow_mut().take() {
                    window
                    .request_animation_frame(f.as_ref().unchecked_ref())
                    .expect("register'animationFrameRequest' OK");
                *g.borrow_mut() = Some(f);
                }
            }) as Box<dyn FnMut()>));
            window 
            .request_animation_frame(r.borrow_mut().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("register 'requestanimationframe' OK");
        }
    }
}