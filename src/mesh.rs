use camera::Camera;
use graphics::Graphics;
use graphics::default_draw_state;
use graphics::math::Matrix2d;
use graphics;
use lights::LightSource;
use math::mat3xv3_mul;
use math::mat_rotation;
use math::vec3_rotate_around;
use rand;
use std;
use std::cell::RefCell;
use std::rc::Rc;
use types::Color;
use types::Vec3;
use vecmath::vec3_add;
use vecmath::vec3_cross;
use vecmath::vec3_dot;
use vecmath::vec3_normalized;
use vecmath::vec3_square_len;
use vecmath::vec3_sub;

pub use float::One;
pub use float::Zero;

/// A 3D mesh
#[derive(Debug,Clone)]
pub struct Mesh {
    mesh: Rc<MeshContents>,
    r: Vec3,
    wireframe: bool,
    theta: Vec3,
}

/// Contents of a 3D mesh
#[derive(Debug,Clone)]
pub struct MeshContents {
    pub faces: RefCell<Vec<Face>>,
    pub vertices: RefCell<Vec<Vertex>>,
}

/// A 3D Face
#[derive(Debug,Clone)]
pub struct Face {
    vertices: [usize; 3],
    color: Color,
    mesh: Rc<MeshContents>,
}

/// A 3D vertex in a mesh
#[derive(Debug,Clone)]
pub struct Vertex {
    r: Vec3,
    faces: Vec<usize>,
    mesh: Rc<MeshContents>,
}

/// Cast light and shadows on faces
#[derive(Debug,Clone)]
pub struct Light {
    r: Vec3,
    intensity: f32,
}

// ======================================================================
// Light

impl Light {
    fn new(r: Vec3, intensity: f32) -> Light {
        Light { r: r, intensity: intensity }
    }
}

// ======================================================================
// Face

impl Face {
    fn new(mesh: Rc<MeshContents>, a: usize, b: usize, c: usize) -> Face {
        Face {
            vertices: [a, b, c],
            color: [0.5; 4],
            mesh: mesh,
        }
    }

    pub fn color(mut self, color: Color) -> Face {
        self.color = color;
        self
    }

    pub fn project(&self, camera: &Camera) -> graphics::types::Triangle {
        let points = self.get_points();
        [
            camera.projected(points[0]),
            camera.projected(points[1]),
            camera.projected(points[2]),
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

    fn get_points(&self) -> [Vec3; 3] {
        [
            self.mesh.vertices.borrow()[self.vertices[0]].r,
            self.mesh.vertices.borrow()[self.vertices[1]].r,
            self.mesh.vertices.borrow()[self.vertices[2]].r,
        ]
    }

    pub fn normal(&self) -> Vec3 {
        let points = self.get_points();
        vec3_normalized(vec3_cross(points[1], points[2]))
    }

    pub fn shade(&self, lights: &Vec<LightSource>) -> Color {
        let norm = self.normal();
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
// Mesh

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            r: [0.0; 3],
            theta: [0.0; 3],
            mesh: Rc::new(MeshContents{
                vertices: RefCell::new(vec![]),
                faces: RefCell::new(vec![]),
            }),
            wireframe: false,
        }
    }

    fn add_vertex(&self, r: Vec3) -> usize {
        self.mesh.vertices.borrow_mut().push(
            Vertex {
                faces: vec![],
                r: r,
                mesh: self.mesh.clone(),
            }
        );
        self.mesh.vertices.borrow().len() - 1
    }

    pub fn add_face(&self, face: Face) -> usize {
        self.mesh.faces.borrow_mut().push(face);
        self.mesh.faces.borrow().len() - 1
    }

    pub fn translate(&mut self, r: Vec3) {
        self.r = vec3_add(r, self.r);
        for vertex in self.mesh.vertices.borrow_mut().iter_mut() {
            vertex.r = vec3_add(r, vertex.r);
        }
    }

    pub fn rotate(&mut self, theta: Vec3) {
        self.theta = vec3_add(self.theta, theta);
        let rotation = mat_rotation(theta);
        for vertex in self.mesh.vertices.borrow_mut().iter_mut() {
            vertex.r = vec3_rotate_around(vertex.r, rotation, self.r);
        }
    }

    pub fn wireframe(&mut self, wireframe: bool) {
        self.wireframe = wireframe;
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
        for face in self.mesh.faces.borrow().iter() {
            graphics::Polygon::new(face.shade(lights))
                .draw(&face.project(camera),
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
        for face in self.mesh.faces.borrow().iter() {
            for line in &face.project_lines(camera) {
                graphics::Line::new(face.color, 0.5)
                    .draw(*line, ds, transform, g)
            }
        }
    }

    pub fn position(mut self, r: Vec3) -> Mesh {
        self.r = r;
        self
    }

    pub fn add_terrain(&self, size: f64, res: f64) {
        self.add_vertex([0.0; 3]);
        let n = (size / res) as usize;
        for j in 0..n {
            for i in 0..n {
                let x = i as f64 * res - size / 2.0;
                let z = j as f64 * res - size / 2.0;
                let y = ((x/100.0).sin() + (z/100.0).cos())*100.0;
                self.add_vertex([x,  y,  z]);
            }
        }
        let color = [0.0, 0.25, 0., 1.0];
        for i in 0..n-1 {
            for j in 0..n-1 {
                let a = j * n + i;
                let b = j * n + i + 1;
                let c = (j + 1) * n + i;
                let d = (j + 1) * n + i + 1;
                self.add_face(Face::new(self.mesh.clone(), a, b, c).color(color));
                self.add_face(Face::new(self.mesh.clone(), d, b, c).color(color));
            }
        }
    }

    pub fn new_diamond(size: f64) -> Mesh {
        let mut mesh = Mesh::new();
        // let a = mesh.add_vertex(Vertex::new([-size,  0.0,  0.0]));
        // let b = mesh.add_vertex(Vertex::new([  0.0,  0.0,  size]));
        // let c = mesh.add_vertex(Vertex::new([ size,  0.0,  0.0]));
        // let d = mesh.add_vertex(Vertex::new([  0.0,  0.0, -size]));
        // let e = mesh.add_vertex(Vertex::new([  0.0,  size*2.0, 0.0]));
        // let f = mesh.add_vertex(Vertex::new([  0.0, -size*2.0, 0.0]));
        // let color = [0.1, 0.1, 0.9, 0.4];
        // mesh.add_face(Face::new(a, e, b).color(color));
        // mesh.add_face(Face::new(b, e, c).color(color));
        // mesh.add_face(Face::new(c, e, d).color(color));
        // mesh.add_face(Face::new(d, e, a).color(color));
        // mesh.add_face(Face::new(a, f, b).color(color));
        // mesh.add_face(Face::new(b, f, c).color(color));
        // mesh.add_face(Face::new(c, f, d).color(color));
        // mesh.add_face(Face::new(d, f, a).color(color));
        mesh
    }

}
