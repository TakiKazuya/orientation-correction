use opencv::core::{BORDER_CONSTANT, MatTrait, MatTraitManual, Point2f, Scalar, Size, Vector};
use opencv::imgcodecs::{imread, IMREAD_GRAYSCALE, imwrite};
use opencv::imgproc::{get_rotation_matrix_2d, warp_affine, WARP_INVERSE_MAP};

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

    let angles: Vec<f64> = vec![0.0, 90.0, 180.0, 270.0];

    for angle in angles {
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
    }
}
