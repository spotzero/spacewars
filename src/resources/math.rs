use amethyst::core::math::Vector3;

pub fn unit_vector(v: &Vector3<f32>) -> Vector3<f32> {
    if let Some(unit_dir) = v.try_normalize(1.0e-6) {
        unit_dir
    } else {
        Vector3::new(0., 0., 0.)
    }
}
