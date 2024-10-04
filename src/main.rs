use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn read_pgm(filename: &str) -> (u32, u32, Vec<u8>) {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut width = 0;
    let mut height = 0;
    let mut pixels = Vec::new();
    let mut reading_pixels = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line").trim().to_string();
        
        if line.starts_with('#') {
            continue;
        }
        
        if !reading_pixels {
            if line == "P2" {
                continue; // Confirma que o formato é P2
            } else if width == 0 && height == 0 {
                let dims: Vec<_> = line.split_whitespace().collect();
                width = dims[0].parse().expect("Invalid width");
                height = dims[1].parse().expect("Invalid height");
            } else if line.parse::<u32>().is_ok() {
                reading_pixels = true;
            }
            continue;
        }

        for value in line.split_whitespace() {
            pixels.push(value.parse().expect("Invalid pixel value"));
        }
    }

    (width, height, pixels)
}

fn apply_median_filter(width: u32, height: u32, pixels: &Vec<u8>) -> Vec<u8> {
    let mut output = pixels.clone();
    
    let neighbors = |x: i32, y: i32| -> Vec<u8> {
        let mut values = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let idx = (ny as u32 * width + nx as u32) as usize;
                    values.push(pixels[idx]);
                }
            }
        }
        values
    };

    for y in 0..height {
        for x in 0..width {
            let values = neighbors(x as i32, y as i32);
            let mut sorted_values = values.clone();
            sorted_values.sort();
            let median = sorted_values[sorted_values.len() / 2];
            let idx = (y * width + x) as usize;
            output[idx] = median;
        }
    }

    output
}

use std::fs::write;

fn write_pgm(filename: &str, width: u32, height: u32, pixels: &Vec<u8>) {
    let mut content = String::new();
    content.push_str("P2\n");
    content.push_str(&format!("{} {}\n255\n", width, height));

    for (i, pixel) in pixels.iter().enumerate() {
        content.push_str(&format!("{} ", pixel));
        if (i + 1) % width as usize == 0 {
            content.push('\n');
        }
    }

    write(filename, content).expect("Failed to write file");
}

fn process_files_in_directory(directory: &str, output_directory: &str) {
    fs::create_dir_all(output_directory).expect("Failed to create output directory");

    for entry in fs::read_dir(directory).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("pgm") {
            let filename = path.to_str().unwrap();
            let (width, height, pixels) = read_pgm(filename);
            let filtered_pixels = apply_median_filter(width, height, &pixels);

            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let output_path = format!("{}/{}_modified.pgm", output_directory, file_stem);

            write_pgm(&output_path, width, height, &filtered_pixels);

            println!("Filtro de mediana aplicado e salvo em {}", output_path);
        }
    }
}

fn main() {
    let input_dir = "assets";
    let output_dir = "./src/filtered_images";

    process_files_in_directory(input_dir, output_dir);
}
