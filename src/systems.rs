use crate::world::World;

pub trait StartUpSystem {
    fn run(&mut self, world: &mut World) -> bool;
}

pub trait System {
    fn run(&mut self, world: &mut World) -> bool;
}
