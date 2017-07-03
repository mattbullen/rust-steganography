extern crate image;
extern crate num;

use image::{ GenericImage };
use num::Integer;
use std::env;
use std::path::Path;

fn main() {
   
    println!("\n\n------------------------------------------------------------");
    
    // Capture the command line arguments.
    let args: Vec<String> = env::args().collect();
    // println!("Parameters supplied: {:?}", args);
    let phrase = &args[1];
    let split = phrase.split("*");
    let parts: Vec<&str> = split.collect();
    let key = parts[0];
    println!("Password: {}", key);
	let width = parts[1].to_string().parse::<u32>().unwrap();
	let height = parts[2].to_string().parse::<u32>().unwrap();
    let msg_length = parts[5].to_string().parse::<u8>().unwrap();
    println!("Message length: {} characters", msg_length);
	let start_offset = parts[6].to_string().parse::<u32>().unwrap();
    println!("Sequence offset: {}", start_offset);
	let offset_max = parts[7].to_string().parse::<u32>().unwrap();
    println!("Sequence max: {}", offset_max);
    let start_x = parts[3].to_string().parse::<u32>().unwrap();
    println!("Start X coordinate: {}", start_x);
    let start_y = parts[4].to_string().parse::<u32>().unwrap();
    println!("Start Y coordinate: {}", start_y);
    let offsets: Vec<_> = key.chars().into_iter().map(|x| x as u8).collect();
    println!("Coordinate offsets (u8): {:?}", offsets);
    let source_file_name = &args[2];
    let source_image = image::open(&Path::new(source_file_name)).unwrap();
	// let target = &args[3];
    // println!("Decoding: {}", target);
	
	let mut bin_list = Vec::new();
	let mut source_pixels_counter :u32 = 0;
	let mut source_cycle :u8 = 0;
	let mut c_val :i32 = 0;
	for (x, y, pixel) in source_image.pixels() {
		if source_pixels_counter >= start_offset && source_pixels_counter < offset_max {
			// println!("{:?} {} {}", pixel, source_pixels_counter, source_pixels_counter - start_offset);
			c_val = pixel[source_cycle as usize] as i32;
			if c_val.is_even() {
				bin_list.push("0");
			} else {
				bin_list.push("1");
			}
			source_cycle += 1;
			if source_cycle > 3 {
				source_cycle = 0;
			}
		}
		source_pixels_counter += 1;
    }
	// println!("Retrieved binary digits: {:?}", bin_list);

	let mut bin_assembled = Vec::new();
	let mut bin_counter :u32 = 0;
	for i in 0..bin_list.len() {
		if bin_counter == 0 {
			let mut bin_temp = Vec::new();
			bin_temp.push(bin_list[i]);
			bin_temp.push(bin_list[i + 1]);
			bin_temp.push(bin_list[i + 2]);
			bin_temp.push(bin_list[i + 3]);
			bin_temp.push(bin_list[i + 4]);
			bin_temp.push(bin_list[i + 5]);
			bin_temp.push(bin_list[i + 6]);
			bin_temp.push(bin_list[i + 7]);
			let bin_string = bin_temp.join("");
			bin_assembled.push(bin_string);
		}
		bin_counter += 1;
		if bin_counter > 7 {
			bin_counter = 0;
		}
	}
	// println!("Retrieved binary pixels: {:?}", bin_assembled);
	
	let pixel_vals: Vec<_> = bin_assembled.into_iter().map(|x| u32::from_str_radix(&x, 2).unwrap()).collect();
	// println!("Pixel channel values: {:?}", pixel_vals);
	println!("Number of pixel channels: {}", pixel_vals.len());
	
	let mut val_cycle :u8 = 0;
	let mut final_pixels_list = Vec::new();
	let mut new_pixel = image::Rgba([0u8, 0u8, 0u8, 0u8]);
	for i in 0..pixel_vals.len() {
		if val_cycle == 0 {
			new_pixel = image::Rgba([pixel_vals[i] as u8, pixel_vals[i + 1] as u8, pixel_vals[i + 2] as u8, pixel_vals[i + 3] as u8]);
			final_pixels_list.push(new_pixel);
		}
		val_cycle += 1;
		if val_cycle > 3 {
            val_cycle = 0;
        }
	}
	// println!("Reassembled pixels: {:?}", final_pixels_list);
	println!("Number of reassembled pixels: {}", final_pixels_list.len());
	
	let mut decode_image = image::ImageBuffer::new(width, height);
	let mut pixel_counter :u32 = 0;
	for (x, y, pixel) in decode_image.enumerate_pixels_mut() {
		*pixel = final_pixels_list[pixel_counter as usize];
		pixel_counter += 1;
	}
	
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
		
		/*
		if target == "raw" {
			let pixel = raw_source_image.get_pixel(x_location as u32 + start_x, i as u32 + start_y);
			item = pixel[cycle as usize] as u8;
			retrieved_characters.push(item);
		} else {
			let pixel = decode_image.get_pixel(x_location as u32 + start_x, i as u32 + start_y);
			item = pixel[cycle as usize] as u8;
			retrieved_characters.push(item);
		}
		*/
		
		let pixel = decode_image.get_pixel(x_location as u32 + start_x, i as u32 + start_y);
		item = pixel[cycle as usize] as u8;
		retrieved_characters.push(item);
		
		cycle += 1;
        if cycle > 3 {
            cycle = 0;
        }
    }
    // println!("Character codes retrieved: {:?}", retrieved_characters);
    
    let string_characters: Vec<_> = retrieved_characters.into_iter().map(|x| x as char).map(|x| x.to_string()).collect();
    // println!("Text characters retrieved: {:?}", string_characters);
    
    // Display the decoded message.
    let message = string_characters.join("");
    println!("Decoded message:\n\n{}", message);

    println!("\n------------------------------------------------------------");
}