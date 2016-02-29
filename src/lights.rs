use vecmath::{
    vec3_sub,
    vec3_add,
};

use types::{
    Vec3
};

pub struct LightSource {
    pub r: Vec3,
    pub direction: Vec3,
    pub intensity: f32,
    pub color: Vec3,
    pub point_source: bool,
}

impl LightSource {
    pub fn new(r: Vec3) -> LightSource {
        LightSource {
            r: r,
            direction: vec3_sub(r, [0.0; 3]),
            intensity: 1.0,
            color: [1.0; 3],
            point_source: true,
        }
    }
}
