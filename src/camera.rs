use std::f64::NAN;

use types::{
    Vec3,
    Mat3,
};

pub struct Camera {
    pub width: f64,
    pub height: f64,
    pub r: Vec3,
    pub theta: Vec3,
    pub projection: Mat3,
    pub screen: f64,
}

use math::{
    mat_rotation,
    mat3xv3_mul,
};

use vecmath::{
    vec3_sub,
    vec3_add,
    vec3_square_len,
    vec3_scale,
};

impl Camera {

    /// Create a default camera
    pub fn default() -> Camera {
        let mut camera = Camera {
            width: 200.0,
            height: 200.0,
            r: [0.0, -200.0, -250.0],
            theta: [-0.4, 0.0, 0.0],
            screen: 300.0,
            projection: [[0.0; 3]; 3]
        };
        camera.update_projection();
        camera
    }

    pub fn update_projection(&mut self) {
        self.projection = mat_rotation(self.theta);
    }

    /// Get the x, y location as projected on the screen
    #[inline(always)]
    pub fn projected(&self, r: Vec3) -> [f64; 2] {
        let dr = vec3_sub(r, self.r);
        let d =  mat3xv3_mul(self.projection, dr);
        let scale = self.screen / d[2] / 1500.0;
        let bx = scale * d[0] * self.width + self.width  / 2.0;
        let by = scale * d[1] * self.height + self.height / 2.0;
        if d[2] < 0.0 {
            [NAN, NAN]
        } else {
            [bx, by]
        }
    }

    pub fn look_at(&mut self, r: Vec3) {
        let dr = vec3_sub(r, self.r);
        let d = vec3_square_len(dr);
        let dr = vec3_scale(dr, 1.0/d/2.0);
        self.theta[0] = (dr[1]/d).acos();
        self.theta[1] = (dr[0]/d).acos();
    }

    pub fn rotate(&mut self, theta: Vec3) {
        self.theta = vec3_add(self.theta, theta)
    }

    pub fn trans(&mut self, r: Vec3) {
        vec3_add(r, self.r);
    }

    pub fn put(&mut self, r: Vec3) {
        self.r = r;
    }

}
