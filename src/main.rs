use crate::sim::nodes::and::AndGate;
use crate::sim::nodes::io::Input;
use crate::sim::nodes::not::NotGate;
use crate::sim::nodes::or::OrGate;
use crate::sim::nodes::{Node, NodeComponent};
use bevy::{color::palettes::basic::WHITE, prelude::*};
use util::input::{InputPlugin, Mouse};

pub mod sim;
pub mod util;

fn main() {
    // App::new()
    //     .add_plugins((DefaultPlugins, InputPlugin))
    //     .add_systems(Startup, setup)
    //     .add_systems(Update, draw_cursor)
    //     .run();

    let switch_a: NodeComponent = Box::new(Input { state: false });
    let switch_b: NodeComponent = Box::new(Input { state: false });
    let switch_c: NodeComponent = Box::new(Input { state: true });

    let or: NodeComponent = Box::new(OrGate::new(switch_a, switch_b));

    let lamp = AndGate::new(or, switch_c).simulate();

    let not = NotGate::new(None).simulate();

    println!("AND(OR(A, B), C) -> {}", lamp);
    println!("NOT -> {}", not);
}

fn draw_cursor(mut gizmos: Gizmos, mouse: Res<Mouse>) {
    gizmos.circle_2d(mouse.pos, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
