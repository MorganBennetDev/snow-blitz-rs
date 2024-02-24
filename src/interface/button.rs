use sfml::{
    graphics::{
        Color,
        ConvexShape,
        Font,
        RenderTarget,
        RenderWindow,
        Shape,
        Text,
        Transformable,
    },
    system::{
        Vector2f,
    },
};

pub struct ButtonConfig<'a> {
    text: &'a str,
    text_size: u32,
    dimensions: Vector2f,
    text_color: Color,
    color: Color,
    stroke_color: Color,
    stroke_width: f32,
    corner_radius: f32
}

impl<'a> ButtonConfig<'a> {
    pub fn new(text: &'a str, text_size: u32, dimensions: Vector2f, text_color: Color,
        color: Color, stroke_color: Color, stroke_width: f32, corner_radius: f32) -> ButtonConfig {
            return ButtonConfig {
                text,
                text_size,
                dimensions,
                text_color,
                color,
                stroke_color,
                stroke_width,
                corner_radius,
            }
    }
}

pub struct Button<'a> {
    position: Vector2f,
    base: &'a ButtonConfig<'a>,
    hover: &'a ButtonConfig<'a>,
    action: Box<dyn Fn()>,
    shape: ConvexShape<'a>,
    text: Text<'a>,
    font: &'a Font,
}

impl<'a> Button<'a> {
    pub fn new(position: Vector2f, base: &'a ButtonConfig, hover: &'a ButtonConfig, action: Box<dyn Fn()>, font: &'a Font) -> Button<'a> {
        let mut shape = ConvexShape::new(4);

        shape.set_origin(base.dimensions / 2.);

        shape.set_point(0, Vector2f::new(0., 0.));
        shape.set_point(1, Vector2f::new(base.dimensions.x, 0.));
        shape.set_point(2, Vector2f::new(base.dimensions.x, base.dimensions.y));
        shape.set_point(3, Vector2f::new(0., base.dimensions.y));

        shape.set_outline_thickness(base.stroke_width);

        shape.set_outline_color(base.stroke_color);

        shape.set_fill_color(base.color);

        shape.set_position(position);

        let mut text = Text::new(base.text, font, base.text_size);

        let text_bounds = text.local_bounds();

        text.set_origin(Vector2f::new(text_bounds.width / 2. + text_bounds.left, text_bounds.height / 2. + text_bounds.top));

        text.set_fill_color(base.text_color);

        text.set_position(position);

        return Button {
            position,
            base,
            hover,
            action,
            shape,
            text,
            font,
        }
    }
    pub fn draw(&self, window: &'a mut RenderWindow) {
        window.draw(&self.shape);
        window.draw(&self.text)
    }
}