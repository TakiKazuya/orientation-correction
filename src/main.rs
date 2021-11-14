use std::env;
use mecab::Tagger;
use opencv::core::{MatTraitManual,
                   Vector,
                   rotate,
                   ROTATE_180, ROTATE_90_CLOCKWISE, ROTATE_90_COUNTERCLOCKWISE,};
const NO_ROTATE: i32 = -1;

use opencv::imgcodecs::{imread, imwrite, IMREAD_GRAYSCALE};
use opencv::text::{OEM_DEFAULT, PSM_AUTO};
use opencv::text::prelude::OCRTesseract;

const SOURCE_IMAGE_PATH: &str = "src_img.png";

fn main() {
    // 処理元の画像を定義
    let src_img = imread(SOURCE_IMAGE_PATH, IMREAD_GRAYSCALE).unwrap();
    if src_img.data().is_err() {
        panic!("Failed to read image.")
    };

    let rotate_code_list = vec![NO_ROTATE, ROTATE_90_CLOCKWISE, ROTATE_180, ROTATE_90_COUNTERCLOCKWISE];
    let mut dst_rotate_code = NO_ROTATE;
    let mut max_words_count = 0;

    for rotate_code in rotate_code_list {
        // 画像の回転
        let mut dst_img = src_img.clone();
        rotate(&src_img, &mut dst_img, rotate_code).unwrap_or_else(|error| panic!("{}", error));
        let angle_int = rotate_code_to_angle_int(rotate_code);

        // 回転した画像の描画
        let filename = angle_int.to_string() + "_rotated.png";
        let filename: &str = &filename;
        imwrite(filename, &dst_img, &Vector::new()).ok();

        // OCR
        let mut ocr = OCRTesseract::create("", "jpn", "", OEM_DEFAULT, PSM_AUTO)
            .unwrap_or_else(|code| panic!("{}", code));
        let ocr_str = ocr.run(&dst_img, 0, 0).unwrap();

        // mecabを使った形態素解析
        let mecab_env = env::var("EXTEND_MECAB_DICTIONARY");
        let mecab_arg = match mecab_env {
            Ok(path) => "-d ".to_string() + &path,
            Err(err) => {
                panic!("mecabの拡張辞書の取得に失敗しました。デフォルトの辞書を使用します。 Message: {}", err)
            }
        };
        let mecab = Tagger::new(mecab_arg);
        let parse_result = mecab.parse_str(ocr_str.clone());
        let mut result_lines: Vec<&str> = parse_result.trim().split("\n").collect();
        result_lines.remove(result_lines.len() - 1);

        let recognizable_words = result_lines.iter().filter(|line| {
            let regex = regex::Regex::new(r"\t|,").unwrap();
            let word_info: Vec<&str> = regex.split(line).collect();
            word_info[0].chars().count() >= 2 // 2文字以上であること
        });

        println!("回転角度: {}° 認識した文字数: {:?} 認識した文字 =>「{}」", angle_int, recognizable_words.clone().count(), ocr_str);
        if recognizable_words.clone().count() > max_words_count {
            max_words_count = recognizable_words.count();
            dst_rotate_code = rotate_code;
        }
    }

    let angle_at_max = rotate_code_to_angle_int(dst_rotate_code);
    println!("認識できた文字数が最も多かった角度: {}°, 認識した単語数: {}", angle_at_max, max_words_count);

    // 認識できた文字数が1番多かった角度へ回転させる
    let result_img = if dst_rotate_code != NO_ROTATE {
        println!("{}°回転します。", angle_at_max);
        let mut dst_img = src_img.clone();
        rotate(&src_img, &mut dst_img, dst_rotate_code).unwrap_or_else(|error| panic!("{}", error));
        dst_img
    } else {
        println!("回転の必要がありません。");
        src_img
    };

    imwrite("result.png", &result_img, &Vector::new()).ok();
}

fn rotate_code_to_angle_int(code: i32) -> i32 {
    match code {
        NO_ROTATE => 0,
        ROTATE_90_CLOCKWISE => 90,
        ROTATE_180 => 180,
        ROTATE_90_COUNTERCLOCKWISE => 270,
        _ => panic!(),
    }
}
