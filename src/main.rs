use ggez::event::{self, EventHandler};
use ggez::{graphics, conf};
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("rusty_hangman", "guppy")
        .default_conf(conf::Conf::new())
        .build()
        .expect("Could not create ggez context!");

    let state = State {
      deltaTime: std::time::Duration::new(0, 0),
    };

    event::run(ctx, event_loop, state);
}

struct State {
  // The time each frame has taken.
  deltaTime: std::time::Duration,
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult  {
        self.deltaTime = ctx.time.delta();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello ggez! dt = {}ns", self.deltaTime.as_nanos());
        Ok(())
    }
}