use std;

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

use graphics;

use graphics::{
    Polygon,
    Line,
};

use camera::Camera;

// ======================================================================
// Faces

/// A 3D Face
#[derive(Debug)]
pub struct Face {
    vertices: [usize; 3],
    color: [f32; 4],
}

#[derive(Debug)]
pub struct Vertex {
    r: Vec3,
    faces: Vec<usize>,
}

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    r: Vec3,
    wireframe: bool,
    theta: Vec3,
}


impl Face {
    pub fn color(mut self, color: [f32; 4]) -> Face {
        self.color = color;
        self
    }

    pub fn new(a: usize, b: usize, c: usize) -> Face {
        Face { vertices: [a, b, c], color: [0.5; 4] }
    }

    pub fn project(&self, mesh: &Mesh, camera: &Camera) -> graphics::types::Triangle {
        [
            camera.projected(mesh.vertices[self.vertices[0]].r),
            camera.projected(mesh.vertices[self.vertices[1]].r),
            camera.projected(mesh.vertices[self.vertices[2]].r),
        ]
    }

    pub fn project_lines(&self, mesh: &Mesh, camera: &Camera) -> [[f64; 4]; 3] {
        let p = self.project(mesh, camera);
        [
            [p[0][0], p[0][1], p[1][0], p[1][1]],
            [p[1][0], p[1][1], p[2][0], p[2][1]],
            [p[2][0], p[2][1], p[0][0], p[0][1]],
        ]
    }
}

// ======================================================================
// Vertices

impl Vertex {
    fn new(r: Vec3) -> Vertex {
        Vertex { r: r , faces: vec![]}
    }
}

// ======================================================================
// Mesh

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            r: [0.0; 3],
            theta: [0.0; 3],
            vertices: vec![],
            faces: vec![],
            wireframe: false,
        }
    }

    pub fn add_vertex(&mut self, r: Vec3) -> usize {
        self.vertices.push(Vertex::new(r));
        self.vertices.len() - 1
    }

    pub fn add_face(&mut self, t: Face) {
        self.faces.push(t)
    }

    pub fn translate(&mut self, r: Vec3) {

    }

    pub fn rotate(&mut self, theta: Vec3) {
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
                .draw(&face.project(self, camera),
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
            for line in &face.project_lines(self, camera) {
                Line::new(face.color, 0.5).draw(*line, ds, transform, g);
            }
        }
    }

    pub fn position(mut self, r: Vec3) -> Mesh {
        self.r = r;
        self
    }

    pub fn new_terrain(size: f64, res: f64) -> Mesh {
        let mut mesh = Mesh::new();
        mesh
    }

    pub fn new_domain() -> Mesh {
        Mesh::new()
    }

    pub fn new_diamond(size: f64) -> Mesh {
        let mut mesh = Mesh::new();
        let a = mesh.add_vertex([-size,  0.0,  0.0]);
        let b = mesh.add_vertex([  0.0,  0.0,  size]);
        let c = mesh.add_vertex([ size,  0.0,  0.0]);
        let d = mesh.add_vertex([  0.0,  0.0, -size]);
        let e = mesh.add_vertex([  0.0,  size, 0.0]);
        let f = mesh.add_vertex([  0.0, -size, 0.0]);
        mesh.add_face(Face::new(a, e, b));
        mesh.add_face(Face::new(b, e, c));
        mesh.add_face(Face::new(c, e, d));
        mesh.add_face(Face::new(d, e, a));
        mesh.add_face(Face::new(a, f, b));
        mesh.add_face(Face::new(b, f, c));
        mesh.add_face(Face::new(c, f, d));
        mesh.add_face(Face::new(d, f, a));
        mesh
    }

}
