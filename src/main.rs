extern crate tcod;
extern crate verbonia;

use verbonia::gui::{Console, GUI};
use verbonia::engine::Game;
use verbonia::util::units::Size;

fn main() {
    let game = Game::new();
    let console = Console::new(Size::new(80, 50));
    let mut gui = GUI::new(game, console);

    gui.run();
}
