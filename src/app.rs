
use mesh::Mesh;
use world::WorldObject;
use world::World;

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Game {
        let mut terrain = Mesh::new();
        terrain.wireframe(false);
        terrain.add_terrain(600.0, 20.0);

        let mut diamond = Mesh::new_diamond(15.0);
        diamond.wireframe(false);

        let world = World::new()
            .object(WorldObject::new().mesh(terrain))
            .object(WorldObject::new().mesh(diamond));

        Game { world: world }
    }

    pub fn run(self) {
        self.world.run()
    }
}
