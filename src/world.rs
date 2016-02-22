use std::ops::{ Add, Mul, Neg, Sub, Div };

pub use float::{
    One,
    Zero,
};

use vecmath;

use types::{
    Vec3,
};

use graphics::Graphics;

use graphics::math::{
    Matrix2d,
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

use camera;
use camera::Camera;
use mesh::{ Mesh, Face };


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
        let camera = &mut self.camera;

        camera.width = args.width as f64;
        camera.height = args.height as f64;

        camera.update_projection();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            for object in objects {
                object.draw(camera, c.transform, gl);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.t += args.dt;
        self.objects[1].meshes[0].translate([self.t.cos()*10.0, 0.0, self.t.sin()*10.0])
    }

    fn move_diamond(&mut self, key: &String) {
        // let s = 6.26573;
        // println!("{:?}", args);
        // let (w, h) = (self.camera.width, self.camera.height);

        let d = 10.0;
        let r = match key.as_str() {
            "a" => [ -d, 0.0, 0.0],
            "d" => [  d, 0.0, 0.0],
            "s" => [0.0, 0.0,  -d],
            "w" => [0.0, 0.0,   d],
            _ => [0.0; 3]
        };
        self.objects[1].meshes[0].translate(r);

    }

    fn move_camera(&mut self, key: &String) {
        let d = 10.0;
        let r = match key.as_str() {
            "a" => [ -d, 0.0, 0.0],
            "d" => [  d, 0.0, 0.0],
            "s" => [0.0, 0.0,  -d],
            "w" => [0.0, 0.0,   d],
            "2" => [0.0, d,  0.0],
            "x" => [0.0, -d, 0.0],
            _ => [0.0; 3]
        };
        self.camera.r = vecmath::vec3_add(self.camera.r, r);
    }

    fn turn_camera(&mut self, pos: [f64; 2]) {
        self.camera.theta[1] = (pos[0]-self.camera.width/2.0) / self.camera.width*6.0;
        self.camera.theta[0] = -(pos[1]-self.camera.height/2.0) /  self.camera.height*2.0;
    }

    pub fn run(mut self) {
        println!("Running world...");

        let mut events = self.window.events();

        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.render(&r);
            }

            else if let Some(u) = e.update_args() {
                self.update(&u);
            }

            if let Some(c) = e.mouse_cursor_args() {
                self.turn_camera(c);
            }

            else if let Some(c) = e.text_args() {
                self.move_camera(&c);
            }

            // else {
            //     println!("{:?}", e);
            // }
        }
    }

}
