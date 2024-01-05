use bevy::prelude::*;

mod window;

fn main() {
    App::new().add_plugins(window::WindowStuffPlugin).run();
}
