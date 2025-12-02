use godot::prelude::*;

struct SimExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SimExtension {}
