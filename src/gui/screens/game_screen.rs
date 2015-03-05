use engine::{Game, MessageType, Tile};
use gui::screens::{self, Screen, ScreenChange};
use gui::{Console, Colors, Key};
use gui::chars;
use util::units::{AsTuple, Direction, Offset, Point, Rectangle, Size};

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    map_frame: Rectangle,
    map_view: Point,
    info_frame: Rectangle,
    message_frame: Rectangle,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            GameScreen {
                map_frame: Rectangle::new(Point::new(16, 1), Size::new(63, 38)),
                map_view: Point::new(0, 0),
                info_frame: Rectangle::new(Point::new(1, 1), Size::new(13, 48)),
                message_frame: Rectangle::new(Point::new(16, 41), Size::new(63, 8)),
            }
        )
    }
}

impl Screen for GameScreen {

    #[allow(unused)]
    fn input(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        if let Some(key) = console.check_for_keypress() {
            match key {
                Key::Up => {
                    game.world.walk(Direction::Up);
                },
                Key::Down => {
                    game.world.walk(Direction::Down);
                },
                Key::Left => {
                    game.world.walk(Direction::Left);
                },
                Key::Right => {
                    game.world.walk(Direction::Right);
                },
                Key::Escape => {
                    return Some(ScreenChange::AddScreen(screens::PauseScreen::new()));
                },
                _ => {}
            }
        }

        None
    }

    #[allow(unused)]
    fn update(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        None
    }

    #[allow(unused)]
    fn render(&mut self, game: &mut Game, console: &mut Console) {
        self.draw_borders(game, console);
        self.draw_map(game, console);
        self.draw_player(game, console);
        self.draw_messages(game, console);
    }
}

impl GameScreen {

    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        self.draw_box_with_title(console, "Map", self.map_frame.translate(Offset::new(-1, -1)).resize(Offset::new(1, 1)));
        self.draw_box_with_title(console, "Info", self.info_frame.translate(Offset::new(-1, -1)).resize(Offset::new(1, 1)));
        self.draw_box_with_title(console, "Messages", self.message_frame.translate(Offset::new(-1, -1)).resize(Offset::new(1, 1)));
    }

    fn draw_box_with_title(&self, console: &mut Console, title: &str, rect: Rectangle) {

        let loc = rect.location();
        let width = rect.width();
        let height = rect.height();

        console.put_plain(loc + (0, 0),             chars::NW);
        console.put_plain(loc + (width, 0),         chars::NE);
        console.put_plain(loc + (0, height),        chars::SW);
        console.put_plain(loc + (width, height),    chars::SE);

        for x in 1..width {
            console.put_plain(loc + (x, 0),         chars::HLINE);
            console.put_plain(loc + (x, height),    chars::HLINE);
        }

        for y in 1..height {
            console.put_plain(loc + (0, y),         chars::VLINE);
            console.put_plain(loc + (width, y),     chars::VLINE);
        }

        console.print_plain(rect.location() + (1, 0), title);
    }

    #[allow(unused)]
    fn draw_map(&self, game: &mut Game, console: &mut Console) {
        let map = &game.world.map;

        let (ux, uy): (usize, usize) = self.map_view.as_tuple();
        let (width, height) = self.map_frame.size().as_tuple();

        for (y, line) in map.tiles[uy .. uy + height].iter().enumerate() {
            for (x, cell) in line[ux .. ux + width].iter().enumerate() {
                let bg_color = match *cell {
                    Tile::Empty => Colors::black,
                    Tile::Wall => Colors::darker_grey,
                    Tile::Floor => Colors::darkest_sepia,
                };

                console.put(self.map_frame.location() + Point::new(x as i32, y as i32), ' ', Colors::white, bg_color);
            }
        }
    }

    #[allow(unused)]
    fn draw_player(&mut self, game: &mut Game, console: &mut Console) {
        let pos = *game.world.player.pos();

        let adjusted_pos = pos + self.map_frame.location() - self.map_view;

        if adjusted_pos.x >= self.map_frame.location().x + self.map_frame.width() - 10 {
            if self.view_can_move_right(game) {
                self.map_view = self.map_view.right(1);
            }
        }

        if adjusted_pos.y >= self.map_frame.location().y + self.map_frame.height() - 5 {
            if self.view_can_move_down(game) {
                self.map_view = self.map_view.down(1);
            }
        }

        if adjusted_pos.y <= self.map_frame.location().y + 5 {
            if self.view_can_move_up(game) {
                self.map_view = self.map_view.up(1);
            }
        }

        if adjusted_pos.x <= self.map_frame.location().x + 10 {
            if self.view_can_move_left(game) {
                self.map_view = self.map_view.left(1);
            }
        }

        if adjusted_pos.x >= self.map_frame.location().x && adjusted_pos.y >= self.map_frame.location().y {
            console.put_plain(self.map_frame.location() - self.map_view + pos, '@');
        }
    }

    #[allow(unused)]
    fn draw_messages(&mut self, game: &mut Game, console: &mut Console) {
        let nmessages = self.message_frame.height() as usize;

        for (i, msg) in game.log.items().take(nmessages).enumerate() {
            let message_color = match *msg.ty() {
                MessageType::Info => Colors::white,
                MessageType::Error => Colors::red,
            };

            console.print(self.message_frame.location().down(i as i32), msg.text(), message_color, Colors::black);
        }
    }

    #[allow(unused)]
    fn view_can_move_up(&self, game: &Game) -> bool {
        self.map_view.y  > 0
    }

    #[allow(unused)]
    fn view_can_move_down(&self, game: &Game) -> bool {
        self.map_view.y + self.map_frame.height() < game.world.map.height()
    }

    #[allow(unused)]
    fn view_can_move_left(&self, game: &Game) -> bool {
        self.map_view.x > 0
    }

    #[allow(unused)]
    fn view_can_move_right(&self, game: &Game) -> bool {
        self.map_view.x + self.map_frame.width() < game.world.map.width()
    }

}
