extern crate sfml;

use sfml::graphics::{Color, RenderTarget, RenderWindow, RectangleShape, Transformable, Shape, Font, Text, Drawable, RenderStates};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::{Vector2f};

struct InputField<'a> {
    input: String,
    position: Vector2f,
    max_len: usize,
    texture: Text<'a>,
    background: RectangleShape<'a>,
    cursor: RectangleShape<'a>
}

impl<'a> InputField<'a> {
    fn new(font: &'a Font) -> InputField<'a> {
        let background_size = Vector2f::new(200., 40.);
        let mut background = RectangleShape::new().unwrap();
        background.set_size(&background_size);
        background.set_fill_color(&Color::new_rgb(173, 255, 47));

        let cursor_size = Vector2f::new(5., 25.);
        let mut cursor = RectangleShape::new().unwrap();
        cursor.set_size(&cursor_size);
        cursor.set_fill_color(&Color::white());

        let mut text = Text::new_init("", font, 16).unwrap();
        text.set_color(&Color::black());

        InputField {
            input: String::from(""),
            position: Vector2f::new(0., 0.),
            max_len: 25,
            texture: text,
            background: background,
            cursor: cursor
        }
    }

    fn handle_input(&mut self, code: char) {
        if code == '\x08' { // Backspace has been pressed
            if self.input.len() > 0 {
                self.input.pop();
            }
            return;
        }

        if self.input.len() >= self.max_len {
            return;
        }

        self.input.push(code);
    }

    fn update(&mut self) {
        self.texture.set_string(&self.input);

        // TODO: Workaround. How can we do this better?
        // If we replace 'position' with the direct function call, we get a compiler error:
        // error: cannot use `self.position` because it was mutably borrowed [E0503]
        // See also: https://github.com/rust-lang/rfcs/issues/811
        let position = self.position;
        self.set_position(&position);

        let bounds = self.texture.get_global_bounds();
        self.cursor.set_position2f(bounds.left + bounds.width, bounds.top);
    }

    fn set_position<'b>(&mut self, position: &Vector2f) {
        self.position = *position;

        self.texture.set_position(position);
        self.background.set_position(position);
    }
}

impl<'a> Drawable for InputField<'a> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, _: &mut RenderStates) {
        render_target.draw(&self.background);
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

    let font = Font::new_from_file("VT323-Regular.ttf").unwrap();

    let mut input_field = InputField::new(&font);
    input_field.set_position(&Vector2f::new(0., 560.));

    loop {
        // Handle input
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code: Key::Escape, .. } => return,
                event::TextEntered { code } => {
                    input_field.handle_input(code);
                }
                _ => {}
            }
        }

        // Update logic
        input_field.update();

        // Render
        window.clear(&Color::black());
        window.draw(&input_field);
        window.display();
    }
}
