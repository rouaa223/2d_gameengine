

use crate::engine::input::InputHandler;
use crate::engine::render::Renderer;
use crate::engine::physics::PhysicsEngine;
use crate::engine::game::Game;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

pub struct Pong {
    left_paddle_y: f64,
    right_paddle_y: f64,
    ball_x: f64,
    ball_y: f64,
    ball_dx: f64,
    ball_dy: f64,
    paddle_width: f64,
    paddle_height: f64,
    ball_radius: f64,
    canvas_width: f64,
    canvas_height: f64,
    renderer: Renderer,
    input_handler: InputHandler,
    physics_engine: PhysicsEngine,
}

impl Pong {
    pub fn new(context: CanvasRenderingContext2d, canvas: HtmlCanvasElement) -> Self {
        let paddle_width = 10.0;
        let paddle_height = 75.0;
        let ball_radius = 10.0;

        let left_paddle_y = (canvas.height() as f64 - paddle_height) / 2.0;
        let right_paddle_y = (canvas.height() as f64 - paddle_height) / 2.0;
        let ball_x = canvas.width() as f64 / 2.0;
        let ball_y = canvas.height() as f64 / 2.0;
        let ball_dx = 4.0;
        let ball_dy = -4.0;

        Self {
            left_paddle_y,
            right_paddle_y,
            ball_x,
            ball_y,
            ball_dx,
            ball_dy,
            paddle_width,
            paddle_height,
            ball_radius,
            canvas_width: canvas.width() as f64,
            canvas_height: canvas.height() as f64,
            renderer: Renderer::new(context),
            input_handler: InputHandler::new(),
            physics_engine: PhysicsEngine::new(),
        }
    }
}

impl Game for Pong {
    fn update(&mut self, _delta_time: f64) {
        // Update ball position and handle collisions using the physics engine
        self.physics_engine.update_ball_position(
            &mut self.ball_x,
            &mut self.ball_y,
            &mut self.ball_dx,
            &mut self.ball_dy,
            self.canvas_width,
            self.canvas_height,
            self.ball_radius,
            self.left_paddle_y,
            self.right_paddle_y,
            self.paddle_width,
            self.paddle_height,
        );

        // Move left paddle based on input
        if self.input_handler.is_down_pressed() && self.left_paddle_y < self.canvas_height - self.paddle_height {
            self.left_paddle_y += 5.0;
        } else if self.input_handler.is_up_pressed() && self.left_paddle_y > 0.0 {
            self.left_paddle_y -= 5.0;
        }

        // Move right paddle automatically
        if self.ball_y > self.right_paddle_y + self.paddle_height / 2.0
            && self.right_paddle_y < self.canvas_height - self.paddle_height
        {
            self.right_paddle_y += 5.0;
        } else if self.ball_y < self.right_paddle_y + self.paddle_height / 2.0
            && self.right_paddle_y > 0.0
        {
            self.right_paddle_y -= 5.0;
        }
    }

    fn render(&self) {
        self.renderer.clear(self.canvas_width, self.canvas_height);

        // Draw left paddle
        self.renderer
            .draw_rect(0.0, self.left_paddle_y, self.paddle_width, self.paddle_height, "red");

        // Draw right paddle
        self.renderer.draw_rect(
            self.canvas_width - self.paddle_width,
            self.right_paddle_y,
            self.paddle_width,
            self.paddle_height,
            "red",
        );

        // Draw ball
        self.renderer
            .draw_ball(self.ball_x, self.ball_y, self.ball_radius, "white");
    }
}
