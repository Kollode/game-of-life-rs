extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod gameboard;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("GameOfLive", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut event_loop_settings = EventSettings::new();
    event_loop_settings.set_ups(3);
    let mut events = Events::new(event_loop_settings);
    let mut gl = GlGraphics::new(opengl);

    let gameboard_settings = gameboard::GameboardSettings::new();
    let mut gameboard = gameboard::Gameboard::new(gameboard_settings);

    while let Some(e) = events.next(&mut window) {
        if let Some(update_args) = e.update_args() {
            gameboard.update(&update_args);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard.draw(&c, g);
            });
        }
    }
}
