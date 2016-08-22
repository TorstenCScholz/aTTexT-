extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, RectangleShape, Transformable, Shape, Font, Text};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::{Vector2f};

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "Attext!",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    let mut input = String::from("");

    let font = Font::new_from_file("arial.ttf").unwrap();
    let mut text = Text::new().unwrap();

    text.set_character_size(20);
    text.set_font(&font);
    text.set_position2f(0f32, 580f32);

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code: Key::Escape, .. } => return,
                event::TextEntered { code } => {
                    input.push(code);
                    text.set_string(&input);
                }
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&text);
        window.display();
    }
}
