// src/engine/physics.rs

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn update_ball_position(
        &self,
        ball_x: &mut f64,
        ball_y: &mut f64,
        ball_dx: &mut f64,
        ball_dy: &mut f64,
        canvas_width: f64,
        canvas_height: f64,
        ball_radius: f64,
        left_paddle_y: f64,
        right_paddle_y: f64,
        paddle_width: f64,
        paddle_height: f64,
    ) {
        // Update ball position
        *ball_x += *ball_dx;
        *ball_y += *ball_dy;

        // Check for collision with top and bottom
        if *ball_y + *ball_dy > canvas_height - ball_radius || *ball_y + *ball_dy < ball_radius {
            *ball_dy = -*ball_dy;
        }

        // Check for collision with left paddle
        if *ball_x + *ball_dx < paddle_width && *ball_y > left_paddle_y && *ball_y < left_paddle_y + paddle_height {
            *ball_dx = -*ball_dx;
        }

        // Check for collision with right paddle
        if *ball_x + *ball_dx > canvas_width - paddle_width && *ball_y > right_paddle_y && *ball_y < right_paddle_y + paddle_height {
            *ball_dx = -*ball_dx;
        }
    }
}
