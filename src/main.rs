use std::{error::Error, io::stdout, rc::Rc};

use nalgebra_glm as glm;
use rand::{self, Rng};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;

fn ray_color(ray: &ray::Ray, world: Box<&dyn hittable::Hittable>) -> ray::Vec3 {
    if let Some(record) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5 * (record.normal() + glm::vec3(1.0, 1.0, 1.0))
    } else {
        let unit_direction = glm::normalize(ray.direction());
        let t = 0.5 * (unit_direction[1] + 1.0);
        glm::lerp(&glm::vec3(1.0, 1.0, 1.0), &glm::vec3(0.5, 0.7, 1.0), t)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    // constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // world
    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(sphere::Sphere::new(glm::vec3(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(sphere::Sphere::new(
        glm::vec3(0.0, -100.5, -1.0),
        100.0,
    )));

    // camera
    let camera = camera::Camera::new();

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = glm::vec3(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                pixel_color += ray_color(
                    &camera.get_ray(u, v),
                    Box::new(&world),
                );
            }
            color::write_color(&mut stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
