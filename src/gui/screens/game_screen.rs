use engine::{Game, Command, MessageType, Tile, Actor};
use engine::log;
use gui::{primitives, popups};
use gui::{Console, Colors, Key, Widget};
use gui::screens::{self, Screen, ScreenChange};
use util::units::{AsTuple, Direction, Point, Size};
use tcod::input as input;
use std::thread;
use std::time::Duration;

#[allow(missing_copy_implementations)]
pub struct GameScreen {
    map: Widget,
    info: Widget,
    messages: Widget,
    map_view: Point,
    x_on_map: i32,
    y_on_map: i32,
    mouse_x: i32,
    mouse_y: i32,
    mut popups: Vec<popups::Popup>,
}

impl GameScreen {
    pub fn new() -> Box<Screen> {
        let info_widget_location = Point::new(0, 1);
        let map_widget_location = Point::new(19, 1);
        let message_widget_location = Point::new(19, 36);

        Box::new(
            GameScreen {
                map: Widget::new(map_widget_location, Size::new(59, 34)),
                info: Widget::new(info_widget_location, Size::new(18, 48)),
                messages: Widget::new(message_widget_location, Size::new(59, 13)),
                map_view: Point::new(0, 0),
                x_on_map: 0,
                y_on_map: 0,
                mouse_x: 0,
                mouse_y: 0,
                popups: Vec::new(),
            }
        )
    }
}

impl Screen for GameScreen {
    #[allow(unused)]
    fn input(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {

        if let Some((_, event)) = input::check_for_event(input::KEY | input::MOUSE) {
            match event {
                input::Event::Key(ref key_state) => {
                    match key_state {
                        // Down => {
                        //     game.do_command(Command::Walk(Direction::Up));
                        // },
                        // Down => {
                        //     game.do_command(Command::Walk(Direction::Down));
                        // },
                        // Left => {
                        //     game.do_command(Command::Walk(Direction::Left));
                        // },
                        // Right => {
                        //     game.do_command(Command::Walk(Direction::Right));
                        // },
                        // Escape => {
                        //     return Some(ScreenChange::AddScreen(screens::PauseScreen::new()));
                        // },
                        _ => {}
                    }
                },

                input::Event::Mouse(ref mouse_state) => {
                    self.mouse_x = mouse_state.cx as i32;
                    self.mouse_y = mouse_state.cy as i32;
                    // println!("{:?}", mouse_state);
                    let selected_actor: Vec<&Actor> = game.world.actors.iter().filter(|actor| actor.pos().x == self.mouse_x && actor.pos().y == self.mouse_y).collect();
                    if selected_actor.len() > 0 {
                        println!("True");
                    }

                    self.x_on_map = self.mouse_x - self.map_view.x - self.map.rect.inner_location().x;
                    self.y_on_map = self.mouse_y - self.map_view.y - self.map.rect.inner_location().y;


                }
            }
        }
        None
    }

    #[allow(unused)]
    fn update(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        game.step();
        None
    }

    #[allow(unused)]
    fn render(&mut self, game: &mut Game, console: &mut Console) {
        self.draw_borders(game, console);
        self.draw_info(game, console);
        self.draw_map(game, console);
        self.draw_player(game, console);
        self.draw_messages(game, console);
        self.draw_popups(game, console);
    }
}

impl GameScreen {
    #[allow(unused)]
    fn draw_popups(&self, game: &mut Game, console: &mut Console) {
        for mut popup in self.popups {
            let mut slice = &game.world.player.name();
            if popup.actor.pos().x == self.x_on_map && popup.actor.pos().y == self.y_on_map && !popup.is_visible {
                for x in 0..slice.len() + 1 {
                    console.print(Point::new(self.mouse_x + 2, self.mouse_y), &slice[..x], Colors::BLACK, Colors::Color::new(236, 229, 206));                    console.flush();
                    thread::sleep(Duration::from_millis(1));
                }
                popup.set_visible(true);
            } else if popup.is_visible {
                console.print(Point::new(self.mouse_x + 2, self.mouse_y), &slice, Colors::BLACK, Colors::Color::new(236, 229, 206));
            }
        }
    }

