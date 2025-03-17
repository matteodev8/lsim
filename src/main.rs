use bevy::{color::palettes::basic::WHITE, prelude::*};
use sim::gates::Gate;
use util::input::{InputPlugin, Mouse};

pub mod sim;
pub mod util;

fn main() {
    // App::new()
    //     .add_plugins((DefaultPlugins, InputPlugin))
    //     .add_systems(Startup, setup)
    //     .add_systems(Update, draw_cursor)
    //     .run();

    let switch_a = Box::new(sim::gates::Input { state: true });
    let switch_b = Box::new(sim::gates::Input { state: false });
    let lamp = sim::gates::OrGate::new(switch_a, switch_b).simulate();

    println!("{}", lamp)
}

fn draw_cursor(mut gizmos: Gizmos, mouse: Res<Mouse>) {
    gizmos.circle_2d(mouse.pos, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
