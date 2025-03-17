use bevy::{color::palettes::basic::WHITE, prelude::*};
use util::input::{InputPlugin, Mouse};

pub mod util;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

fn draw_cursor(mut gizmos: Gizmos, mouse: Res<Mouse>) {
    gizmos.circle_2d(mouse.pos, 10., WHITE);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
