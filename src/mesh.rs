pub use float::{
    One,
    Zero,
};

use vecmath::{
    vec3_add,
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

    pub fn translate(&self, r: Vec3) -> Face {
        Face {
            color: self.color,
            points: [
                vec3_add(self.points[0], r),
                vec3_add(self.points[1], r),
                vec3_add(self.points[2], r),
            ],
        }
    }

}

// ======================================================================
// Mesh

#[derive(Debug)]
pub struct Mesh {
    faces: Vec<Face>,
    wireframe: bool,
    r: Vec3,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            r: [0.0; 3],
            faces: vec![],
            wireframe: false
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
            let translated = face.translate(self.r);
            Polygon::new(translated.color)
                .draw(&translated.project(camera),
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
                // println!("{:.1}, {:.1}, {:.1} {:.1}", line[0], line[1], line[2], line[3]);
                Line::new(face.color, 0.5).draw(*line, ds, transform, g);
            }
        }
    }

    pub fn position(mut self, r: Vec3) -> Mesh {
        self.r = r;
        self
    }

    pub fn translate(&mut self, r: Vec3) {
        self.r = vec3_add(self.r, r);
    }

    ///  z         a --- b
    ///  ^         | \   |
    ///  |         |   \ |
    ///  +-> x     c --- d
    pub fn new_terrain() -> Mesh {
        let size = 600.0;
        let a = [-size,  size, 0.0];
        let b = [ size,  size, 0.0];
        let c = [-size, -size, 0.0];
        let d = [ size, -size, 0.0];
        Mesh::new().face(Face::new(a, c, d)).face(Face::new(a, b, d))
    }

    ///  z         a --- b
    ///  ^         | \   |
    ///  |         |   \ |
    ///  +-> x     c --- d
    pub fn new_domain() -> Mesh {
        let size = 600.0;
        let a =  [-size,  size/2.0, 0.0];
        let b =  [ size,  size/2.0, 0.0];
        let c =  [-size, -size/2.0, 0.0];
        let d =  [ size, -size/2.0, 0.0];
        let aa = [-size,  size/2.0, size];
        let bb = [ size,  size/2.0, size];
        let cc = [-size, -size/2.0, size];
        let dd = [ size, -size/2.0, size];
        Mesh::new()
            .face(Face::new(a, b, d).color([0.5, 0.5, 1.0, 0.5]))
            .face(Face::new(a, c, d).color([0.5, 0.5, 1.0, 0.5]))
            .face(Face::new(aa, bb, dd))
            .face(Face::new(aa, cc, dd))
            .face(Face::new(a, aa, c))
            .face(Face::new(a, c, cc))
            .face(Face::new(b, bb, d))
            .face(Face::new(b, d, dd))
            .wireframe(true)
    }


}
