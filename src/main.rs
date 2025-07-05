use gif::{DecodeOptions, Encoder, Frame, Repeat};
use std::{
    env,
    fs::{File, read_to_string},
    io::{BufReader, BufWriter},
    path::Path,
};

#[derive(Clone, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color { r, g, b })
        } else {
            None
        }
    }

    fn distance_squared(&self, other: &Color) -> u32 {
        let dr = self.r as i32 - other.r as i32;
        let dg = self.g as i32 - other.g as i32;
        let db = self.b as i32 - other.b as i32;
        (dr * dr + dg * dg + db * db) as u32
    }
}

fn nearest_color_index(color: &Color, palette: &[Color]) -> usize {
    palette
        .iter()
        .enumerate()
        .min_by_key(|(_, c)| c.distance_squared(color))
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn load_palette_from_txt(path: &str) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
    let mut palette = Vec::new();
    let content = read_to_string(path)?;

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match Color::from_hex(line) {
            Some(color) => palette.push(color),
            None => {
                eprintln!(
                    "Warning: Invalid hex color '{}' on line {}",
                    line,
                    line_num + 1
                );
            }
        }
    }

    if palette.is_empty() {
        return Err("No valid colors found in palette file".into());
    }

    Ok(palette)
}

fn load_palette_from_args(args: &[String]) -> Result<Vec<Color>, Box<dyn std::error::Error>> {
    let mut palette = Vec::new();

    for arg in args {
        match Color::from_hex(arg) {
            Some(color) => palette.push(color),
            None => {
                eprintln!("Warning: Invalid hex color '{}'", arg);
            }
        }
    }

    if palette.is_empty() {
        return Err("No valid colors provided".into());
    }

    Ok(palette)
}

fn create_mapped_palette(original_palette: &[u8], target_palette: &[Color]) -> Vec<u8> {
    let mut mapped = Vec::new();

    // Map each color in the original palette to the nearest target color
    for rgb in original_palette.chunks(3) {
        if rgb.len() == 3 {
            let original_color = Color {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            };
            let nearest_idx = nearest_color_index(&original_color, target_palette);
            let nearest = &target_palette[nearest_idx];
            mapped.extend_from_slice(&[nearest.r, nearest.g, nearest.b]);
        }
    }

    mapped
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <input.gif> <output.gif> <palette.txt|#hex1> [#hex2] ...",
            args[0]
        );
        eprintln!("Examples:");
        eprintln!("  {} input.gif output.gif colors.txt", args[0]);
        eprintln!("  {} input.gif output.gif #ff0000 #00ff00 #0000ff", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Load target palette
    let target_palette = if Path::new(&args[3]).exists() {
        load_palette_from_txt(&args[3])?
    } else {
        load_palette_from_args(&args[3..])?
    };

    println!("Loaded {} colors in target palette", target_palette.len());
    for (i, color) in target_palette.iter().enumerate() {
        println!(
            "  {}: #{:02x}{:02x}{:02x}",
            i + 1,
            color.r,
            color.g,
            color.b
        );
    }

    let file_in = File::open(input_path)?;
    let mut decoder = DecodeOptions::new().read_info(BufReader::new(file_in))?;

    let original_global_palette = decoder.global_palette().unwrap_or(&[]).to_vec();

    let mapped_global_palette = if !original_global_palette.is_empty() {
        create_mapped_palette(&original_global_palette, &target_palette)
    } else {
        target_palette
            .iter()
            .flat_map(|c| [c.r, c.g, c.b])
            .collect()
    };

    let file_out = File::create(output_path)?;
    let mut encoder = Encoder::new(
        BufWriter::new(file_out),
        decoder.width(),
        decoder.height(),
        &mapped_global_palette,
    )?;
    encoder.set_repeat(Repeat::Infinite)?;

    let mut frame_count = 0;

    while let Some(frame) = decoder.read_next_frame()? {
        frame_count += 1;

        let mut new_frame = Frame {
            delay: frame.delay,
            dispose: frame.dispose,
            transparent: frame.transparent,
            needs_user_input: frame.needs_user_input,
            top: frame.top,
            left: frame.left,
            width: frame.width,
            height: frame.height,
            interlaced: frame.interlaced,
            palette: None, // Use global palette
            buffer: frame.buffer.clone(),
        };

        if let Some(local_palette) = &frame.palette {
            let mapped_local = create_mapped_palette(local_palette, &target_palette);
            new_frame.palette = Some(mapped_local);
        }

        encoder.write_frame(&new_frame)?;

        if frame_count % 10 == 0 {
            println!("Processed {} frames...", frame_count);
        }
    }

    println!("✅ Successfully processed {} frames", frame_count);
    println!("✅ Saved recolored GIF to {}", output_path);

    Ok(())
}
