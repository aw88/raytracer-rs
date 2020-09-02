use crate::canvas::Canvas;
use crate::sphere::{ Ray, Sphere };
use crate::vector3::Vector3;

mod canvas;
mod sphere;
mod vector3;

fn main() -> std::io::Result<()> {
    let mut c: Canvas = Canvas::new(320, 240);

    let sphere: Sphere = Sphere::new(
        Vector3::new(0.0, 0.0, 15.0),
        100.0,
    );

    let half_width = c.width as f32 * 0.5;
    let half_height = c.height as f32 * 0.5;

    for x in 0..c.width {
        for y in 0..c.height {
            let origin = Vector3::new(x as f32 - half_width, y as f32 - half_height, 0.0);
            let direction = Vector3::new(0.0, 0.0, 1.0);
            let ray = Ray::new(origin, direction);

            let intersection = sphere.intersect(ray);

            if intersection.is_some() {
                let t = intersection.unwrap();
                let point = ray.sample(t);
                let normal = (sphere.get_normal(point) + Vector3::ONE) * 0.5;

                c.put_pixel(x, y, normal);
            }
        }
    }

    c.write_to_file("output.ppm")?;

    Ok(())
}
