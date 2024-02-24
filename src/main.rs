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
    RenderTarget,
    RenderWindow,
    Text,
    Transformable,
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
    
    let mut text = Text::new("Snow goes here", &font, 24);

    text.set_position(Vector2f::new((WIDTH as f32) / 2., (HEIGHT as f32) / 2.));
    text.set_fill_color(Color::GREEN);

    // Create the window of the application
    let mut window : RenderWindow = RenderWindow::new(VideoMode::new(WIDTH, HEIGHT, 32),
                                                      "Snow Blitz",
                                                      Style::CLOSE,
                                                      &ContextSettings::default());

    while window.is_open() {
        if clock.elapsed_time() >= frame_time {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed => window.close(),
                    _ => {}
                }
            }

            window.clear(Color::WHITE);
            
            window.draw(&text);

            window.display();
            clock.restart();
        }
    }
}
