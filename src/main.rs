extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);
        });
        self.snake.render(&mut self.gl, arg);
    }
}

struct Snake {
    position: Vector2
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square = graphics::rectangle::square(
            self.position.x as f64,
            self.position.y as f64,
            10_f64
            );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(GREEN, square, transform, gl);
        });
    }
}

struct Vector2 {
    x: i32,
    y: i32
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "RsSnake",
        [400, 400]
        ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake {
            position: Vector2 {
                x: 50,
                y: 100
            }
        }
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
    }
}
