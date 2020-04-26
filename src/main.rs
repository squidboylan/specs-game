mod game;
mod renderer;
mod debug;

fn main() -> Result<(), String> {
    let mut game = game::Game::new();

    game.run();
    Ok(())
}
