use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::vec::{self, Color, Vec};

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Vec)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + vec::random_unit_vector();
        Some((
            Ray::new(
                rec.p,
                if vec::near_zero(scatter_direction) {
                    rec.normal
                } else {
                    scatter_direction
                },
            ),
            self.albedo,
        ))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = vec::reflect(&r.direction().normalize(), &rec.normal);
        if reflected.dot(&rec.normal) > 0.0 {
            Some((
                Ray::new(rec.p, reflected + self.fuzz * vec::random_in_unit_sphere()),
                self.albedo,
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r.direction().normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        Some((
            Ray::new(
                rec.p,
                if refraction_ratio * sin_theta > 1.0
                    || Dielectric::reflectance(cos_theta, refraction_ratio) > util::random_f64()
                {
                    vec::reflect(&unit_direction, &rec.normal)
                } else {
                    vec::refract(&unit_direction, &rec.normal, refraction_ratio)
                },
            ),
            Color::new(1.0, 1.0, 1.0),
        ))
    }
}
