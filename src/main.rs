use opencv::core::{BORDER_CONSTANT, Mat, MatTrait, MatTraitManual, Point2f, Scalar, Size, Vector};
use opencv::imgcodecs::{imread, IMREAD_GRAYSCALE, imwrite};
use opencv::imgproc::{get_rotation_matrix_2d, warp_affine, WARP_INVERSE_MAP};
use opencv::text::{OEM_DEFAULT, PSM_AUTO};
use opencv::text::prelude::OCRTesseract;

const SOURCE_IMAGE_PATH: &str = "src_img.png";

fn main() {
    // 処理元の画像を定義
    let src_img = imread(SOURCE_IMAGE_PATH, IMREAD_GRAYSCALE).unwrap();
    if src_img.data().is_err() {
        panic!("Failed to read image.")
    };

    let width = src_img.cols();
    let height = src_img.rows();
    let center = Point2f::new((width/2) as f32, (height/2) as f32);

    let mut angle = 0.0;
    let mut max_chars_count = 0;
    let mut angle_at_max = 0.0;
    loop {
        if angle > 270.0 {
            break; // 4方向(0°, 90°, 180°, 270°)の解析が終わったら、ループを抜ける
        }
        let matrix;
        let result_get_rotation_matrix_2d = get_rotation_matrix_2d(center, angle, 1.0);
        match result_get_rotation_matrix_2d {
            Ok(m) => matrix = m,
            Err(code) => {
                panic!("{}", code)
            }
        }

        let size = Size::new(width, height);

        let mut dst_img = src_img.clone();
        let result_affine = warp_affine(&src_img, &mut dst_img, &matrix, size, WARP_INVERSE_MAP, BORDER_CONSTANT, Scalar::default());
        if let Err(code) = result_affine {
            panic!("{}", code)
        }

        let filename = angle.to_string() + "_rotated.png";
        let filename: &str = &filename;
        imwrite(filename, &dst_img, &Vector::new()).ok();

        let mut ocr = OCRTesseract::create("", "jpn", "", OEM_DEFAULT, PSM_AUTO)
            .unwrap_or_else(|code| panic!("{}", code));
        let str = ocr.run(&dst_img, 0, 0).unwrap();

        println!("回転角度: {}° 文字数: {} 認識した文字 =>「{}」", angle, str.chars().count(), str);

        if str.chars().count() > max_chars_count {
            max_chars_count = str.chars().count();
            angle_at_max = angle;
        }
        angle += 90.0;
    }

    println!("認識できた文字数が最も多かった角度: {}, 認識した文字数: {}", angle_at_max, max_chars_count);
}
