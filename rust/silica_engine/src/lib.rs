mod api;
mod particle;
mod variant;
mod variant_type;
mod world;

use particle::*;
use variant::*;
use variant_type::*;
use world::*;

use godot::{engine::Image, prelude::*};
struct SilicaExtension;
#[gdextension]
unsafe impl ExtensionLibrary for SilicaExtension {}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct GDWorld {
    world: World,
    #[base]
    node2d: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for GDWorld {
    fn init(node2d: Base<Node2D>) -> Self {
        let world = World::new(256, 256);
        let image = Image::new();

        GDWorld { world, node2d }
    }

    fn physics_process(&mut self, delta: f64) {
        if self.world.running {
            self.world.tick();
        }

        // create image and edit pixels based on world.world.particle value
    }
}
