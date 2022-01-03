use std::{
    error::Error,
    io::stdout,
};

use nalgebra_glm as glm;

mod ray;
mod color;

fn ray_color(ray: &ray::Ray) -> ray::Vec3 {
    let t = hit_sphere(&glm::vec3(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = glm::normalize(&(ray.at(t) - glm::vec3(0.0, 0.0, -1.0)));
        return 0.5 * glm::vec3(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = glm::normalize(ray.direction());
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * glm::vec3(1.0, 1.0, 1.0) + t * glm::vec3(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &ray::Vec3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = r.origin() - center;
    let a = glm::length2(r.direction());
    let half_b = glm::dot(&oc, r.direction());
    let c = glm::length2(&oc) - radius * radius;
    let discriminant = half_b * half_b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = glm::vec3(0.0, 0.0, 0.0);
    let horizontal = glm::vec3(viewport_width, 0.0, 0.0);
    let vertical = glm::vec3(0.0, viewport_height, 0.0);

    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - glm::vec3(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            eprint!("\rScanlines remaining: {} ", j);
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let pixel_color = ray_color(&ray::Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            ));
            color::write_color(&mut stdout(), pixel_color)?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
