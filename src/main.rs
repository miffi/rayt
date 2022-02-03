use std::{error::Error, io::stdout, rc::Rc};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec;
mod util;

fn ray_color(ray: &ray::Ray, world: Box<&dyn hittable::Hittable>, depth: u32) -> vec::Vec3 {
    if depth <= 0 {
        vec::Vec3::zeros()
    } else if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered_ray, attenuation)) = record.material().scatter(ray, &record) {
            attenuation.component_mul(&ray_color(&scattered_ray, world, depth - 1))
        } else {
            vec::Vec3::zeros()
        }
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        vec::Vec3::from_element(1.0).lerp(&vec::Vec3::new(0.5, 0.7, 1.0), t)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // constants
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // world
    let material_ground = Rc::new(material::Lambertian::new(vec::Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(vec::Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Dielectric::new(1.5));
    let material_right = Rc::new(material::Metal::new(vec::Vec3::new(0.8, 0.6, 0.2), 0.0));

    let mut world = hittable_list::HittableList::new();
    world.add(Rc::new(sphere::Sphere::new(
        vec::Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(sphere::Sphere::new(
        vec::Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(sphere::Sphere::new(
        vec::Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(sphere::Sphere::new(
        vec::Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    /* let r: f64 = (std::f64::consts::FRAC_PI_4).cos();
    let mut world = hittable_list::HittableList::new();
    
    let material_left = Rc::new(material::Lambertian::new(vec::Vec3::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(material::Lambertian::new(vec::Vec3::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(sphere::Sphere::new(
        vec::Vec3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));

    world.add(Rc::new(sphere::Sphere::new(
        vec::Vec3::new(r, 0.0, -1.0),
        r,
        material_right,
    ))); */

    // camera
    let lookfrom = vec::Vec3::new(3.0, 3.0, 2.0);
    let lookat = vec::Vec3::new(0.0, 0.0, -1.0);
    let vup = vec::Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).norm();
    let aperture = 2.0;

    let camera = camera::Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = vec::Vec3::zeros();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + util::random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + util::random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                pixel_color += ray_color(&camera.get_ray(u, v), Box::new(&world), MAX_DEPTH);
            }
            color::write_color(&mut stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
