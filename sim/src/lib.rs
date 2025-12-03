use godot::prelude::*;

mod sim;
mod util;
mod physics;
mod state;

struct SimExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SimExtension {}
