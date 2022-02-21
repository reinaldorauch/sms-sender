use fltk::{app::App, app::Scheme::Gtk, prelude::*, window::Window, frame::Frame, button::Button};

fn main() {
    let mut counter: u8 = 0;
    let app = App::default().with_scheme(Gtk);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    but.set_callback(move |_| {
        let label = format!("Hello world: {}", counter);
        frame.set_label(label.as_str());
        counter += 1;
    });
    app.run().unwrap();
}