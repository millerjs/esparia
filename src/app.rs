use engine::{
    Mesh,
    Face,
    WorldObject,
    World,
};

pub struct Game {
    world: World,
}



impl Game {

    pub fn new() -> Game {

        let terrain = Mesh::new_domain()
            .wireframe(true);

        let world = World::new()
            .object(WorldObject::new().mesh(terrain));

        Game { world: world }

    }

    pub fn run(self) {
        self.world.run()
    }
}
