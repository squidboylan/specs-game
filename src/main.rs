use ggez::{self, *};

mod game;
mod renderer;
mod debug;

fn main() {
    let cb = ContextBuilder::new("game-template", "ggez")
        .window_setup(conf::WindowSetup::default().title("game template"))
        .window_mode(conf::WindowMode::default().dimensions(renderer::SCREEN_WIDTH, renderer::SCREEN_HEIGHT));
    let (ctx, ev) = &mut cb.build().unwrap();

    ggez::input::mouse::set_cursor_grabbed(ctx, true);
    ggez::input::mouse::set_cursor_hidden(ctx, true);

    let mut game = game::Game::new(ctx);

    if let Err(e) = event::run(ctx, ev, &mut game) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
