use fltk::{app, frame, image as fl_image, prelude::*, window};
use image::GenericImageView;

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "../glow/"]
struct Asset;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(800, 400);
    let mut frame1 = frame::Frame::new(0, 0, 400, 400, "");
    let mut frame2 = frame::Frame::new(400, 0, 400, 400, "");
    wind.end();
    wind.show();

    let img1 = Asset::get("ex.jpg").unwrap();
    let mut jpg = fl_image::JpegImage::from_data(&img1).unwrap();
    jpg.scale(frame1.width(), frame1.height(), false, true);

    let img2 = image::open("../glut/ex.png").unwrap();
    let (x, y) = img2.dimensions();
    let mut rgb = fl_image::RgbImage::new(&img2.to_bytes(), x, y, 4).unwrap();

    frame1.set_image(Some(jpg));
    frame2.draw2(move |f| {
        rgb.draw(f.x(), f.y(), f.width(), f.height());
    });

    app.run().unwrap();
}
