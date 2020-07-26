extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent, ButtonEvent, ButtonArgs};
use piston::window::WindowSettings;
use piston::Button;
use piston::Key;
use crate::piston::Window;
use rand::prelude::*;

#[derive(PartialEq)]
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
        if self.snake.alive{
            self.gl.draw(arg.viewport(), |_c, gl| {
                graphics::clear(BLACK, gl);
            });
            self.fruit.render(&mut self.gl, arg, &self.scale);
            self.snake.render(&mut self.gl, arg, &self.scale);
        } else {
        }
    }

    fn update(&mut self){
        self.snake.update(&self.scale, self.size);
        if self.snake.position == self.fruit.position {
            self.fruit = Fruit::random(self.scale, self.size);
            self.snake.length_to_grow += 2;
        }
    }

    fn button_pressed(&mut self, b: &ButtonArgs, w: &mut GlutinWindow){
        if self.snake.alive {
            self.snake.change_direction(&b);
        } else {
            if let Some(c) = self.continue_y_n(&b){
                if c {
                    self.snake = Snake::new();
                    self.fruit = Fruit::random(self.scale, self.size);
                } else {
                    w.set_should_close(true);
                }
            }
        }
    }

    fn continue_y_n(&mut self, b: &ButtonArgs) -> Option<bool>{
        match b.button {
            Button::Keyboard(Key::Y) => return Some(true),
            Button::Keyboard(Key::N) => return Some(false),
            _ => return None,
        }
    }
}

struct Snake {
    position: Point,
    direction: Direction,
    length: u32,
    length_to_grow: u32,
    tail: Vec<Point>,
    alive: bool,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, scale: &i32){
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        draw_block(&self.position, gl, args, scale, GREEN);
        self.render_tail(gl, args, scale);
    }

    fn update(&mut self, scale: &i32, size: [u32; 2]){
        self.update_tail();
        match self.direction {
            Direction::Right => self.position.x += scale,
            Direction::Left => self.position.x -= scale,
            Direction::Up => self.position.y -= scale,
            Direction::Down => self.position.y += scale,
        }
        if self.tail.contains(&self.position) || self.position.x < 0 || self.position.x >= size[0] as i32 || self.position.y < 0 || self.position.y >= size[1] as i32 {
            self.alive = false;
        }
    }

    fn change_direction(&mut self, b: &ButtonArgs){
        match b.button {
            Button::Keyboard(Key::D)|Button::Keyboard(Key::Right) => {
                if self.direction != Direction::Left {
                    self.direction = Direction::Right
                }
            },
            Button::Keyboard(Key::A)|Button::Keyboard(Key::Left) => {
                if self.direction != Direction::Right {
                    self.direction = Direction::Left
                }
            },
            Button::Keyboard(Key::W)|Button::Keyboard(Key::Up) => {
                if self.direction != Direction::Down {
                    self.direction = Direction::Up
                }
            },
            Button::Keyboard(Key::S)|Button::Keyboard(Key::Down) => {
                if self.direction != Direction::Up {
                    self.direction = Direction::Down
                }
            },
            _ => ()
        }
    }

    fn update_tail(&mut self){
        self.tail.insert(0, self.position);
        if self.length_to_grow > 0 {
            self.length_to_grow -= 1;
            self.length += 1;
        } else {
            self.tail.pop();
        }
    }

    fn render_tail(&mut self, gl: &mut GlGraphics, args: &RenderArgs, scale: &i32){
        const DARK_GREEN: [f32; 4] = [0.0, 0.7, 0.0, 1.0];
        for p in &self.tail{
            draw_block(&p, gl, args, scale, DARK_GREEN);
        }
    }

    fn new() -> Snake{
        Snake{
            position: Point {
                x: 200,
                y: 200,
            },
            direction: Direction::Up,
            length: 0,
            length_to_grow: 0,
            tail: vec![],
            alive: true,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
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

fn main() {
    let opengl = OpenGL::V3_2;

    const SIZE: [u32; 2] = [400, 400];
    const SCALE: i32 = 10;

    let mut window: GlutinWindow = WindowSettings::new(
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
            direction: Direction::Up,
            length: 0,
            length_to_grow: 0,
            tail: vec![],
            alive: true,
        },
        scale: SCALE,
        size: SIZE,
        fruit: Fruit::random(SCALE, SIZE),
    };

    let mut settings = EventSettings::new();
    settings.ups = 10;
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(_u) = e.update_args() {
            game.update();
        }
        if let Some(b) = e.button_args(){
            game.button_pressed(&b, &mut window);
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
