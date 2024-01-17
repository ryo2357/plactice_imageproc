use image::{open, GenericImage, GrayImage, Luma, Rgb, RgbImage};
use imageproc::definitions::Image;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::map::map_colors;
use imageproc::rect::Rect;
use imageproc::template_matching::{match_template, MatchTemplateMethod};
use std::env;
use std::f32;
use std::fs;
use std::path::PathBuf;

struct TemplateMatchingArgs {
    template_x: u32,
    template_y: u32,
    template_w: u32,
    template_h: u32,
}

/// Convert an f32-valued image to a 8 bit depth, covering the whole
/// available intensity range.
fn convert_to_gray_image(image: &Image<Luma<f32>>) -> GrayImage {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;

    for p in image.iter() {
        lo = if *p < lo { *p } else { lo };
        hi = if *p > hi { *p } else { hi };
    }

    let range = hi - lo;
    let scale = |x| (255.0 * (x - lo) / range) as u8;
    map_colors(image, |p| Luma([scale(p[0])]))
}

fn copy_sub_image(image: &GrayImage, x: u32, y: u32, w: u32, h: u32) -> GrayImage {
    assert!(
        x + w < image.width() && y + h < image.height(),
        "invalid sub-image"
    );

    let mut result = GrayImage::new(w, h);
    for sy in 0..h {
        for sx in 0..w {
            result.put_pixel(sx, sy, *image.get_pixel(x + sx, y + sy));
        }
    }

    result
}

fn draw_green_rect(image: &GrayImage, rect: Rect) -> RgbImage {
    let mut color_image = map_colors(image, |p| Rgb([p[0], p[0], p[0]]));
    draw_hollow_rect_mut(&mut color_image, rect, Rgb([0, 255, 0]));
    color_image
}

fn run_match_template(
    args: &TemplateMatchingArgs,
    image: &GrayImage,
    template: &GrayImage,
    method: MatchTemplateMethod,
) -> RgbImage {
    // Match the template and convert to u8 depth to display
    println!("before match_template");
    let result = match_template(image, template, method);
    println!("after match_template");
    let result_scaled = convert_to_gray_image(&result);

    // Pad the result to the same size as the input image, to make them easier to compare
    let mut result_padded = GrayImage::new(image.width(), image.height());
    result_padded
        .copy_from(&result_scaled, args.template_w / 2, args.template_h / 2)
        .unwrap();

    // Show location the template was extracted from
    let roi = Rect::at(args.template_x as i32, args.template_y as i32)
        .of_size(args.template_w, args.template_h);

    draw_green_rect(&result_padded, roi)
}

pub fn example_template_match() {
    //     let continue_image = image::open("img/temp/20240115_102922.png")
    //         .unwrap()
    //         .to_luma8();
    //     let reword_image = image::open("img/temp/20240115_162651.png")
    //         .unwrap()
    //         .to_luma8();
    //     let start_image = image::open("img/temp/20240115_162700.png")
    //         .unwrap()
    //         .to_luma8();

    let input_path = PathBuf::from("img/temp/20240115_102922.png");
    let output_dir = PathBuf::from("img/output");

    if !output_dir.is_dir() {
        fs::create_dir(&output_dir).expect("Failed to create output directory")
    }

    if !input_path.is_file() {
        panic!("Input file does not exist");
    }

    // Load image and convert to grayscale
    let image = open(&input_path)
        .expect(&format!("Could not load image at {:?}", input_path))
        .to_luma8();

    println!("open");

    // Extract the requested image sub-region to use as the template
    let template = copy_sub_image(&image, 300, 1560, 750 - 300, 1610 - 1560);
    let template_args = TemplateMatchingArgs {
        template_x: 300,
        template_y: 1560,
        template_w: 750 - 300,
        template_h: 1610 - 1560,
    };

    let template_path = output_dir.join("template.png");
    template.save(&template_path).unwrap();

    println!("copy_sub_image");

    // Match using all available match methods
    // 処理が終わらない
    // let sse = run_match_template(
    //     &template_args,
    //     &image,
    //     &template,
    //     MatchTemplateMethod::SumOfSquaredErrors,
    // );
    // println!("sse");

    let sse_norm = run_match_template(
        &template_args,
        &image,
        &template,
        MatchTemplateMethod::CrossCorrelationNormalized,
    );
    println!("sse_norm");

    // Show location the template was extracted from
    let roi = Rect::at(
        template_args.template_x as i32,
        template_args.template_y as i32,
    )
    .of_size(template_args.template_w, template_args.template_h);

    println!("Rect");

    let image_with_roi = draw_green_rect(&image, roi);

    // Save images to output_dir
    let source_path = output_dir.join("image.png");
    image_with_roi.save(&source_path).unwrap();
    // let sse_path = output_dir.join("result_sse.png");
    // sse.save(&sse_path).unwrap();
    let sse_path = output_dir.join("result_sse_norm.png");
    sse_norm.save(&sse_path).unwrap();
}
// fn main() {
//     // 元の画像とテンプレート画像を読み込む
//     let continue_image = image::open("img/temp/20240115_102922.png")
//         .unwrap()
//         .to_luma8();
//     let reword_image = image::open("img/temp/20240115_162651.png")
//         .unwrap()
//         .to_luma8();
//     let start_image = image::open("img/temp/20240115_162700.png")
//         .unwrap()
//         .to_luma8();

//     // テンプレートマッチングを実行する

//     let result_f32 = match_template(
//         &reword_image,
//         &start_image,
//         MatchTemplateMethod::SumOfSquaredErrorsNormalized,
//     );
//     let result: GrayImage = result_f32;

//     // 結果を保存する
//     result.save("result.png").unwrap();
// }
