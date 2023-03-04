use ggez::event::{self, EventHandler};
use ggez::{graphics, conf};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("rusty_hangman", "guppy")
        .default_conf(conf::Conf::new())
        .build()
        .expect("Could not create ggez context!");

    let state = State {};

    event::run(ctx, event_loop, state);
}

struct State {}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult  {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}