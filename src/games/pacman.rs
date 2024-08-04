use crate::engine::game::Game;
use crate::engine::render::Renderer;
use crate::engine::input::InputHandler;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(PartialEq)]
pub enum Cell {
    Wall,
    Dot,
    Empty,
    PowerUp,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Character {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub speed: f64, // Movement speed control
    pub move_timer: f64, // Timer to manage movement speed
}

pub struct PacmanGame {
    grid: Vec<Vec<Cell>>,
    pacman: Character,
    ghosts: Vec<Character>,
    renderer: Renderer,
    input_handler: InputHandler,
}

impl PacmanGame {
    pub fn new(context: CanvasRenderingContext2d, _canvas: HtmlCanvasElement) -> Self {
        let grid = vec![
            vec![Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            vec![Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
        ];

        let pacman = Character {
            x: 1,
            y: 1,
            direction: Direction::Right,
            speed: 0.1, // Pac-Man speed
            move_timer: 0.0,
        };

        let ghosts = vec![
            Character {
                x: 9,
                y: 1,
                direction: Direction::Left,
                speed: 0.9, // Ghost speed
                move_timer: 0.8,
            },
            Character {
                x: 17,
                y: 3,
                direction: Direction::Up,
                speed: 0.9, // Ghost speed
                move_timer: 0.8,
            },
        ];

        Self {
            grid,
            pacman,
            ghosts,
            renderer: Renderer::new(context),
            input_handler: InputHandler::new(),
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.update_pacman(delta_time);
        self.update_ghosts(delta_time);
    }

    fn update_pacman(&mut self, delta_time: f64) {
        // Update Pac-Man's move timer
        self.pacman.move_timer += delta_time;

        // Move Pac-Man only if enough time has passed based on its speed
        if self.pacman.move_timer >= self.pacman.speed {
            let mut new_x = self.pacman.x;
            let mut new_y = self.pacman.y;

            if self.input_handler.is_left_pressed() {
                new_x = new_x.saturating_sub(1);
                self.pacman.direction = Direction::Left;
            } else if self.input_handler.is_right_pressed() {
                new_x += 1;
                self.pacman.direction = Direction::Right;
            } else if self.input_handler.is_down_pressed() {
                new_y += 1;
                self.pacman.direction = Direction::Down;
            } else if self.input_handler.is_up_pressed() {
                new_y = new_y.saturating_sub(1);
                self.pacman.direction = Direction::Up;
            }

            // Check if Pac-Man can move to the new position
            if self.grid[new_y][new_x] != Cell::Wall {
                self.pacman.x = new_x;
                self.pacman.y = new_y;

                if self.grid[new_y][new_x] == Cell::Dot {
                    self.grid[new_y][new_x] = Cell::Empty; 
                } else if self.grid[new_y][new_x] == Cell::PowerUp {
                    
                    self.grid[new_y][new_x] = Cell::Empty;
                    
                }
            }

            // Reset move timer 
            self.pacman.move_timer = 0.0;
        }
    }

    fn update_ghosts(&mut self, delta_time: f64) {
        let pacman_position = (self.pacman.x, self.pacman.y);

        for ghost in &mut self.ghosts {
            // Update ghost's move timer
            ghost.move_timer += delta_time;

            // Move ghost only if enough time has passed based on its speed
            if ghost.move_timer >= ghost.speed {
                let (mut best_direction, mut min_distance) = (ghost.direction, f64::MAX);

                // Determine the best direction to move towards Pac-Man
                for &direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                    let (new_x, new_y) = match direction {
                        Direction::Up => (ghost.x, ghost.y.saturating_sub(1)),
                        Direction::Down => (ghost.x, ghost.y + 1),
                        Direction::Left => (ghost.x.saturating_sub(1), ghost.y),
                        Direction::Right => (ghost.x + 1, ghost.y),
                    };

                    if self.grid[new_y][new_x] != Cell::Wall {
                        let distance = distance_to_pacman(new_x, new_y, pacman_position);

                        if distance < min_distance {
                            min_distance = distance;
                            best_direction = direction;
                        }
                    }
                }

                // Update ghost pos
                match best_direction {
                    Direction::Up => ghost.y = ghost.y.saturating_sub(1),
                    Direction::Down => ghost.y += 1,
                    Direction::Left => ghost.x = ghost.x.saturating_sub(1),
                    Direction::Right => ghost.x += 1,
                }

                ghost.direction = best_direction;

                // Reset move timer 
                ghost.move_timer = 0.3;
            }
        }
    }
}

fn distance_to_pacman(x: usize, y: usize, pacman_position: (usize, usize)) -> f64 {
    let dx = pacman_position.0 as isize - x as isize;
    let dy = pacman_position.1 as isize - y as isize;
    ((dx * dx + dy * dy) as f64).sqrt()
}

impl PacmanGame {
    fn render(&self) {
        self.renderer
            .clear((self.grid[0].len() * 20) as f64, (self.grid.len() * 20) as f64);

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Wall => {
                        self.renderer.draw_rect((x * 20) as f64, (y * 20) as f64, 20.0, 20.0, "blue");
                    }
                    Cell::Dot => {
                        self.renderer
                            .draw_ball((x * 20 + 10) as f64, (y * 20 + 10) as f64, 3.0, "white");
                    }
                    Cell::PowerUp => {
                        self.renderer
                            .draw_ball((x * 20 + 10) as f64, (y * 20 + 10) as f64, 5.0, "green");
                    }
                    Cell::Empty => {}
                }
            }
        }

        self.renderer.draw_ball(
            (self.pacman.x * 20 + 10) as f64,
            (self.pacman.y * 20 + 10) as f64,
            10.0,
            "yellow",
        );

        for ghost in &self.ghosts {
            self.renderer.draw_ball(
                (ghost.x * 20 + 10) as f64,
                (ghost.y * 20 + 10) as f64,
                10.0,
                "red",
            );
        }
    }
}

impl Game for PacmanGame {
    fn update(&mut self, delta_time: f64) {
        self.update(delta_time);
    }

    fn render(&self) {
        self.render();
    }
}
