

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

        let terrain = Mesh::new_terrain()
            .translate([0.0, 0.0, 0.0]);

        // let mesh1 = Mesh::new()
        //     .face(Face::new([ 100.0, 0.0, 0.0],
        //                     [-100.0, 0.0, 0.0],
        //                     [  0.0, 100.0, 0.0])
        //           .color([1.0, 1.0, 1.0, 0.4]))
        //     .position([0.0, 0.0, 0.0])
        //     .wireframe(false);

        let world = World::new()
            // .object(WorldObject::new().mesh(mesh1));
            .object(WorldObject::new().mesh(terrain));

        Game { world: world }

    }

    pub fn run(self) {
        self.world.run()
    }
}
