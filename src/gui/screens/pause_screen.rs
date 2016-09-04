use engine::Game;
use gui::{Console, Key, Menu, MenuOption};
use gui::screens::{Screen, ScreenChange};
use util::units::Point;

pub struct PauseScreen {
    menu: Menu<PauseMenu>,
}

enum PauseMenu {
    Resume,
    Exit,
}

impl PauseScreen {
    pub fn new() -> Box<Screen> {
        Box::new(
            PauseScreen {
                menu: Menu::new(vec![
                                    MenuOption("Resume Game", PauseMenu::Resume),
                                    MenuOption("Exit Game", PauseMenu::Exit),
                                ]),
            }
        )
    }
}

impl Screen for PauseScreen {
    #[allow(unused)]
    fn input(&mut self, game: &mut Game, console: &mut Console) -> Option<ScreenChange> {
        if let Some(key) = console.check_for_keypress() {
            match key {
                Key::Up => {
                    self.menu.prev();
                }
                Key::Down => {
                    self.menu.next();
                }
                Key::Enter => {
                    match *self.menu.selected().option() {
                        PauseMenu::Resume => return Some(ScreenChange::RemoveScreen),
                        PauseMenu::Exit => return Some(ScreenChange::ExitGame),
                    }
                },
                Key::Escape => return Some(ScreenChange::RemoveScreen),
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
        console.print_plain(Point::new(0, 0), "Paused");

        let longest_item = self.menu.items()
                                    .max_by_key(|item| item.text().len())
                                    .expect("No Items found")
                                    .text().len() as i32;

        let menu_location_x = console.size().x / 2 - longest_item / 2;
        let menu_location_y = console.size().y / 2 - self.menu.items().count() as i32 / 2;
        let menu_location = Point::new(menu_location_x, menu_location_y);

        for (i, menu_item) in self.menu.items().enumerate() {
            console.print_plain(menu_location.down(i as i32).right(2), menu_item.text());
            if self.menu.is_selected(i) {
                console.put_plain(menu_location.down(i as i32), '>');
            }
        }
    }
}
