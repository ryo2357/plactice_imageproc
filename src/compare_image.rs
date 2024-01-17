use image::buffer::ConvertBuffer;
use image::{GrayImage, RgbImage};
use std::fs;
use std::path::PathBuf;

pub fn execute() {
    let folder_path = "./img/compare";
    // フォルダ内のエントリを取得

    let mut image_path_vec: Vec<PathBuf> = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            image_path_vec.push(file_path);
        }
    } else {
        println!("Failed to read directory");
    }
    println!("len:{}", image_path_vec.len());

    for i in 0..image_path_vec.len() {
        for j in i..image_path_vec.len() {
            let image_i = image::open(&image_path_vec[i]).unwrap().to_rgb8();
            let image_j = image::open(&image_path_vec[j]).unwrap().to_rgb8();

            let result = calculate_similarity(&image_i, &image_j);
            let result2 = calculate_similarity_gray(image_i, image_j);
            println!(
                "{:?} - {:?} : result:{:?},gray_result:{:?}",
                &image_path_vec[i], &image_path_vec[j], result, result2
            );
        }
    }
}

fn calculate_similarity(image1: &image::RgbImage, image2: &image::RgbImage) -> f64 {
    // ズレには弱いが十分な速度で比較できる
    // 画像のサイズを取得
    let (width, height) = image1.dimensions();

    // 各ピクセルの差分を計算
    let mut diff_sum = 0.0;
    for y in 0..height {
        for x in 0..width {
            let pixel1 = image1.get_pixel(x, y).0[0] as f64;
            let pixel2 = image2.get_pixel(x, y).0[0] as f64;
            diff_sum += (pixel1 - pixel2).abs();
        }
    }

    // 平均差分を計算
    let avg_diff = diff_sum / (width * height) as f64;

    // 類似度を算出（1に近いほど類似）
    let similarity = 1.0 - avg_diff / 255.0;

    return similarity;
}

fn calculate_similarity_gray(image1: image::RgbImage, image2: image::RgbImage) -> f64 {
    // ズレには弱いが十分な速度で比較できる
    // 画像のサイズを取得
    let (width, height) = image1.dimensions();
    let image1: GrayImage = image1.convert();
    let image2: GrayImage = image2.convert();

    // 各ピクセルの差分を計算
    let mut diff_sum = 0.0;
    for y in 0..height {
        for x in 0..width {
            let pixel1 = image1.get_pixel(x, y).0[0] as f64;
            let pixel2 = image2.get_pixel(x, y).0[0] as f64;
            diff_sum += (pixel1 - pixel2).abs();
        }
    }

    // 平均差分を計算
    let avg_diff = diff_sum / (width * height) as f64;

    // 類似度を算出（1に近いほど類似）
    let similarity = 1.0 - avg_diff / 255.0;

    return similarity;
}
