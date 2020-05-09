use ggez::{self, *};
use std::path;
use std::env;

mod components;
mod debug;
mod game;
mod renderer;
mod systems;

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    }
    else {
        path::PathBuf::from("./resources")
    };
    let cb = ContextBuilder::new("game-template", "ggez")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("game template"))
        .window_mode(
            conf::WindowMode{ width: renderer::SCREEN_WIDTH, height: renderer::SCREEN_HEIGHT, .. conf::WindowMode::default() }
        );
    let (ctx, ev) = &mut cb.build().unwrap();

    ggez::input::mouse::set_cursor_grabbed(ctx, true).unwrap();
    ggez::input::mouse::set_cursor_hidden(ctx, true);
    ggez::graphics::set_default_filter(ctx, ggez::graphics::FilterMode::Nearest);

    let mut game = game::Game::new(ctx);

    if let Err(e) = event::run(ctx, ev, &mut game) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