    #[allow(unused)]
    fn draw_borders(&self, game: &mut Game, console: &mut Console) {
        primitives::draw_box_with_title(console, "Map", self.map.rect);
        primitives::draw_box_with_title(console, "Info", self.info.rect);
        primitives::draw_box_with_title(console, "Messages", self.messages.rect);

        let width = console.size().x;
        let height = console.size().y;

        for x in 0..width {
            for y in 0..height {
                if y == 0 {
                    console.put_plain(Point::new(x, y), '\u{80}');
                } else if x == width - 1 {
                    console.put_plain(Point::new(x, y), '\u{81}');
                }
            }
        }
        console.put_plain(Point::new(width - 1, 0), '\u{8}');
        let bg_color = Colors::Color::new(236, 229, 206);
        console.put_plain(Point::new(3, 0), '\u{8}');
        console.put(Point::new(4, 0), ' ', Colors::WHITE, bg_color);
        console.put(Point::new(5, 0), ' ', Colors::WHITE, bg_color);
        console.put(Point::new(6, 0), ' ', Colors::WHITE, bg_color);
        console.put(Point::new(7, 0), ' ', Colors::WHITE, bg_color);
        console.put_plain(Point::new(8, 0), '\u{9}');

        let map_loc = self.info.rect.size().x + 1;

        console.put_plain(Point::new(map_loc + 3, 0), '\u{8}');
        console.put(Point::new(map_loc + 4, 0), ' ', Colors::WHITE, bg_color);
        console.put(Point::new(map_loc + 5, 0), ' ', Colors::WHITE, bg_color);
        console.put(Point::new(map_loc + 6, 0), ' ', Colors::WHITE, bg_color);
        console.put_plain(Point::new(map_loc + 7, 0), '\u{9}');
    }

    #[allow(unused)]
    fn draw_info(&self, game: &mut Game, console: &mut Console) {
        return;
    }

    #[allow(unused)]
    fn draw_map(&self, game: &mut Game, console: &mut Console) {
        let map = &game.world.map;

        let (ux, uy) = self.map_view.as_tuple();
        let (width, height) : (usize, usize) = self.map.rect.inner_size().as_tuple();

        for (y, line) in map.tiles[uy .. uy + height + 1].iter().enumerate() {
            for (x, cell) in line[ux .. ux + width + 1].iter().enumerate() {
                let bg_color = match *cell {
                    Tile::Empty => Colors::BLACK,
                    Tile::Wall => Colors::DARKER_GREY,
                    Tile::Floor => Colors::DARKEST_SEPIA,
                    Tile::Grass => Colors::DESATURATED_GREEN,
                };

                self.map.put(console, Point::new(x as i32, y as i32), ' ', Colors::WHITE, bg_color);
            }
        }
    }

    #[allow(unused)]
    fn draw_player(&mut self, game: &mut Game, console: &mut Console) {
        let pos = *game.world.player.pos();

        let adjusted_pos = pos + self.map.rect.inner_location() - self.map_view;

        if adjusted_pos.x >= self.map.rect.inner_location().x + self.map.rect.width() - 10 {
            if self.view_can_move_right(game) {
                self.map_view = self.map_view.right(1);
            }
        }

        if adjusted_pos.y >= self.map.rect.inner_location().y + self.map.rect.height() - 5 {
            if self.view_can_move_down(game) {
                self.map_view = self.map_view.down(1);
            }
        }

        if adjusted_pos.y <= self.map.rect.inner_location().y + 5 {
            if self.view_can_move_up(game) {
                self.map_view = self.map_view.up(1);
            }
        }

        if adjusted_pos.x <= self.map.rect.inner_location().x + 10 {
            if self.view_can_move_left(game) {
                self.map_view = self.map_view.left(1);
            }
        }

        if adjusted_pos.x >= self.map.rect.inner_location().x && adjusted_pos.y >= self.map.rect.inner_location().y {
            self.map.put_plain(console, pos - self.map_view, '\u{40}');
        }
    }

    #[allow(unused)]
    fn draw_messages(&mut self, game: &mut Game, console: &mut Console) {
        let nmessages = self.messages.rect.height() as usize;

        log::LOG.with(|w| {
            for (i, msg) in w.borrow().items().take(nmessages).enumerate() {
                let message_color = match *msg.ty() {
                    MessageType::Info => Colors::WHITE,
                    MessageType::Error => Colors::RED,
                };

                self.messages.print(console, Point::new(0, i as i32), msg.text(), message_color, Colors::BLACK);
            }
        });
    }

    #[allow(unused)]
    fn view_can_move_up(&self, game: &Game) -> bool {
        self.map_view.y  > 0
    }

    #[allow(unused)]
    fn view_can_move_down(&self, game: &Game) -> bool {
        self.map_view.y + self.map.rect.height() <= game.world.map.height()
    }

    #[allow(unused)]
    fn view_can_move_left(&self, game: &Game) -> bool {
        self.map_view.x > 0
    }

    #[allow(unused)]
    fn view_can_move_right(&self, game: &Game) -> bool {
        self.map_view.x + self.map.rect.width() <= game.world.map.width()
    }
}
