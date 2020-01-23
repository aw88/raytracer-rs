use crate::vector3::Vector3;
use crate::canvas::Canvas;

mod canvas;
mod vector3;

fn main() -> std::io::Result<()> {
    let mut c = Canvas::new(320, 240);
    let a = Vector3::new(1.0, 2.0, 3.0);
    println!("a: {:?}", a);

    c.put_pixel(10, 10, Vector3::new(1.0, 1.0, 1.0));
    c.put_pixel(15, 10, Vector3::new(1.0, 0.0, 0.0));

    c.write_to_file("output.ppm")?;

    Ok(())
}
