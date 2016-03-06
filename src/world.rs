//! Module managing a world of mesh and actor objects

use std::cell::RefCell;

pub use float::One;
pub use float::Zero;
use camera::Camera;
use glutin_window::GlutinWindow as Window;
use lights::LightSource;
use mesh::Mesh;
use mesh::Face;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::cmp::Ordering;
use types::Vec3;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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
}


pub struct World {
    pub objects: Vec<WorldObject>,
    pub gl: GlGraphics,
    pub window: Window,
    pub t: f64,
    pub camera: Camera,
    pub lights: Vec<LightSource>,
    pub triangles: RefCell<Vec<DepthTriangle>>,
}


pub struct DepthTriangle {
    face: Face,
    dist: f64,
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

        let light = LightSource::new([200.0, 100.0, 0.0]);

        World {
            gl: GlGraphics::new(opengl),
            objects: vec![],
            t: 0.0,
            window: window,
            camera: Camera::default(),
            lights: vec![light],
            triangles: RefCell::new(vec![]),
        }

    }

    pub fn object(mut self, object: WorldObject) -> World {
        self.objects.push(object);
        self
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::clear;
        use graphics::Transformed;

        let lights = &self.lights;
        let objects = &self.objects;
        let camera = &mut self.camera;
        let triangles = &mut self.triangles;

        camera.width = args.width as f64;
        camera.height = args.height as f64;
        camera.update_projection();


        // let mut n_triangles = 0;
        // Count all of the triangles in the whole world
        // for object in &self.objects {
        //     for mesh in object.meshes.iter() {
        //         n_triangles += mesh.mesh.faces.borrow().len();
        //     }
        // }

        // if triangles.borrow().len() != n_triangles {
        // Get all of the triangles in the whole world
        triangles.borrow_mut().clear();
        for object in &self.objects {
            for mesh in object.meshes.iter() {
                for face in mesh.mesh.faces.borrow_mut().iter() {
                    let d = face.distance(camera.r);
                    triangles.borrow_mut().push(DepthTriangle {
                        face: face.clone(),
                        dist: d,
                    });
                    // println!("{:}", d);
                }
            }
        }
        // }

        // Sort those triangles based on distance from the camera

        triangles.borrow_mut().sort_by(
            |a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Less)
        );

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            // // Get all of the triangles in the whole world
            // for object in objects.iter() {
            //     for mesh in object.meshes.iter() {
            //         for face in mesh.mesh.faces.borrow_mut().iter() {
            //             if face.distance(camera.r) < 300.0 {
            //                 face.draw_filled(
            //                     camera,
            //                     lights,
            //                     c.transform,
            //                     gl,
            //                 )
            //             }
            //         }
            //     }
            // }
            //

            for triangle in triangles.borrow().iter() {
                if triangle.dist < 600.0 {
                    triangle.face.draw_filled(
                        camera,
                        lights,
                        c.transform,
                        gl,
                    );
                }
            }


        });



    }

    fn update(&mut self, args: &UpdateArgs) {
        self.t += args.dt;
        self.objects[1].meshes[0].translate([self.t.cos(), 0.0, self.t.sin()])
    }

    fn move_diamond(&mut self, key: &String) {
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
            "x" => [0.0, d,  0.0],
            "2" => [0.0, -d, 0.0],
            _ => [0.0; 3]
        };
        self.camera.translate(r);
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
        }
    }

}
