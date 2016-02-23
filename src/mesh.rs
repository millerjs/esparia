
pub use float::{
    One,
    Zero,
};

use vecmath::{
    vec3_add,
    vec3_sub,
};

use math::{
    mat_rotation,
    mat3xv3_mul,
    vec3_rotate_around,
};

use types::{
    Vec3,
};

use graphics::Graphics;
use graphics::default_draw_state;

use graphics::math::{
    Matrix2d,
};

use graphics::types::{
    Triangle,
};

use graphics::{
    Polygon,
    Line,
};

use camera::Camera;

// ======================================================================
// Faces

/// A 3D Triangle
#[derive(Debug)]
pub struct Face {
    points: [Vec3; 3],
    color: [f32; 4],
}


impl Face {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Face {
        Face { points: [a, b, c], color: [0.5; 4] }
    }

    pub fn color(mut self, color: [f32; 4]) -> Face {
        self.color = color;
        self
    }

    pub fn project(&self, camera: &Camera) -> Triangle {
        [
            camera.projected(self.points[0]),
            camera.projected(self.points[1]),
            camera.projected(self.points[2]),
        ]
    }

    pub fn project_lines(&self, camera: &Camera) -> [[f64; 4]; 3] {
        let p = self.project(camera);
        [
            [p[0][0], p[0][1], p[1][0], p[1][1]],
            [p[1][0], p[1][1], p[2][0], p[2][1]],
            [p[2][0], p[2][1], p[0][0], p[0][1]],
        ]
    }

    pub fn translate(&mut self, r: Vec3) {
        self.points = [
            vec3_add(self.points[0], r),
            vec3_add(self.points[1], r),
            vec3_add(self.points[2], r),
        ]
    }

    pub fn rotate(&mut self, theta: Vec3, r: Vec3) {
        let rot = mat_rotation(theta);
        self.points = [
            vec3_rotate_around(self.points[0], rot, r),
            vec3_rotate_around(self.points[1], rot, r),
            vec3_rotate_around(self.points[2], rot, r),
        ]
    }
}


// ======================================================================
// Mesh

#[derive(Debug)]
pub struct Mesh {
    r: Vec3,
    faces: Vec<Face>,
    wireframe: bool,
    theta: Vec3,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            r: [0.0; 3],
            theta: [0.0; 3],
            faces: vec![],
            wireframe: false,
        }
    }

    pub fn translate(&mut self, r: Vec3) {
        self.r = vec3_add(self.r, r);
        for face in self.faces.iter_mut() {
            face.translate(r);
        }
    }

    pub fn rotate(&mut self, theta: Vec3) {
        for face in self.faces.iter_mut() {
            face.rotate(theta, self.r);
        }
    }

    pub fn face(mut self, face: Face) -> Mesh {
        self.faces.push(face);
        self
    }

    pub fn wireframe(mut self, wireframe: bool) -> Mesh {
        self.wireframe = wireframe;
        self
    }

    pub fn draw<G>(&self, camera: &Camera, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        match self.wireframe {
            true => self.draw_wireframe(camera, transform, g),
            false => self.draw_filled(camera, transform, g),
        }
    }


    pub fn draw_filled<G>(&self, camera: &Camera, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        for face in self.faces.iter() {
            Polygon::new(face.color)
                .draw(&face.project(camera),
                      default_draw_state(),
                      transform,
                      g);
        }
    }

    pub fn draw_wireframe<G>(&self, camera: &Camera, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        let ds = default_draw_state();
        for face in self.faces.iter() {
            for line in &face.project_lines(camera) {
                Line::new(face.color, 0.5).draw(*line, ds, transform, g);
            }
        }
    }

    pub fn position(mut self, r: Vec3) -> Mesh {
        self.r = r;
        self
    }

    ///  z         a --- b
    ///  ^         | \   |
    ///  |         |   \ |
    ///  +-> x     c --- d
    pub fn new_terrain(size: f64, res: f64) -> Mesh {
        let mut mesh = Mesh::new();
        let n_x = (size / res) as i32;
        let n_z = (size / res) as i32;
        for i in 0..n_x {
            for j in 0..n_z {
                let x = i as f64 * res - size/2.0;
                let z = j as f64 * res - size/2.0;
                let y1 = x.sin() * z.cos() * 100.0;
                let y2 = (x+res).sin() * (z+res).cos() * 100.0;
                mesh = mesh.face(Face::new(
                    [x, y1, z],
                    [x+res, y2, z],
                    [x, y2, z+res]));
                mesh = mesh.face(Face::new(
                    [x+res, y2, z+res],
                    [x+res, y2, z],
                    [x, y1, z+res]));
            }
        }
        mesh
    }

    ///  y         a --- b
    ///  ^         | \   |
    ///  |         |   \ |
    ///  +-> x     c --- d
    pub fn new_domain() -> Mesh {
        let size = 800.0;
        let a =  [-size, -size, 0.0];
        let b =  [ size, -size, 0.0];
        let c =  [-size, 0.0, 0.0];
        let d =  [ size, 0.0, 0.0];
        let aa = [-size, -size, size];
        let bb = [ size, -size, size];
        let cc = [-size, 0.0, size];
        let dd = [ size, 0.0, size];
        Mesh::new()
            .face(Face::new(aa, bb, dd))
            .face(Face::new(aa, cc, dd))
            .face(Face::new(a, aa, c))
            .face(Face::new(aa, cc, c))
            .face(Face::new(b, bb, d))
            .face(Face::new(bb, dd, d))
            .wireframe(true)
    }


    ///  y            a
    ///  ^         b ef d
    ///  |           c
    ///  +-> x
    pub fn new_diamond(size: f64) -> Mesh {
        let a =  [0.0,  size*2.0, 0.0];
        let b =  [-size, 0.0, 0.0];
        let c =  [0.0,  -size*2.0, 0.0];
        let d =  [size,  0.0, 0.0];
        let e =  [0.0,  0.0, size];
        let f =  [0.0,  0.0, -size];
        Mesh::new()
            .face(Face::new(a, b, e))
            .face(Face::new(a, b, f))
            .face(Face::new(a, d, e))
            .face(Face::new(a, d, f))
            .face(Face::new(c, b, e))
            .face(Face::new(c, b, f))
            .face(Face::new(c, d, e))
            .face(Face::new(c, d, f))
            .wireframe(true)
    }


}
