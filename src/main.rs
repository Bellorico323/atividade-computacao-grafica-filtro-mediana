use std::fs::{self, File};
use std::io::{BufRead, BufReader};

// Função para ler uma imagem PGM
fn read_pgm(filename: &str) -> (u32, u32, Vec<u8>) {
    // Abre o arquivo PGM e cria um buffer para leitura
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut width = 0;
    let mut height = 0;
    let mut pixels = Vec::new();
    let mut reading_pixels = false;

    // Lê as linhas do arquivo
    for line in reader.lines() {
        let line = line.expect("Failed to read line").trim().to_string();
        
        // Ignora comentários
        if line.starts_with('#') {
            continue;
        }
        
        // Processa o cabeçalho do arquivo PGM
        if !reading_pixels {
            if line == "P2" {
                continue; // Confirma que o formato é P2
            } else if width == 0 && height == 0 {
                // Extrai largura e altura da imagem
                let dims: Vec<_> = line.split_whitespace().collect();
                width = dims[0].parse().expect("Invalid width");
                height = dims[1].parse().expect("Invalid height");
            } else if line.parse::<u32>().is_ok() {
                // Se a linha contém um número, inicia a leitura dos pixels
                reading_pixels = true;
            }
            continue;
        }

        // Lê os valores dos pixels
        for value in line.split_whitespace() {
            pixels.push(value.parse().expect("Invalid pixel value"));
        }
    }

    (width, height, pixels) // Retorna a largura, altura e os pixels da imagem
}

// Função para aplicar o filtro de mediana
fn apply_median_filter(width: u32, height: u32, pixels: &Vec<u8>) -> Vec<u8> {
    let mut output = pixels.clone(); // Cria uma cópia dos pixels de entrada
    
    // Função para obter os vizinhos de um pixel
    let neighbors = |x: i32, y: i32| -> Vec<u8> {
        let mut values = Vec::new(); // Vetor para armazenar os valores dos pixels vizinhos
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x + dx; // Coordenada x do vizinho
                let ny = y + dy; // Coordenada y do vizinho
                // Verifica se o vizinho está dentro dos limites da imagem
                if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                    let idx = (ny as u32 * width + nx as u32) as usize; // Índice do pixel no vetor
                    values.push(pixels[idx]); // Adiciona o pixel vizinho ao vetor
                }
            }
        }
        values
    };

    // Percorre todos os pixels da imagem
    for y in 0..height {
        for x in 0..width {
            let values = neighbors(x as i32, y as i32); // Obtém os vizinhos do pixel atual
            let mut sorted_values = values.clone(); // Clona os valores dos vizinhos
            sorted_values.sort(); // Ordena os valores
            let median = sorted_values[sorted_values.len() / 2];
            let idx = (y * width + x) as usize; // Índice do pixel no vetor de saída
            output[idx] = median; // Substitui o valor do pixel pela mediana
        }
    }

    output
}

use std::fs::write;

// Função para escrever uma imagem PGM em um arquivo
fn write_pgm(filename: &str, width: u32, height: u32, pixels: &Vec<u8>) {
    let mut content = String::new();
    content.push_str("P2\n"); // Escreve o cabeçalho do formato
    content.push_str(&format!("{} {}\n255\n", width, height)); // Escreve largura, altura e valor máximo

    // Adiciona os valores dos pixels ao conteúdo
    for (i, pixel) in pixels.iter().enumerate() {
        content.push_str(&format!("{} ", pixel));
        if (i + 1) % width as usize == 0 {
            content.push('\n'); // Nova linha após cada linha de pixels
        }
    }

    // Escreve o conteúdo em um arquivo
    write(filename, content).expect("Failed to write file");
}

// Função para processar todos os arquivos em um diretório
fn process_files_in_directory(directory: &str, output_directory: &str) {
    fs::create_dir_all(output_directory).expect("Failed to create output directory"); // Cria o diretório de saída

    // Lê todos os arquivos do diretório
    for entry in fs::read_dir(directory).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        // Verifica se a entrada é um arquivo PGM
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("pgm") {
            let filename = path.to_str().unwrap(); // Obtém o nome do arquivo
            let (width, height, pixels) = read_pgm(filename); // Lê a imagem PGM
            let filtered_pixels = apply_median_filter(width, height, &pixels); // Aplica o filtro de mediana

            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let output_path = format!("{}/{}_modified.pgm", output_directory, file_stem);

            write_pgm(&output_path, width, height, &filtered_pixels);

            println!("Filtro de mediana aplicado e salvo em {}", output_path);
        }
    }
}

fn main() {
    let input_dir = "assets"; // Diretório de entrada
    let output_dir = "./src/filtered_images"; // Diretório de saída

    process_files_in_directory(input_dir, output_dir);
}
