use crate::{hittable, ray, util, vec};

pub trait Material {
    // output: Option<output ray, attenuation>
    fn scatter(
        &self,
        ray: &ray::Ray,
        record: &hittable::HitRecord,
    ) -> Option<(ray::Ray, vec::Vec3)>;
}

pub struct Lambertian {
    albedo: vec::Vec3,
}

impl Lambertian {
    pub fn new(albedo: vec::Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &ray::Ray,
        record: &hittable::HitRecord,
    ) -> Option<(ray::Ray, vec::Vec3)> {
        let mut scatter_direction = record.normal() + vec::random::unit_vector();
        if vec::near_zero(scatter_direction) {
            scatter_direction = record.normal().clone()
        }
        Some((ray::Ray::new(record.p(), scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: vec::Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: vec::Vec3, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &ray::Ray,
        record: &hittable::HitRecord,
    ) -> Option<(ray::Ray, vec::Vec3)> {
        let reflected = vec::reflect(&ray.direction().normalize(), &record.normal());
        if reflected.dot(&record.normal()) > 0.0 {
            Some((
                ray::Ray::new(
                    record.p(),
                    reflected + self.fuzz * vec::random::unit_vector(),
                ),
                self.albedo,
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &ray::Ray,
        record: &hittable::HitRecord,
    ) -> Option<(ray::Ray, vec::Vec3)> {
        let refraction_ratio = if record.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > util::random_f64()
        {
            vec::reflect(&unit_direction, record.normal())
        } else {
            vec::refract(&unit_direction, record.normal(), refraction_ratio)
        };

        Some((
            ray::Ray::new(record.p(), direction),
            vec::Vec3::from_element(1.0),
        ))
    }
}
