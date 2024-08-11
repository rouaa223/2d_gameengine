#2D_game_engine
#Rust
#Web_Assembly
#Pong Game
#Pacman_Game
//download the project using git command
git clone github_url
//to run the pacman game navigate to the project location using command line then run the following command 
devserver
// to run the pong game firstly replace this line in lib.rs 
let game = Rc::new(RefCell::new(PacmanGame::new(context, canvas)));
//with this line
let game = Rc::new(RefCell::new(Pong::new(context, canvas)));
//then run the following commands
cargo clean 
wasm-pack build --target web 
devserver
