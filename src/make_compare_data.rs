use std::fs;
use std::path::PathBuf;

struct Coordinate {
    x: u32,
    y: u32,
}

// size: 180,180
pub fn execute() {
    let input_path = PathBuf::from("img/temp/20240115_102922.png");
    let output_dir = PathBuf::from("img/compare");

    if !output_dir.is_dir() {
        fs::create_dir(&output_dir).expect("Failed to create output directory")
    }

    let image = image::open(&input_path)
        .expect(&format!("Could not load image at {:?}", input_path))
        .to_rgb8();

    let mut list: Vec<Coordinate> = Vec::new();
    list.push(Coordinate { x: 366, y: 1197 });
    list.push(Coordinate { x: 540, y: 1182 });
    list.push(Coordinate { x: 189, y: 1185 });
    list.push(Coordinate { x: 342, y: 1536 });
    list.push(Coordinate { x: 522, y: 807 });
    list.push(Coordinate { x: 303, y: 1977 });
    list.push(Coordinate { x: 612, y: 1983 });
    list.push(Coordinate { x: 306, y: 1980 });

    for (index, item) in list.iter().enumerate() {
        let save_path = output_dir.clone().join(format!("{:03}.png", index));
        let save_img = cutout_image(&image, item.x, item.y);
        save_img.save(&save_path).unwrap();
    }
}

fn cutout_image(image: &image::RgbImage, x: u32, y: u32) -> image::RgbImage {
    let w = 180;
    let h = 180;

    assert!(
        x + w < image.width() && y + h < image.height(),
        "invalid sub-image"
    );

    let mut result = image::RgbImage::new(w, h);
    for sy in 0..h {
        for sx in 0..w {
            result.put_pixel(sx, sy, *image.get_pixel(x + sx, y + sy));
        }
    }
    result
}
