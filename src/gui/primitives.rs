use gui::chars;
use gui::Console;
use util::units::BorderedRectangle;
use util::units::Point;
use gui::{Color, Colors};

pub fn draw_box(console: &mut Console, rect: BorderedRectangle) {
    let loc = rect.location();
    let width = rect.width();
    let height = rect.height();

    console.put_plain(loc + (0, 0),             '\u{6}');
    console.put_plain(loc + (width, 0),         '\u{4}');
    console.put_plain(loc + (0, height),        '\u{5}');
    console.put_plain(loc + (width, height),    '\u{7}');

    for x in 1..width {
        console.put_plain(loc + (x, 0),         '\u{2}');
        console.put_plain(loc + (x, height),    '\u{1}');
    }

    for y in 1..height {
        console.put_plain(loc + (0, y),         '\u{3}');
        console.put_plain(loc + (width, y),     '\u{0}');
    }
}

pub fn draw_box_with_title(console: &mut Console, title: &str, rect: BorderedRectangle) {
    draw_box(console, rect);
    console.put_plain(rect.location() + (2, 0), '\u{4}');
    console.put_plain(rect.location() + Point::new(title.len() as i32 + 5, 0), '\u{6}');
    console.put_plain(rect.location() + (3, 0), '\u{81}');
    console.put_plain(rect.location() + Point::new(title.len() as i32 + 4, 0), '\u{82}');
    console.print(rect.location() + (4, 0), title, Colors::BLACK, Colors::Color::new(236, 229, 206));
}
