extern crate image;
extern crate rand;

use rand::Rng;
use std::env;
use std::fs::File;
use std::num;
use std::io::prelude::*;
use std::path::Path;

fn main() {
   
    println!("\n\n--------------- START ---------------");
    
    // Capture the command line arguments.
    let args: Vec<String> = env::args().collect();
    // println!("Parameters supplied: {:?}", args);
    let width = args[1].parse::<u32>().unwrap();
    let height = args[2].parse::<u32>().unwrap();
    let key = &args[3];
    let text_file_name = &args[4];
    let save_to_file_name = &args[5];
    
    // Generate an image filled with random RGBA pixel channel values.
    let mut generated_image = image::ImageBuffer::new(width, height);
    let mut r :u8 = 0;
    let mut g :u8 = 0;
    let mut b :u8 = 0;
    let mut a :u8 = 0;
    for (x, y, pixel) in generated_image.enumerate_pixels_mut() {
        r = rand::thread_rng().gen_range(0, 255);
        g = rand::thread_rng().gen_range(0, 255);
        b = rand::thread_rng().gen_range(0, 255);
        a = rand::thread_rng().gen_range(0, 255);
        *pixel = image::Rgba([r, g, b, a]);
        // *pixel = image::Rgba([255u8, 255u8, 255u8, 255u8]);
    }
    
    // Capture the text file contents.
    let mut text_file = File::open(text_file_name).expect("File not found!");
    let mut contents = String::new();
    text_file.read_to_string(&mut contents).expect("Could not read the file! Make sure to supply an ordinary .txt file.");
    println!("Message to encode:\n\n{}\n", contents);
    
    let message_length = contents.chars().count();
    println!("Message length: {} characters", message_length);
    
    let offsets: Vec<_> = key.chars().into_iter().map(|x| x as u8).collect();
    // println!("Key numerical: {:?}", offsets);

    let msg: Vec<_> = contents.chars().into_iter().map(|x| x as u8).collect();
    // println!("Message numerical: {:?}", msg);
    let msg_length = msg.len() as u32;

    // Encode the message into the RGBA channels.
    let mut cycle :u8 = 0;
    let mut x_location :u8 = 0;
    let noise_start_x :u32 = rand::thread_rng().gen_range(0, (width - msg_length) / 2);
    let noise_start_y :u32 = rand::thread_rng().gen_range(0, (height - msg_length) / 2);
    let offset_length = offsets.len() as u8;
    let offset_max_index :u8 = offset_length - 1;
    let mut reset :f32 = 0.0;
    let mut reset_u8 :u8 = 0;
    let mut reset_multiplier :u8 = 0;
    for (i, item) in msg.iter().enumerate() {
        
        let map_index = i as u8;
        if map_index > offset_max_index {
            reset = (map_index as f32 / offset_length as f32).floor();
            reset_u8 = reset as u8;
            reset_multiplier = offset_length * reset_u8;
            if reset_u8 > 0 {
                x_location = map_index - reset_multiplier;
            } else {
                x_location = map_index - offset_length; 
            }
        } else {
            x_location = map_index;
        }
        x_location = offsets[x_location as u32 as usize];
        
        let mut pixel = generated_image[(x_location as u32 + noise_start_x, i as u32 + noise_start_y)];
        r = rand::thread_rng().gen_range(0, 255);
        g = rand::thread_rng().gen_range(0, 255);
        b = rand::thread_rng().gen_range(0, 255);
        a = rand::thread_rng().gen_range(0, 255);
        if cycle > 2 {
            cycle = 0;
        }
        if cycle == 0 {
            pixel = image::Rgba([*item as u8, g, b, a]);
        }
        if cycle == 1 {
            pixel = image::Rgba([r, *item as u8, b, a]);
        }
        if cycle == 2 {
            pixel = image::Rgba([r, g, *item as u8, a]);
        }
        generated_image.put_pixel(x_location as u32 + noise_start_x, i as u32 + noise_start_y, pixel);
        cycle += 1;
    }
    
    // Save the image to a new PNG file.
    let ref mut fout = File::create(&Path::new(save_to_file_name)).unwrap();
    let _ = image::ImageRgba8(generated_image).save(fout, image::PNG);
    println!("Image saved as: {}", save_to_file_name);
    let decode_password = format!("{}*{}*{}*{}", key, noise_start_x, noise_start_y, msg_length);
    println!("To decode: {}", decode_password);
	
	println!("---------------- END ----------------");
}
