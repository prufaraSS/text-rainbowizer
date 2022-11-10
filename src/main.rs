use crossterm::{
    queue,
    execute,
    style::{
        Print,
        Stylize,
        Color
    },
    event::{read}
};
use std::io::{
    stdin,
    stdout,
    Write
};

use bracket_color::prelude::HSV;

use std::fs::{OpenOptions,File,read_to_string};

fn draw_text(text:&String,h:f32,s:f32,v:f32,iters:f32,rotation:u16,file:&mut File) {
    let mut lines = text.lines();
    let mut lines_iter = 0;
    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        let mut offset = lines_iter;
        let mut skip = 0;
        while let Some(ch) = chars.next() {
            let rgb = HSV::from_f32( (h+offset as f32/iters) % iters ,s,v).to_rgb();
            offset += 1;
            let result = ch.with(
                Color::Rgb {
                    r: (rgb.r*255.0) as u8,
                    g: (rgb.g*255.0) as u8,
                    b: (rgb.b*255.0) as u8
                } 
            );
            queue!(
                stdout(),
                Print(result)
            ).unwrap();
            file.write_all(&format!("{}",result).as_bytes());
        }
        queue!(
            stdout(),
            Print("\n")
        ).unwrap();

        file.write(b"\n");

        skip += 1;
        match rotation {
            1 => if skip % 2 == 0 {lines_iter += 1},
            2 => lines_iter += 1,
            3 => lines_iter += 2,
            4 => lines_iter += 3,
            _ => ()
        }
        
    }
    queue!(
        stdout(),
        Print("\n")
    ).unwrap();

    file.write(b"\n");

    file.flush().unwrap();
    stdout().flush().unwrap();
}

const WARNING_MSG:&str = "WARNING! This program is not used to be finished.
No error handling and optimizations provided. ONLY FUNCTIONALITY."; 
const ROTATION_MSG:&str = "\n\nRotation:
Options:
1 - 15° rotation
2 - 45° rotation
3 - ~67,5° rotation (every 2 pixels)
4 - ~75° rotation (every 3 pixels)
Any other number - No rotation\n"; //these are too large uh

fn main() {
    let input = read_to_string("input.txt").unwrap();

    execute!(
        stdout(),
        Print(WARNING_MSG.red()),
        Print("\n\nHow many colors you need? (max: 510):")
    ).unwrap();
    let mut iterations = String::new();
    stdin().read_line(&mut iterations).unwrap();
    let iterations:f32 = iterations.trim().parse::<f32>().unwrap().min(510f32);
    execute!(
        stdout(),
        Print(ROTATION_MSG)
    ).unwrap();

    let mut rainbow_rotation = String::new();
    stdin().read_line(&mut rainbow_rotation).unwrap();
    let rainbow_rotation = rainbow_rotation.trim().parse::<u16>().unwrap();


    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("output.txt")
        .unwrap();

    for i in 0..iterations as u16 {
        let color = i as f32 / iterations;
        let saturation = 1f32;
        let value = 1f32;
        draw_text(&input,color,saturation,value,iterations,rainbow_rotation,&mut file);
    }

    execute!(
        stdout(),
        Print("Checkout the result! All data was written in 'output.txt' in program's folder.")
    ).unwrap();
    read().unwrap();
}
