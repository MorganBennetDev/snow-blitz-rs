mod interface;

use sfml::system::{
    Vector2f,
    Clock,
    Time
};
use sfml::window::{
    ContextSettings,
    Event,
    Style,
    VideoMode
};
use sfml::graphics::{
    Color,
    Font,
    RectangleShape,
    Transformable,
    Shape,
    RenderTarget,
    RenderWindow,
};

use interface::{
    button::{
        Button,
        ButtonConfig
    }
};

const FPS: i64 = 30;
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    let frame_time = Time::seconds(1.) / FPS;
    let mut clock = Clock::start();

    let font = match Font::from_file("resources/fonts/arial.ttf") {
        Some(font) => font,
        None => {
            panic!("Failed to read font file!");
        }
    };
    
    // let mut text = Text::new("Snow goes here", &font, 24);

    // text.set_position(Vector2f::new((WIDTH as f32) / 2., (HEIGHT as f32) / 2.));
    // text.set_fill_color(Color::GREEN);

    // Create the window of the application
    let mut window : RenderWindow = RenderWindow::new(
        VideoMode::new(WIDTH, HEIGHT, 32),
        "Snow Blitz",
        Style::CLOSE,
        &ContextSettings::default()
    );

    let button_config = ButtonConfig::new(
        "O",
        48,
        Vector2f::new(100., 100.),
        Color::rgb(255, 0, 0),
        Color::rgb(0, 255, 0),
        Color::rgb(0, 0, 255),
        2.,
        2.
    );

    let b = Button::new(
        Vector2f::new(200., 200.),
        &button_config,
        &button_config,
        Box::new(|| print!("Hi")),
        &font
    );

    while window.is_open() {
        if clock.elapsed_time() >= frame_time {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => window.close(),
                    _ => {}
                }
            }

            window.clear(Color::WHITE);
            
            b.draw(&mut window);

            window.display();
            clock.restart();
        }
    }
}
