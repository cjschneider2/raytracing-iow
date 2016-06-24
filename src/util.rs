use vec3::Vec3;
use rand;

pub fn rand_in_unit_sphere() -> Vec3 {
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let mut vr = Vec3::new(rand::random::<f32>(),
                           rand::random::<f32>(),
                           rand::random::<f32>()) - v1;
    while vr.dot(vr) >= 1.0 {
        vr = Vec3::new(rand::random::<f32>(),
                       rand::random::<f32>(),
                       rand::random::<f32>()) - v1;
    }
    vr
}
