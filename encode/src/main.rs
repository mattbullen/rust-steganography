extern crate image;
extern crate num;
extern crate rand;

use image::{ GenericImage };
use num::Integer;
use rand::Rng;
use std::env;
use std::fs::File;
use std::iter::once;
use std::io::prelude::*;
use std::path::Path;

fn main() {
   
    println!("\n\n------------------------------------------------------------");
    
    // Capture the command line arguments.
    let args: Vec<String> = env::args().collect();
    // println!("Parameters supplied: {:?}", args);
	let key = &args[3];
	println!("Password: {}", key);
    let width = args[1].parse::<u32>().unwrap();
	println!("Encoding image width: {}px", width);
    let height = args[2].parse::<u32>().unwrap();
    println!("Encoding image height: {}px", height);
    let text_file_name = &args[4];
	println!("Message source: {}", text_file_name);
	let mask_file = &args[5];
	println!("Destination image: {}", mask_file);
    let toggle_save_raw = &args[6];
	
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
    let raw_start_x :u32 = rand::thread_rng().gen_range(0, (width - msg_length) / 2);
    let raw_start_y :u32 = rand::thread_rng().gen_range(0, (height - msg_length) / 2);
    let offset_length = offsets.len() as u8;
    let offset_max_index :u8 = offset_length - 1;
    let mut reset :f32 = 0.0;
    let mut reset_u8 :u8 = 0;
    let mut reset_multiplier :u8 = 0;
	let mut pixel;
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
        
        pixel = generated_image[(x_location as u32 + raw_start_x, i as u32 + raw_start_y)];
        r = rand::thread_rng().gen_range(0, 255);
        g = rand::thread_rng().gen_range(0, 255);
        b = rand::thread_rng().gen_range(0, 255);
        a = rand::thread_rng().gen_range(0, 255);
        if cycle > 3 {
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
		if cycle == 3 {
            pixel = image::Rgba([r, g, b, *item as u8]);
        }
        generated_image.put_pixel(x_location as u32 + raw_start_x, i as u32 + raw_start_y, pixel);
        cycle += 1;
    }
	
	// Masking println!("y: {}", y); 
	
	let mut channel_values_list = Vec::new();
	for (x, y, pixel) in generated_image.enumerate_pixels_mut() {
		channel_values_list.push(pixel[0]);
		channel_values_list.push(pixel[1]);
		channel_values_list.push(pixel[2]);
		channel_values_list.push(pixel[3]);
    }
	// println!("Channel values list: {:?}", channel_values_list); 
	
	let bin_values: Vec<_> = channel_values_list.into_iter().map(|x| format!("{:b}", x)).map(|x| x.to_string()).collect();
	// println!("Binary values list: {:?}", bin_values);
	
	let mut padded_bin = Vec::new();
	for i in 0..bin_values.len() {
		let ref bin = bin_values[i];
		let padded = format!("00000000{}", bin);
		let iter_bin: Vec<char> = padded.chars().rev().take(8).collect();
		
		//println!("{} {:?}", padded, iter_bin);
		
		padded_bin.push(iter_bin[7]);
		padded_bin.push(iter_bin[6]);
		padded_bin.push(iter_bin[5]);
		padded_bin.push(iter_bin[4]);
		padded_bin.push(iter_bin[3]);
		padded_bin.push(iter_bin[2]);
		padded_bin.push(iter_bin[1]);
		padded_bin.push(iter_bin[0]);
	}
	// println!("{:?} {}", padded_bin, padded_bin.len());
	
	let mask_image = image::open(&Path::new(mask_file)).unwrap();
	let channel_values_length = padded_bin.len();
	let channel_values_length_u32 = channel_values_length as u32;
	let (mask_width, mask_height) = mask_image.dimensions();
	let sequence_max = (mask_height * mask_width) - channel_values_length_u32;
	
	let sequence_offset = rand::thread_rng().gen_range(0, sequence_max);
	println!("Sequence offset: {}", sequence_offset);
	let sequence_offset_max = sequence_offset + channel_values_length_u32;
	println!("Sequence offset max: {}", sequence_offset_max);
	
	let mut final_values_list = Vec::new();
	let mut mask_counter :u32 = 0;
	let mut cycle :u8 = 0;
	let mut bin_val :char = 0 as char;
	let mut cycle_usize = cycle as usize;
	let mut c_val_int :i32 = 0;
	let mut c_val :u8 = 0;
	let mut combined_pixel = image::Rgba([0u8, 0u8, 0u8, 0u8]);
	for (x, y, pixel) in mask_image.pixels() {
		
		if mask_counter >= sequence_offset && mask_counter < sequence_offset_max {
			
			// println!("{} {} {}", mask_counter, mask_counter - sequence_offset, cycle);
			// println!("{:?} {}", pixel, pixel[cycle as usize]);
			
			bin_val = padded_bin[(mask_counter - sequence_offset) as usize];
			cycle_usize = cycle as usize;
				
			c_val_int = pixel[cycle_usize] as i32;
		    c_val = pixel[cycle_usize] as u8;
			if bin_val == 49 as char {
				if c_val_int.is_even() {
					c_val = (pixel[cycle_usize] as u8) + 1;
				}
			} else {
				if c_val_int.is_odd() {
					if c_val == 255 { 
						c_val = 254 as u8; 
					} else { 
						c_val = (pixel[cycle_usize] as u8) + 1; 
					}
				}
			}
				
			if cycle == 0 {
				combined_pixel = image::Rgba([c_val, pixel[1], pixel[2], pixel[3]]);
				final_values_list.push(combined_pixel);
			}
				
			if cycle == 1 {
				combined_pixel = image::Rgba([pixel[0], c_val, pixel[2], pixel[3]]);
				final_values_list.push(combined_pixel);
			}
				
			if cycle == 2 {
				combined_pixel = image::Rgba([pixel[0], pixel[1], c_val, pixel[3]]);
				final_values_list.push(combined_pixel);
			}
				
			if cycle == 3 {
				combined_pixel = image::Rgba([pixel[0], pixel[1], pixel[2], c_val]);
				final_values_list.push(combined_pixel);
			}
				
			cycle += 1;
			if cycle > 3 {
				cycle = 0;
			}	
		
		} else {
			combined_pixel = image::Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]);
			final_values_list.push(combined_pixel);
		}
		
		mask_counter += 1;
		
    }
	// println!("Final pixels list: {:?}", final_values_list);
	
	let mut final_image = image::ImageBuffer::new(mask_width, mask_height);
	let mut final_counter :u32 = 0;
	for (x, y, pixel) in final_image.enumerate_pixels_mut() {
		*pixel = final_values_list[final_counter as usize];
		final_counter += 1;
	}

	// Output 
	let masked_name = format!("encoded_{}", mask_file);
		let ref mut mask_fout = File::create(&Path::new(&masked_name)).unwrap();
		let _ = image::ImageRgba8(final_image).save(mask_fout, image::PNG);
		println!("Message saved to: {}", &masked_name);
	
	if toggle_save_raw == "raw" {
		let raw_name = format!("raw_encoded_{}", mask_file);
		let ref mut raw_fout = File::create(&Path::new(&raw_name)).unwrap();
		let _ = image::ImageRgba8(generated_image).save(raw_fout, image::PNG);
		println!("Encoding image file: {}", &raw_name);
	}

    let decode_password = format!("{}*{}*{}*{}*{}*{}*{}*{}", key, width, height, raw_start_x, raw_start_y, msg_length, sequence_offset, sequence_offset_max);
    println!("To decode: {}", decode_password);
    
    println!("------------------------------------------------------------");
}


































