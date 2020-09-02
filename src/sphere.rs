use crate::vector3::Vector3;
use std::mem::swap;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub radius: f32,
    pub radius2: f32,
    pub origin: Vector3,
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn sample(&self, t: f32) -> Vector3 {
        self.origin + (self.direction * t)
    }
}

impl Sphere {
    pub fn new(origin: Vector3, radius: f32) -> Sphere {
        Sphere { origin, radius, radius2: radius * radius }
    }

    pub fn intersect(&self, ray: Ray) -> Option<f32> {
        let l = self.origin - ray.origin;
        let tca = l.dot(ray.direction);
        let d2 = l.length_sqr() - tca * tca;

        if d2 > self.radius2 {
            return None;
        }

        let thc = (self.radius2 - d2).sqrt();

        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        if t0 < 0.0 {
            t0 = t1;
            if t0 < 0.0 {
                return None;
            }
        }

        return Some(t0);
    }

    pub fn get_normal(&self, point: Vector3) -> Vector3 {
        (point - self.origin).normalized()
    }
}
