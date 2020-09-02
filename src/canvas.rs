use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::vector3::Vector3;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vector3>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![Vector3::ZERO; width * height],
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: Vector3) -> () {
        self.pixels[y * self.width + x] = pixel;
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        file.write(b"P3\n")?;
        file.write(format!("{} {}\n", self.width, self.height).as_bytes())?;
        file.write(b"255\n")?;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = &self.pixels[y * self.width + x];
                let r = (pixel.x * 255.0) as i32;
                let g = (pixel.y * 255.0) as i32;
                let b = (pixel.z * 255.0) as i32;

                file.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
            }
        }

        Ok(())
    }
}