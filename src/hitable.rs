use ray::Ray;
use vec3::Vec3;
use material::Material;

#[derive (Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        record: &mut Hit
    ) -> bool;
}

impl Hit {
    pub fn new() -> Hit {
        Hit {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

pub fn hit_in_list<T: Hitable> (
    ray: &Ray,
    t_min: f32,
    t_max: f32,
    list: &mut Vec<T>
) -> Option<Hit>
{
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
