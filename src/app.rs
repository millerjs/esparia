//! The app to handle events

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use cam::Camera;

use opengl_graphics::{
    GlGraphics,
    OpenGL,
};

use engine::{
    Face,
    GameObject,
    World,
};

pub struct Game {
    gl: GlGraphics,
    window: Window,
    world: World,
    camera: Camera<f64>,
}


impl Game {

    pub fn new() -> Game {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window: Window = WindowSettings::new("Esparia", [800, 800])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Game {
            gl: GlGraphics::new(opengl),
            window: window,
            world: World::new(),
            camera: Camera::<f64>::new([0.0, 0.0, 0.0]),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [0.8, 0.8, 0.8, 1.0];

        let obj1 = GameObject::new()
            .face(Face::new([0.0, 0.0, 0.0],
                            [1.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0]));

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);

            let square = rectangle::square(0.0, 0.0, 50.0);
            let transform = c.transform.trans(10.0, 10.0);

            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }

    pub fn run(mut self) {
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
