use crate::systems::{System, StartUpSystem};
use crate::world::World;

#[derive(Default)]
pub struct Scheduler {
    startup_systems: Vec<Box<dyn StartUpSystem>>,
    systems: Vec<Box<dyn System>>,
}

impl Scheduler {
    pub fn run_startup_systems(&mut self, world: &mut World) -> bool {
        //self.startup_systems.iter_mut().any(|system| system.run(world))
        for system in self.startup_systems.iter_mut() {
            if !system.run(world) {
                return false;
            }
        }
        true
    }

    pub fn run_systems(&mut self, world: &mut World) -> bool{
        //self.systems.iter_mut().any(|system| system.run(world))
        for system in self.systems.iter_mut() {
            if !system.run(world) {
                return false;
            }
        }
        true
    }

    pub fn add_startup_system(&mut self, system: impl StartUpSystem + 'static) {
        self.startup_systems.push(Box::new(system));
    }

    pub fn add_system(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }
}