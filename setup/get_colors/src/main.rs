use image::{GenericImageView};
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide an image path as an argument");
        return;
    }

    let image_path = &args[1];

    let img = image::open(image_path).expect("Error opening image");
    let width = img.width();
    let height = img.height();

    let mut color_counts: HashMap<String, usize> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let color = pixel.0;
            let color_hex = format!("{:02X}{:02X}{:02X}", color[0], color[1], color[2]);

            if let Some(count) = color_counts.get_mut(&color_hex) {
                *count += 1;
            } else {
                color_counts.insert(color_hex.clone(), 1);
            }
        }
    }

    let mut most_common_colors: Vec<String> = Vec::new();

    for (color_hex, _count) in color_counts {
        let mut should_add_color = true;

        for common_color in &most_common_colors {
            if color_distance(&color_hex, &common_color) < 50.0 {
                should_add_color = false;
                break;
            }
        }

        if should_add_color {
            most_common_colors.push(color_hex);
        }
    }

    most_common_colors.sort_by(|a, b| {
        let brightness_a = calculate_brightness(a);
        let brightness_b = calculate_brightness(b);
        brightness_a.partial_cmp(&brightness_b).unwrap()
    });

    for color_hex in most_common_colors {
        println!("#{}", color_hex);
    }
}

fn color_distance(color1: &str, color2: &str) -> f32 {
    let r1 = u32::from_str_radix(&color1[0..2], 16).unwrap();
    let g1 = u32::from_str_radix(&color1[2..4], 16).unwrap();
    let b1 = u32::from_str_radix(&color1[4..6], 16).unwrap();

    let r2 = u32::from_str_radix(&color2[0..2], 16).unwrap();
    let g2 = u32::from_str_radix(&color2[2..4], 16).unwrap();
    let b2 = u32::from_str_radix(&color2[4..6], 16).unwrap();

    let distance = ((r1 as i32 - r2 as i32).pow(2) +
        (g1 as i32 - g2 as i32).pow(2) +
        (b1 as i32 - b2 as i32).pow(2)) as f32;

    distance.sqrt()
}

fn calculate_brightness(color: &str) -> f32 {
    let r = u32::from_str_radix(&color[0..2], 16).unwrap() as f32;
    let g = u32::from_str_radix(&color[2..4], 16).unwrap() as f32;
    let b = u32::from_str_radix(&color[4..6], 16).unwrap() as f32;

    // Calculate brightness using the formula (0.
    0.299 * r + 0.587 * g + 0.114 * b
}
