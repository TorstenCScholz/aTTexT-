extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, RectangleShape, Transformable, Shape, Font, Text, Drawable, RenderStates};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::{Vector2f};

struct InputField<'a> {
    input: String,
    texture: Text<'a>,
    cursor: RectangleShape<'a>
}

impl<'a> InputField<'a> {
    pub fn new(font: &'a Font) -> InputField<'a> {
        let cursor_size = Vector2f::new(5., 25.);
        let mut cursor = RectangleShape::new().unwrap();
        cursor.set_size(&cursor_size);
        cursor.set_fill_color(&Color::white());

        InputField {
            input: String::from(""),
            texture: Text::new_init("", font, 20).unwrap(),
            cursor: cursor
        }
    }

    pub fn handle_input(&mut self, code: char) {
        if code == '\x08' { // Backspace has been pressed
            if self.input.len() > 0 {
                self.input.pop();
            }
            return;
        }

        self.input.push(code);
    }

    pub fn update_texture(&mut self) {
        self.texture.set_string(&self.input);

        let bounds = self.texture.get_global_bounds();
        self.cursor.set_position2f(bounds.left + bounds.width, bounds.top);
    }
}

impl<'a> Drawable for InputField<'a> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, _: &mut RenderStates) {
        render_target.draw(&self.texture);
        render_target.draw(&self.cursor);
    }
}

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "Attext!",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    let font = Font::new_from_file("arial.ttf").unwrap();

    let mut input_field = InputField::new(&font);

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code: Key::Escape, .. } => return,
                event::TextEntered { code } => {
                    input_field.handle_input(code);
                    input_field.update_texture();
                }
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&input_field);
        window.display();
    }
}
