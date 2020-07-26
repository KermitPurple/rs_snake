extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent, ButtonEvent, ButtonArgs};
use piston::window::WindowSettings;
use piston::Button;
use piston::Key;
use rand::prelude::*;

enum Direction {
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    scale: i32,
    size: [u32; 2],
    fruit: Fruit,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);
        });
        self.fruit.render(&mut self.gl, arg, &self.scale);
        self.snake.render(&mut self.gl, arg, &self.scale);
    }

    fn update(&mut self){
        self.snake.update(&self.scale, self.size);
        if self.snake.position == self.fruit.position {
            self.fruit = Fruit::random(self.scale, self.size);
        }
    }

    fn button_pressed(&mut self, b: &ButtonArgs){
        self.snake.change_direction(&b);
    }
}

struct Snake {
    position: Point,
    direction: Direction
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, scale: &i32){
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        draw_block(&self.position, gl, args, scale, GREEN);
    }

    fn update(&mut self, scale: &i32, size: [u32; 2]){
        match self.direction {
            Direction::Right => self.position.x += scale,
            Direction::Left => self.position.x -= scale,
            Direction::Up => self.position.y -= scale,
            Direction::Down => self.position.y += scale,
            _ => unreachable!(),
        }
        if self.position.x < 0 || self.position.x >= size[0] as i32 || self.position.y < 0 || self.position.y >= size[1] as i32 {
            println!("Dead");
        }
    }

    fn change_direction(&mut self, b: &ButtonArgs){
        match b.button {
            Button::Keyboard(Key::D)|Button::Keyboard(Key::Right) => self.direction = Direction::Right,
            Button::Keyboard(Key::A)|Button::Keyboard(Key::Left) => self.direction = Direction::Left,
            Button::Keyboard(Key::W)|Button::Keyboard(Key::Up) => self.direction = Direction::Up,
            Button::Keyboard(Key::S)|Button::Keyboard(Key::Down) => self.direction = Direction::Down,
            _ => ()
        }
    }
}

struct Point {
    x: i32,
    y: i32
}

struct Fruit {
    position: Point,
}

impl Fruit {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, scale: &i32){
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        draw_block(&self.position, gl, args, scale, RED);
    }
    fn random(scale: i32,size: [u32; 2]) -> Fruit{
        let mut rng = thread_rng();
        Fruit{
            position: Point{
                x: rng.gen_range(0, size[0] as i32 / scale) * scale,
                y: rng.gen_range(0, size[1] as i32 / scale) * scale,
            }
        }
    }
}

impl std::cmp::PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    const SIZE: [u32; 2] = [400, 400];
    const SCALE: i32 = 10;

    let mut window: Window = WindowSettings::new(
        "RsSnake",
        SIZE,
        ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake {
            position: Point {
                x: 50,
                y: 100
            },
            direction: Direction::Up
        },
        scale: SCALE,
        size: SIZE,
        fruit: Fruit::random(SCALE, SIZE),
    };

    let mut settings = EventSettings::new();
    settings.ups = 8;
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(_u) = e.update_args() {
            game.update();
        }
        if let Some(b) = e.button_args(){
            game.button_pressed(&b);
        }
    }
}

fn draw_block(position: &Point, gl: &mut GlGraphics, args: &RenderArgs, scale: &i32, color: [f32; 4]){
    let square = graphics::rectangle::square(
        position.x as f64,
        position.y as f64,
        *scale as f64,
        );
    gl.draw(args.viewport(), |c, gl| {
        let transform = c.transform;
        graphics::rectangle(color, square, transform, gl);
    });
}
