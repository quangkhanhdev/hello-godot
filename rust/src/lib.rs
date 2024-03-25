use godot::prelude::*;
mod player;
struct HelloGodot;

#[gdextension]
unsafe impl ExtensionLibrary for HelloGodot {
}