mod game;
mod renderer;
mod debug;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().expect("sdl2 init failed");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = game::Game::new(&sdl_context, &ttf_context);

    game.run(event_pump);
    Ok(())
}
