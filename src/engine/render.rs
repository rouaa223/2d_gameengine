// src/engine/render.rs

use web_sys::CanvasRenderingContext2d;

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d) -> Self {
        Self { context }
    }

    pub fn clear(&self, width: f64, height: f64) {
        self.context.clear_rect(0.0, 0.0, width, height);
    }

    pub fn draw_rect(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        self.context.set_fill_style(&color.into());
        self.context.fill_rect(x, y, width, height);
    }

    pub fn draw_ball(&self, x: f64, y: f64, radius: f64, color: &str) {
        self.context.begin_path();
        self.context
            .arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        self.context.set_fill_style(&color.into());
        self.context.fill();
        self.context.close_path();
    }
    pub fn drw_triang(&self, x:f64, color: &str){
      
    }
}
