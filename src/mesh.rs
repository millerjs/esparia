use rand;

use std;

pub use float::{
    One,
    Zero,
};

use vecmath::{
    vec3_add,
    vec3_sub,
    vec3_cross,
    vec3_dot,
    vec3_normalized,
};

use math::{
    mat_rotation,
    mat3xv3_mul,
    vec3_rotate_around,
};

use types::{
    Color,
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
use lights::LightSource;

// ======================================================================
// Faces

/// A 3D Face
#[derive(Debug,Clone)]
pub struct Face {
    vertices: [usize; 3],
    color: Color,
}

#[derive(Debug,Clone)]
pub struct Vertex {
    r: Vec3,
    faces: Vec<usize>,
}

#[derive(Debug,Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    r: Vec3,
    wireframe: bool,
    theta: Vec3,
}


// ======================================================================
// Light

pub struct Light {
    r: Vec3,
    intensity: f32,
}

impl Light {
    fn new(r: Vec3, intensity: f32) -> Light {
        Light { r: r, intensity: intensity }
    }
}

// ======================================================================
// Face

impl Face {
    pub fn color(mut self, color: Color) -> Face {
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

    pub fn normal(&self, mesh: &Mesh) -> Vec3 {
        vec3_normalized(vec3_cross(
            mesh.vertices[self.vertices[1]].r,
            mesh.vertices[self.vertices[2]].r
        ))
    }

    pub fn shade(&self, mesh: &Mesh, lights: &Vec<LightSource>) -> Color {
        let norm = self.normal(mesh);
        let dot = vec3_dot(norm, vec3_normalized(lights[0].r)).abs();
        let shade = (1.0 - dot * 0.4) as f32;
        [
            self.color[0] * shade,
            self.color[1] * shade,
            self.color[2] * shade,
            self.color[3],
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

    pub fn add_face(&mut self, face: Face) {
        self.faces.push(face.clone());
        let idx = self.faces.len() - 1;
        for vertex in &face.vertices {
            self.vertices[*vertex].faces.push(idx)
        }
    }

    pub fn translate(&mut self, r: Vec3) {
        self.r = vec3_add(r, self.r);
        for vertex in self.vertices.iter_mut() {
            vertex.r = vec3_add(r, vertex.r);
        }
    }

    pub fn rotate(&mut self, theta: Vec3) {
        self.theta = vec3_add(self.theta, theta);
        let rotation = mat_rotation(theta);
        for vertex in self.vertices.iter_mut() {
            vertex.r = vec3_rotate_around(vertex.r, rotation, self.r);
        }
    }

    pub fn wireframe(mut self, wireframe: bool) -> Mesh {
        self.wireframe = wireframe;
        self
    }

    pub fn draw<G>(
        &self,
        camera: &Camera,
        lights: &Vec<LightSource>,
        transform: Matrix2d,
        g: &mut G
    ) where G: Graphics
    {
        match self.wireframe {
            true => self.draw_wireframe(camera, lights, transform, g),
            false => self.draw_filled(camera, lights, transform, g),
        }
    }


    pub fn draw_filled<G>(
        &self,
        camera: &Camera,
        lights: &Vec<LightSource>,
        transform: Matrix2d,
        g: &mut G
    ) where G: Graphics
    {
        for face in self.faces.iter() {
            Polygon::new(face.shade(self, lights))
                .draw(&face.project(self, camera),
                      default_draw_state(),
                      transform,
                      g);
        }
    }

    pub fn draw_wireframe<G>(
        &self,
        camera: &Camera,
        lights: &Vec<LightSource>,
        transform: Matrix2d,
        g: &mut G
    ) where G: Graphics
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
        let n = (size / res) as usize;
        for j in 0..n {
            for i in 0..n {
                let x = i as f64 * res - size / 2.0;
                let z = j as f64 * res - size / 2.0;
                let y = ((x/100.0).sin() + (z/100.0).cos())*100.0;
                mesh.add_vertex([x,  y,  z]);
            }
        }
        let color = [0.0, 0.25, 0., 1.0];
        for i in 0..n-1 {
            for j in 0..n-1 {
                let a = j * n + i;
                let b = j * n + i + 1;
                let c = (j + 1) * n + i;
                let d = (j + 1) * n + i + 1;
                mesh.add_face(Face::new(a, b, c).color(color));
                mesh.add_face(Face::new(d, b, c).color(color));
            }
        }
        mesh
    }


    pub fn new_diamond(size: f64) -> Mesh {
        let mut mesh = Mesh::new();
        let a = mesh.add_vertex([-size,  0.0,  0.0]);
        let b = mesh.add_vertex([  0.0,  0.0,  size]);
        let c = mesh.add_vertex([ size,  0.0,  0.0]);
        let d = mesh.add_vertex([  0.0,  0.0, -size]);
        let e = mesh.add_vertex([  0.0,  size*2.0, 0.0]);
        let f = mesh.add_vertex([  0.0, -size*2.0, 0.0]);
        let color = [0.1, 0.1, 0.9, 0.4];
        mesh.add_face(Face::new(a, e, b).color(color));
        mesh.add_face(Face::new(b, e, c).color(color));
        mesh.add_face(Face::new(c, e, d).color(color));
        mesh.add_face(Face::new(d, e, a).color(color));
        mesh.add_face(Face::new(a, f, b).color(color));
        mesh.add_face(Face::new(b, f, c).color(color));
        mesh.add_face(Face::new(c, f, d).color(color));
        mesh.add_face(Face::new(d, f, a).color(color));
        mesh
    }

}
