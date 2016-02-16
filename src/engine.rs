//! Because we're using a 2D graphics library, let's define a few 3D
//! things here.

use std::ops::{
    Add,
    Mul,
};

pub use float::{
    One,
    Zero,
};

use vecmath;

use vecmath::{
    Vector2,
    Vector3,
    Vector4,
    Matrix4,
    vec3_add,
};

use types::{
    Vec2,
    Vec3,
    Vec4,
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

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;

use opengl_graphics::{
    GlGraphics,
    OpenGL,
};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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
            let p = face.project(camera);
            let lines = [
                [p[0][0], p[0][1], p[1][0], p[1][1]],
                [p[1][0], p[1][1], p[2][0], p[2][1]],
                [p[2][0], p[2][1], p[0][0], p[0][1]],
            ];
            for line in &lines {
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
        let size = 100.0;
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
        let size = 100.0;
        let a = [-size,  size, 0.0];
        let b = [ size,  size, 0.0];
        let c = [-size, -size, 0.0];
        let d = [ size, -size, 0.0];
        let aa = [-size,  size, size];
        let bb = [ size,  size, size];
        let cc = [-size, -size, size];
        let dd = [ size, -size, size];
        Mesh::new()
            .face(Face::new(a, b, d))
            .face(Face::new(a, c, d))
            .face(Face::new(aa, bb, dd).color([0.5, 0.5, 1.0, 1.0]))
            .face(Face::new(aa, cc, dd).color([0.5, 0.5, 1.0, 1.0]))
            .wireframe(true)
    }


}


// ======================================================================
// Game Objects

#[derive(Debug)]
pub struct WorldObject {
    pub meshes: Vec<Mesh>,
    pub r: Vec3,
}

impl WorldObject {
    pub fn new() -> WorldObject {
        WorldObject { meshes: vec![], r: [0.0; 3] }
    }

    pub fn mesh(mut self, mesh: Mesh) -> WorldObject {
        self.meshes.push(mesh);
        self
    }

    pub fn draw<G>(&self, camera: &Camera, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        for mesh in &self.meshes {
            mesh.draw(camera, transform, g)
        }
    }
}


// ======================================================================
// World

pub struct World {
    pub objects: Vec<WorldObject>,
    pub gl: GlGraphics,
    pub window: Window,
    pub t: f64,
    pub camera: Camera,
}


impl World {
    pub fn new() -> World {
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window: Window = WindowSettings::new("Esparia", [800, 800])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        World {
            gl: GlGraphics::new(opengl),
            objects: vec![],
            t: 0.0,
            window: window,
            camera: Camera::default(),
        }

    }

    pub fn object(mut self, object: WorldObject) -> World {
        self.objects.push(object);
        self
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::clear;
        use graphics::Transformed;

        let objects = &self.objects;
        let camera = &self.camera;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for object in objects {
                object.draw(camera, [[0.0; 3]; 2], gl);
            }

        });

    }

    fn update(&mut self, args: &UpdateArgs) {
        self.t += args.dt;
    }

    pub fn run(mut self) {
        println!("Running world...");

        let mut events = self.window.events();

        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.update(&u);
            }
        }
    }

}
