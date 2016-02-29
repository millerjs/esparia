

use world::{ WorldObject, World };
use mesh::{ Mesh };

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Game {
        let terrain = Mesh::new_terrain(600.0, 20.0).wireframe(false);
        let diamond = Mesh::new_diamond(15.0).wireframe(false);
        let world = World::new()
            .object(WorldObject::new().mesh(terrain))
            .object(WorldObject::new().mesh(diamond));

        Game { world: world }
    }

    pub fn run(self) {
        self.world.run()
    }
}
