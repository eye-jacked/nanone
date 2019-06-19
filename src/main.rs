use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;

    // Clear the background to Honeydew
    draw.background().color(Rgb::new(0.94,1.00,0.94));

    // draw a circle of circles
    let circle_color =
                // = match (t as i32) % 2{
               // 1 => Rgb::new(1.0,0.16,0.0),
               // 0 => Rgb::new(1.0,0.13,0.32),
               // _ => Rgb::new(1.0,1.0,1.0)
    //};*/
    Rgb::new(1.0,0.145 + 0.03*t.sin(),0.18 + 0.18*t.sin());

    for i in 0..6 {
        draw.ellipse()
            .x_y(
                ((i as f32) + t).sin() * 200.0 + 0.3*app.mouse.x,
                ((i as f32) + t).cos() * 200.0 + 0.3*app.mouse.y,
            )
            .color(circle_color)
            .radius(0.06 * win.w());
    }

    // will this work?
    

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
