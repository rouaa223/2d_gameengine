pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn add(&self, other: &Vector2) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn multiply(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

pub struct Rigidbody {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub mass: f64,
}

impl Rigidbody {
    pub fn new(position: Vector2, mass: f64) -> Self {
        Self {
            position,
            velocity: Vector2::zero(),
            acceleration: Vector2::zero(),
            mass,
        }
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration = self.acceleration.add(&force.multiply(1.0 / self.mass));
    }

    pub fn update(&mut self, delta_time: f64) {
        self.velocity = self.velocity.add(&self.acceleration.multiply(delta_time));
        self.position = self.position.add(&self.velocity.multiply(delta_time));
        self.acceleration = Vector2::zero();
    }
}

pub struct PhysicsEngine {
    pub gravity: Vector2,
    pub friction: f64,
}

impl PhysicsEngine {
    pub fn new(gravity: Vector2, friction: f64) -> Self {
        Self { gravity, friction }
    }

    pub fn update_rigidbody(&self, rigidbody: &mut Rigidbody, delta_time: f64) {
        // Apply gravity
        rigidbody.apply_force(self.gravity.multiply(rigidbody.mass));

        // Apply friction
        rigidbody.velocity = rigidbody.velocity.multiply(1.0 - self.friction);

        // Update position
        rigidbody.update(delta_time);
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
        if *ball_x + *ball_dx < paddle_width
            && *ball_y > left_paddle_y
            && *ball_y < left_paddle_y + paddle_height
        {
            *ball_dx = -*ball_dx;
        }

        // Check for collision with right paddle
        if *ball_x + *ball_dx > canvas_width - paddle_width
            && *ball_y > right_paddle_y
            && *ball_y < right_paddle_y + paddle_height
        {
            *ball_dx = -*ball_dx;
        }
    }
}
