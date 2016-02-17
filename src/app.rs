use world::{ WorldObject, World };
use mesh::{ Mesh };

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Game {
        let terrain_mesh = Mesh::new_domain().wireframe(false);
        let terrain = WorldObject::new().mesh(terrain_mesh);
        let world = World::new().object(terrain);

        Game { world: world }
    }

    pub fn run(self) {
        self.world.run()
    }
}
