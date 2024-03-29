mod aabb;
mod bvh;
mod camera;
mod circle;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod texture;
mod util;
mod vec;

use circle::Sphere;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use moving_sphere::MovingSphere;
use ray::Ray;
use std::{fs::File, rc::Rc};
use texture::{Image, SolidColor};
use vec::{Color, Point, Vec};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        Color::new(0.0, 0.0, 0.0)
    } else if let Some(rec) = world.hit(r, 0.001..f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(r, &rec) {
            attenuation.component_mul(&ray_color(&scattered, world, depth - 1))
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let t = 0.5 * (r.unit_direction().y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::from_texture(Rc::new(texture::Checker::new(
        Rc::new(SolidColor::new(0.2, 0.3, 0.1)),
        Rc::new(SolidColor::new(0.9, 0.9, 0.9)),
    ))));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random_f64();
            let center = Vec::new(
                a as f64 + 0.9 * util::random_f64(),
                0.2,
                b as f64 + 0.9 * util::random_f64(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                let sphere_material: Rc<dyn Material> = {
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = vec::random().component_mul(&vec::random());
                        Rc::new(Lambertian::new(albedo))
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = vec::random_range(0.5..1.0);
                        let fuzz = util::random_f64_range(0.0..0.5);
                        Rc::new(Metal::new(albedo, fuzz))
                    } else {
                        // glass
                        Rc::new(Dielectric::new(1.5))
                    }
                };

                if choose_mat < 0.8 {
                    let center2 = center + Vec::new(0.0, util::random_f64_range(0.0..0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0..1.0,
                        0.2,
                        sphere_material,
                    )))
                } else {
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material_earth = Rc::new(Lambertian::from_texture(Rc::new(
        Image::from_png_file(&File::open("res/2k_earth_daymap.png").unwrap()).unwrap(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material_earth.clone(),
    )));

    let material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material_earth,
    )));

    /* let mut bv = HittableList::new();
    bv.add(Rc::new(bvh::BvhNode::new(&world, 0.0..1.0)));
    bv */
    world
}

fn main() -> std::io::Result<()> {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let world = random_scene();

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:03}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + util::random_f64()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + util::random_f64()) / (IMAGE_HEIGHT as f64 - 1.0);
                pixel_color += ray_color(&cam.get_ray(u, v, 0.0..1.0), &world, MAX_DEPTH);
            }
            vec::write_color(&mut std::io::stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    eprintln!();

    Ok(())
}
