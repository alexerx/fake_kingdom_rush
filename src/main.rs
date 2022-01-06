mod plugins;

use bevy::prelude::*;

use crate::plugins::hello_world::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
