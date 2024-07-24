pub trait Game {
    fn update(&mut self, delta_time: f64);
    fn render(&self);
}
