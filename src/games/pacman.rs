use crate::engine::game::Game;
use crate::engine::render::Renderer;
use crate::engine::input::InputHandler;

pub struct PacmanGame{
    grid: Vec<Vec<Cell>>,
    pacman: Character,
    ghosts: Vec<Character>,
    renderer: Renderer,
    input_handler: InputHandler,
}
impl PacmanGame {
    pub fn new (renderer: Renderer, input_handler: InputHandler) ->Self {
        Self {
            grid: vec![vec![Cell::Empty; GRID_WIDTH]; GRID_HEIGHT],
            pacman: Character::new(),
            ghosts: vec![Character::new(); NUMBER_OF_GHOSTS],
            renderer,
            input_handler,
        }

    }
}