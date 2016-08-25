extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, RectangleShape, Transformable, Shape, Font, Text};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::{Vector2f};

fn handle_input(code: char, input: &mut String) {
    if code == '\x08' { // Backspace has been pressed
        if input.len() > 0 {
            input.pop();
            return;
        }
    }

    input.push(code);
}

fn render_cursor(window: &mut RenderWindow, cursor: &RectangleShape) {
    window.draw(cursor);
}

fn main() {
    let cursor_size = Vector2f::new(5., 25.);

    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "Attext!",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    let mut input = String::from("");

    let font = Font::new_from_file("arial.ttf").unwrap();
    let mut text = Text::new_init("Ag", &font, 20).unwrap();

    let mut cursor = RectangleShape::new().unwrap();
    cursor.set_size(&cursor_size);
    cursor.set_fill_color(&Color::white());

    let initial_bounds = text.get_global_bounds();
    cursor.set_size(&(Vector2f::new(3., initial_bounds.height)));

    text.set_string("");

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code: Key::Escape, .. } => return,
                event::TextEntered { code } => {
                    handle_input(code, &mut input);

                    text.set_string(&input);

                    let bounds = text.get_global_bounds();
                    cursor.set_position2f(bounds.left + bounds.width, bounds.top);
                }
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&text);
        render_cursor(&mut window, &cursor);
        window.display();
    }
}
