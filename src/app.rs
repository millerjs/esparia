//! The app to handle events

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}


impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED:   [f32; 4] = [0.8, 0.8, 0.8, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
            let square = rectangle::square(0.0, 0.0, 50.0);
            let transform = c.transform.trans(10.0, 10.0);
            rectangle(RED, square, transform, gl);
        });

    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}


pub struct Game {
    window: Window,
    app: App,
}


impl Game {

    pub fn new() -> Game {
        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        // Create an Glutin window.
        let window: Window = WindowSettings::new(
            "Esparia",
            [200, 200]
        )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        // Create a new game app
        let app = App {
            gl: GlGraphics::new(opengl),
        };

        Game { window: window, app: app }

    }


    pub fn run(mut self) {
        let mut events = self.window.events();

        // Listen for events
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args() {
                self.app.render(&r);
            }

            if let Some(u) = e.update_args() {
                self.app.update(&u);
            }
        }

    }

}
