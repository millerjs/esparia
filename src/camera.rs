use types::{
    Vec2,
    Vec3,
    Vec4,
    Mat3,
};

pub struct Camera {
    pub width: f64,
    pub height: f64,
    r: Vec3,
    theta: Vec3,
    projection: Mat3,
    fovd: f64,
    screen: f64,
}

use math::{
    mat_rotation,
    mat3xv3_mul,
};

use vecmath::{
    vec3_sub,
};

impl Camera {

    /// Create a default camera
    pub fn default() -> Camera {
        let mut camera = Camera {
            fovd: 60.0,
            width: 200.0,
            height: 200.0,
            r: [0.0, 100.0, -100.0],
            theta: [0.0; 3],
            screen: 3.0,
            projection: [[0.0; 3]; 3]
        };
        camera.update_projection();
        camera
    }

    pub fn update_projection(&mut self) {
        self.projection = mat_rotation(self.theta);
    }

    /// Get the x, y location as projected on the screen
    pub fn projected(&self, r: Vec3) -> [f64; 2] {
        let d =  mat3xv3_mul(self.projection, vec3_sub(r, self.r));
        let scale = d[2] * self.screen;
        let bx = d[0] * self.width / scale;
        let by = d[1] * self.height / scale;
        [self.width/2.0 + bx, self.height/2.0 - by]
    }

}
