extern crate image;

use image::{ GenericImage };
use std::env;
use std::path::Path;

fn main() {
   
    println!("\n\n--------------- START ---------------");
    
    // Capture the command line arguments.
    let args: Vec<String> = env::args().collect();
    // println!("Parameters supplied: {:?}", args);
    let phrase = &args[1];
    let split = phrase.split("*");
    let parts: Vec<&str> = split.collect();
    let key = parts[0];
    // println!("Password: {}", key);
    let msg_length = parts[3].to_string().parse::<u8>().unwrap();
    println!("Message length: {} characters", msg_length);
    let start_x = parts[1].to_string().parse::<u32>().unwrap();
    println!("Start X coordinate: {}", start_x);
    let start_y = parts[2].to_string().parse::<u32>().unwrap();
    println!("Start Y coordinate: {}", start_y);
    let offsets: Vec<_> = key.chars().into_iter().map(|x| x as u8).collect();
    // println!("Coordinate offsets: {:?}", offsets);
    let source_file_name = &args[2];
    let source_image = image::open(&Path::new(source_file_name)).unwrap();
    
    // Decode the message from the image pixel RGBA channels.
    let mut cycle :u8 = 0;
    let mut x_location :u8 = 0;
    let offset_length = offsets.len() as u8;
    let offset_max_index :u8 = offset_length - 1;
    let mut reset :f32 = 0.0;
    let mut reset_u8 :u8 = 0;
    let mut reset_multiplier :u8 = 0;
    let mut retrieved_characters = Vec::new();
    let mut item :u8 = 0;
    for i in 0..msg_length as u8 {
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
        let pixel = source_image.get_pixel(x_location as u32 + start_x, i as u32 + start_y);
        if cycle > 2 {
            cycle = 0;
        }
        item = pixel[cycle as usize];
        retrieved_characters.push(item);
        cycle += 1;
    }
    // println!("Character codes retrieved: {:?}", retrieved_characters);
    
    let string_characters: Vec<_> = retrieved_characters.into_iter().map(|x| x as char).map(|x| x.to_string()).collect();
    // println!("Text characters retrieved: {:?}", string_characters);
    
    // Display the decoded message.
    let message = string_characters.join("");
    println!("Decoded message:\n\n{}", message);
    
    println!("\n---------------- END ----------------");
}