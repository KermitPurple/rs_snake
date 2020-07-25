extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    scale: i32,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);
        });
        self.snake.render(&mut self.gl, arg, &self.scale);
    }

    fn update(&mut self){
        self.snake.update(&self.scale);
    }
}

struct Snake {
    position: Vector2,
    direction: Direction
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, scale: &i32){
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square = graphics::rectangle::square(
            self.position.x as f64,
            self.position.y as f64,
            *scale as f64,
            );
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(GREEN, square, transform, gl);
        });
    }
    
    fn update(&mut self, scale: &i32){
        match self.direction {
            Direction::Right => self.position.x += scale,
            Direction::Left => self.position.x -= scale,
            Direction::Up => self.position.y -= scale,
            Direction::Down => self.position.y += scale,
            _ => unreachable!(),
        }
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
            },
            direction: Direction::Up
        },
        scale: 10
    };

    let mut settings = EventSettings::new();
    settings.ups = 8;
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(u) = e.update_args() {
            game.update();
        }
    }
}
