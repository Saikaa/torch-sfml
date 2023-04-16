use std::iter::zip;

use sfml::{
    graphics::{
        Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape,
        Text, Transformable, 
    },
    window::{ContextSettings, Event, Key, Style},
};

use crate::{array_to_tensor, Model};

const WINDOW_SIZE: (u32, u32) = (1200, 600);
const BOARD_SIZE: (f32, f32) = (28., 28.);
const BOARD_POSITION: (f32, f32) = (50., 50.);
const IMAGE_SIZE: usize = BOARD_SIZE.0 as usize * BOARD_SIZE.1 as usize;
const PIXEL_SIZE: f32 = 18.;
const FONT: &'static [u8] = include_bytes!("../../resources/mesloLG.otf");

struct Board<'a> {
    data: [f32; IMAGE_SIZE],
    pixels: Vec<Vec<RectangleShape<'a>>>,
}

impl Board<'_> {
    fn new() -> Self {
        let mut pixels = Vec::new();
        for y in 0..28 {
            let mut row_pixel = Vec::new();
            for x in 0..28 {
                let mut pixel = RectangleShape::new();
                pixel.set_fill_color(Color::BLACK);
                pixel.set_position((
                    PIXEL_SIZE * x as f32 + BOARD_POSITION.0,
                    PIXEL_SIZE * y as f32 + BOARD_POSITION.1,
                ));
                pixel.set_size((PIXEL_SIZE, PIXEL_SIZE));

                row_pixel.push(pixel);
            }

            pixels.push(row_pixel);
        }
        Self {
            data: [0.; IMAGE_SIZE],
            pixels,
        }
    }

    fn set_pixel_white(&mut self, x: usize, y: usize) {
        self.data[y * 28 + x % 28] = 1.;
        self.pixels[y][x].set_fill_color(Color::WHITE);
    }

    fn clear(&mut self) {
        for y in 0..28 {
            for x in 0..28 {
                self.data[y * 28 + x % 28] = 0.;
                self.pixels[y][x].set_fill_color(Color::BLACK);
            }
        }
    }

    fn draw(&self, window: &mut RenderWindow) {
        for row in &self.pixels {
            for pixel in row {
                window.draw(pixel);
            }
        }
    }
}

fn within_board(mouse_x: i32, mouse_y: i32) -> bool {
    let mouse_x = mouse_x as f32;
    let mouse_y = mouse_y as f32;
    //let offset =
    mouse_x > BOARD_POSITION.0
        && mouse_x < BOARD_POSITION.0 + BOARD_SIZE.0 * PIXEL_SIZE
        && mouse_y > BOARD_POSITION.1
        && mouse_y < BOARD_POSITION.1 + BOARD_SIZE.1 * PIXEL_SIZE
}

fn get_board_pixel(mouse_x: i32, mouse_y: i32) -> Option<(usize, usize)> {
    if !within_board(mouse_x, mouse_y) {
        return None;
    }

    let board_x = (mouse_x as f32 - BOARD_POSITION.0) / PIXEL_SIZE;
    let board_y = (mouse_y as f32 - BOARD_POSITION.1) / PIXEL_SIZE;

    Some((f32::floor(board_x) as usize, f32::floor(board_y) as usize))
}

pub fn init_window() {
    let context_settings = ContextSettings {
        antialiasing_level: 1,
        ..Default::default()
    };

    let mut window = RenderWindow::new(
        WINDOW_SIZE,
        "road to talking menhera",
        Style::CLOSE,
        &context_settings,
    );
    window.set_framerate_limit(30);

    let context_settings = window.settings();
    if context_settings.antialiasing_level > 0 {
        println!("Using {}xAA", context_settings.antialiasing_level);
    }
    //window.set_vertical_sync_enabled(true);

    let font = unsafe { Font::from_memory(FONT).unwrap() };

    let mut rect = RectangleShape::new();
    rect.set_size((
        (PIXEL_SIZE * BOARD_SIZE.0) as f32,
        (PIXEL_SIZE * BOARD_SIZE.1) as f32,
    ));
    rect.set_fill_color(Color::BLACK);
    rect.set_outline_thickness(3.);
    rect.set_outline_color(Color::WHITE);
    rect.set_origin((-50., -50.));

    let mut mouse_down = false;

    let mut board = Board::new();

    // AI stuff
    let mut probabilities_text = Vec::new();

    for i in 0..10 {
        let mut text = Text::default();
        text.set_font(&font);
        text.set_character_size(40);
        text.set_position((650., 50. * (i as f32 + 1.)));
        text.set_fill_color(Color::WHITE);
        text.set_string(format!("{i: >2}: 0.00%").as_str());

        probabilities_text.push(text);
    }

    let mut model = Model::new("./model_scripted.pth").unwrap();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape | Key::Q,
                    ..
                } => return,
                Event::KeyPressed { code: Key::C, .. } => {
                    board.clear();
                }
                Event::MouseButtonPressed { .. } => {
                    mouse_down = true;
                }
                Event::MouseButtonReleased { .. } => {
                    mouse_down = false;
                }
                Event::MouseMoved { x, y, .. } => {
                    if mouse_down {
                        if let Some((x, y)) = get_board_pixel(x, y) {
                            board.set_pixel_white(x, y);
                            model.predict(&array_to_tensor(&board.data));
                        }
                    }
                }
                _ => {}
            };
        }

        window.clear(Color::BLACK);
        window.draw(&rect);
        board.draw(&mut window);

        for (i, (probability, text)) in
            zip(&model.probabilities, &mut probabilities_text).enumerate()
        {
            let probability = probability * 100.;

            //text.set_fill_color(Color::WHITE);
            text.set_string(format!("{i: >2}: {probability:.2}%").as_str());
            window.draw(text);
        }

        // if model.highest_probability > 0. {
        // let highest_text = &mut probabilities_text[model.predicted];
        // highest_text.set_fill_color(Color::GREEN);
        // window.draw(highest_text);
        // }

        window.display();
    }
}
