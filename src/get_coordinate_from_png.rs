use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use image::{imageops::resize, imageops::FilterType, ImageBuffer, RgbaImage};
use slint::Image;
use slint::Rgba8Pixel;
use slint::SharedPixelBuffer;
use std::path::PathBuf;

pub fn get_coordinate_from_png() -> anyhow::Result<()> {
    println!("start capture viewer");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let input_path = PathBuf::from("img/temp/20240115_102922.png");
    let image =
        image::open(&input_path).expect(&format!("Could not load image at {:?}", input_path));

    // リサイズ
    let new_width = image.width() / 3;
    let new_height = image.height() / 3;
    let img_buf: ImageBuffer<_, _> = resize(&image, new_width, new_height, FilterType::Nearest);

    // slint::Imageへの変換
    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        img_buf.as_raw(),
        img_buf.width(),
        img_buf.height(),
    );
    let image = Image::from_rgba8(buffer);

    // 画面の設定
    let ui = MainWindow::new()?;
    ui.set_window_width(new_width as f32);
    ui.set_window_height(new_height as f32);
    ui.set_screen(image);

    let weak = ui.as_weak();

    ui.on_clicked(move || {
        let app = weak.upgrade().unwrap();
        println!("Clicked");
        let x = app.get_mouse_x() * 3.0;
        let y = app.get_mouse_y() * 3.0;

        println!("x:{:?},y:{:?}", x, y);
        let content: String = format!("x:{:?},y:{:?}", x as u32, y as u32);
        ctx.set_contents(content).unwrap();
    });

    ui.run()?;

    Ok(())
}

slint::slint! {
    export component MainWindow inherits Window {
        out property<length> mouse_x;
        out property<length> mouse_y;
        callback clicked;
        in property window_width <=> self.width;
        in property window_height <=> self.height;

        in property<image> screen;

        Image {
            source: screen;
        }

        TouchArea {
            clicked => {
                root.mouse_x = self.pressed-x;
                root.mouse_y = self.pressed-y;
                root.clicked();

            }
        }
    }

}
