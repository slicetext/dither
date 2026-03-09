use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, ImageResult, Pixel, Rgb};

const BLACK_THRESHOLD: u8 = 128;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let img = ImageReader::open(&args[1]).unwrap().decode().unwrap();
    let mut dyn_img = DynamicImage::ImageRgb8(img.to_rgb8());
    dyn_img = dyn_img.rotate90();
    let width = dyn_img.width();
    let height = dyn_img.height();

    let mut error: Vec<Vec<i16>> = vec![vec![0; dyn_img.width() as usize]; dyn_img.height() as usize];
    let mut pixel_index = 0;
    // let mut threshold = 0;
    for j in 0..height {
        for i in 0..width {
            let old_pxl = dyn_img.get_pixel(i, j).channels().to_owned();
            let new_color = ((old_pxl[0] as usize + old_pxl[1] as usize + old_pxl[2] as usize) / 3) as u8;
            // let new_color = (((old_pxl[0] as f32 * 0.2126) + (old_pxl[1] as f32 * 0.7152) + (old_pxl[2] as f32 * 0.0722)) / 3.0) as u8;

            // let threshold_width: u8 = 5;
            // let threshold_sample_rate = 2;
            // let mut total_threshold: usize = 0;
            // let mut threshold_samples: u8 = 0;
            // if pixel_index % (threshold_width * threshold_sample_rate) == 0 {
            //     pixel_index = 0;
            //     for x in 0..threshold_width {
            //         for y in 0..threshold_width {
            //             if check_arr_indexes(x as i32 + i as i32, y as i32 + j as i32, width, height) {
            //                 let cur_pxl = dyn_img.get_pixel(x as u32 + i, y as u32 + j).channels().to_owned();
            //                 total_threshold += ((cur_pxl[0] as usize + cur_pxl[1] as usize + cur_pxl[2] as usize) / 3) as usize;
            //                 threshold_samples += 1;
            //             }
            //         }
            //     }
            //     threshold = total_threshold / threshold_samples as usize;
            // }
            // 
            let quantized: u8;
            // if (new_color as i16 + error[j as usize][i as usize]) as u8 > threshold as u8 {
            if (new_color as i16 + error[j as usize][i as usize]) as u8 > BLACK_THRESHOLD {
                quantized = 255;
            } else {
                quantized = 0
            }
            // Calc error
            let error_amt = new_color as i16 - quantized as i16;
            // Distribute error
            // Atkinson dithering kernel:
            // 1/k * [   * 1 1 ]
            //       [ 1 1 1   ]
            //       [   1     ]
            // k = 8
            let k = 8.0;
            if check_arr_indexes((i + 1) as i32, j as i32, width, height) {
                error[j as usize][i as usize + 1] += (error_amt as f32 * (1.0 / k)) as i16;
            }
            if check_arr_indexes((i + 2) as i32, j as i32, width, height) {
                error[j as usize][i as usize + 2] += (error_amt as f32 * (1.0 / k)) as i16;
            }
            if check_arr_indexes(i as i32 - 1, (j + 1) as i32, width, height) {
                error[j as usize + 1][i as usize - 1] += (error_amt as f32 * (1.0 / k)) as i16;
            }
            if check_arr_indexes(i as i32, (j + 1) as i32, width, height) {
                error[j as usize + 1][i as usize] += (error_amt as f32 * (1.0 / k)) as i16;
            }
            if check_arr_indexes(i as i32 + 1, (j + 1) as i32, width, height) {
                error[j as usize + 1][i as usize + 1] += (error_amt as f32 * (1.0 / k)) as i16;
            }
            if check_arr_indexes(i as i32, (j + 2) as i32, width, height) {
                error[j as usize + 2][i as usize] += (error_amt as f32 * (1.0 / k)) as i16;
            }

            dyn_img.put_pixel(i, j, Pixel::from_channels(quantized, quantized, quantized, 255));

            pixel_index += 1;
        }
    }
    println!("Processing completed");
    let _ = dyn_img.save("result.jpeg");
    println!("Image saved")
}

fn check_arr_indexes(x: i32, y: i32, w: u32, h: u32) -> bool {
    return x >= 0 && x < w as i32 && y >= 0 && y < h as i32;
}
