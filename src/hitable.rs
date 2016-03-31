use ray::Ray;
use vec3::Vec3;
use material::Material;

#[derive (Copy, Clone)]
pub struct Hit<T> {
    pub t: f32,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
}

pub trait Hitable<T> {
    fn hit(&self, ray: &Ray<T>, t_min: f32, t_max: f32, record: &mut Hit<T>) -> bool;
}

impl Hit<f32> {
    pub fn new() -> Hit<f32> {
        Hit {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn hit_in_list<T: Hitable<f32>> (ray: &Ray<f32>,
                                   t_min: f32,
                                   t_max: f32,
                                   list: &mut Vec<T>) -> Option<Hit<f32>> {
    let mut itr_record = Hit::new();
    let mut ret_record = Hit::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for obj in &mut list.iter() {
        if obj.hit(ray, t_min, closest_so_far, &mut itr_record) {
            hit_anything = true;
            closest_so_far = itr_record.t;
            ret_record = itr_record;
        }
    }
    if hit_anything {
        Some(ret_record)
    } else {
        None
    }
}
