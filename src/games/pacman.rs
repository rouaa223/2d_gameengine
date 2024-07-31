use crate::engine::game::Game;
use crate::engine::render::Renderer;
use crate::engine::input::InputHandler;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(PartialEq)]
pub enum Cell {
    Wall,
    Dot,
    Empty
}
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
}

pub struct PacmanGame {
    grid: Vec<Vec<Cell>>, 
    pacman: Character, 
    ghosts: Vec<Character>, 
    renderer: Renderer,
    input_handler: InputHandler,
}

impl PacmanGame {
    pub fn new(context: CanvasRenderingContext2d, canvas: HtmlCanvasElement) -> Self {
        let grid = vec! [
            vec![Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Wall, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Dot, Cell::Dot, Cell::Dot, Cell::Wall],
            vec![Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall, Cell::Wall],

        ];
        let pacman = Character {
            x: 1,
            y: 1,
            direction: Direction::Right,
        };
        let ghosts = vec![
            Character {
                x: 3,
                y: 1,
                direction: Direction::Left,
            },
        ];
        Self {
            grid,
            pacman,
            ghosts,
            renderer:  Renderer::new(context),
            input_handler: InputHandler::new(),
        }
    }
    fn update (&mut self, delta_time: f64){
        self.update_pacman();
        self.update_ghosts();
    }
    fn update_pacman(&mut self){
        let mut new_x = self.pacman.x;
        let mut new_y = self.pacman.y;
        if self.input_handler.is_left_pressed() {
            new_x -=1;
            self.pacman.direction = Direction::Left;
        }else if self.input_handler.is_right_pressed() {
            new_x +=1;
            self.pacman.direction = Direction::Right;
        }else if self.input_handler.is_down_pressed() {
            new_y += 1;
            self.pacman.direction = Direction::Down;
        }else if self.input_handler.is_up_pressed() {
            new_y -=1;
            self.pacman.direction = Direction::Up;
        }
        
        if self.grid[new_y][new_x] != Cell::Wall {
            self.pacman.x = new_x;
            self.pacman.y = new_y;
            
            if self.grid[new_y][new_x] == Cell::Dot {
                self.grid[new_y][new_x] = Cell::Wall;
            }
        }

    }
    fn update_ghosts(&mut self){
        for ghost in &mut self.ghosts {
            let mut new_x = ghost.x;
            let mut new_y = ghost.y;

            match ghost.direction {
                Direction::Up => {
                    if new_y>0 {
                        new_y -=1;
                    }
                }
                Direction::Down => {
                    if new_y < self.grid.len() - 1 {
                        new_y +=1;
                    }
                }
                Direction::Left => {
                    if new_x>0 {
                        new_x -=1;
                    }
                }
                Direction::Right => {
                    if new_x < self.grid[0].len()-1 {
                        new_x += 1;
                    }                
                }
            }

            if self.grid[new_y][new_x] != Cell::Wall {
                ghost.x = new_x;
                ghost.y = new_y;
            }else {
                ghost.direction = match ghost.direction {
                    Direction::Up => Direction::Down,
                    Direction::Down => Direction::Up,
                    Direction::Left => Direction::Right,
                    Direction::Right => Direction::Left,
                };
            }

        }

    }
    fn render(&self){
        self.renderer.clear((self.grid[0].len() * 20) as f64, (self.grid.len() * 20) as f64);
        for (y,row) in self.grid.iter().enumerate(){
            for (x,cell) in row.iter().enumerate(){
                match cell {
                    Cell::Wall => {
                        self.renderer.draw_rect((x*20) as f64, (y*20) as f64, 20.0, 20.0, "blue");
                    }
                    Cell::Dot => {
                        self.renderer.draw_ball((x*20+10) as f64, (y*20+10) as f64, 3.0, "white");
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
