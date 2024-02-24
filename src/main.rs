use sfml::system::{
    Vector2f,
    Clock,
    Time
};
use sfml::window::{
    ContextSettings,
    VideoMode,
    Event,
    Style
};
use sfml::graphics::{
    RenderWindow,
    RenderTarget,
    CircleShape,
    Color,
    Transformable,
    Shape
};

const FPS: i64 = 30;

fn main() {
    let frame_time = Time::seconds(1.) / FPS;
    let mut clock = Clock::start();
    
    // Create the window of the application
    let mut window : RenderWindow = RenderWindow::new(VideoMode::new(800, 600, 32),
                                                      "SFML Example",
                                                      Style::CLOSE,
                                                      &ContextSettings::default());

    // Create a CircleShape
    let mut circle = CircleShape::new(1.0, 12);
    circle.set_radius(30.);
    circle.set_fill_color(Color::BLUE);
    circle.set_position(Vector2f::new(100., 100.));

    while window.is_open() {
        if clock.elapsed_time() >= frame_time {
            // Handle events
            for event in window.poll_event() {
                match event {
                    Event::Closed => window.close(),
                    _             => {/* do nothing */}
                }
            }
            
            // Clear the window
            window.clear(Color::RED);
            // Draw the shape
            window.draw(&circle);
            // Display things on screen
            window.display();
            clock.restart();
        }
    }
}
