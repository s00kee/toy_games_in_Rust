// TODO: implement outta bounds check and food
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone)]
enum Direction{
    Left,
    Right,
    Up,
    Down
}

struct Snake{
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| rectangle::square(x as f64, y as f64, 50.0))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.
                into_iter()
                .for_each(|square| rectangle(RED, square, transform, gl));
        });
    }

    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake is empty"))
                                .clone();
        match self.dir{
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 += 1,
            Direction::Down => new_head.1 -= 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back();
    }

}

struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });

        self.snake.render(args, &mut self.gl);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn keyPressed(&mut self, button: &Button){
        self.snake.dir = match button {
            Button::Keyboard(Key::Up) => Direction::Up,
            Button::Keyboard(Key::Down) => Direction::Down,
            Button::Keyboard(Key::Left) => Direction::Left,
            Button::Keyboard(Key::Right) => Direction::Right,
            _ => self.snake.dir.clone(),
        };
    }
}

fn main(){
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Board", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        snake: Snake {
                    body: LinkedList::from_iter((vec![(0, 0)]).into_iter()),
                    dir: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window){
        if let Some(r) = e.render_args(){
            game.render(&r);
        }

        if let Some(u) = e.update_args(){
            game.update();
        }

        if let Some(b) = e.button_args(){
            if b.state == ButtonState::Press{
                game.keyPressed(&b.button);
            }
        }
    }
}
