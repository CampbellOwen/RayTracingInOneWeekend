use rand::Rng;

use crate::math::rand_in_unit_sphere;

use super::Ray;
use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    lower_left_corner: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64,
    time_0: f64,
    time_1: f64,
}

impl Camera {
    pub fn new(
        look_from: DVec3,
        look_at: DVec3,
        up: DVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_0: f64,
        time_1: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_dist);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture / 2.0,
            time_0,
            time_1,
        }
    }

    pub fn new_instant(
        look_from: DVec3,
        look_at: DVec3,
        up: DVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        Camera::new(
            look_from,
            look_at,
            up,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
            0.0,
            0.0,
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let mut rng = rand::thread_rng();
        let random_in_lens = rand_in_unit_sphere(&mut rng) * self.lens_radius;
        let offset = (self.u * random_in_lens.x) + (self.v * random_in_lens.y);

        let time = if (self.time_1 - self.time_0) > 0.000001 {
            rng.gen_range(self.time_0..self.time_1)
        } else {
            0.0
        };

        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - self.origin
                - offset,
            time,
        }
    }
}
