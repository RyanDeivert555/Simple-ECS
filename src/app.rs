use crate::commands::CommandQueue;
use crate::scheduler::Scheduler;
use crate::systems::{StartUpSystem, System};
use crate::world::World;

#[derive(Default)]
pub struct App {
    world: World,
    scheduler: Scheduler,
    run: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            run: true,
            ..Default::default()
        }
    }

    pub fn add_startup_system(&mut self, system: impl StartUpSystem + 'static) -> &mut Self {
        self.scheduler.add_startup_system(system);
        self
    }

    pub fn add_system(&mut self, system: impl System + 'static) -> &mut Self {
        self.scheduler.add_system(system);
        self
    }

    pub fn run(&mut self) {
        self.world.add_resource(CommandQueue::default());
        self.run = self.scheduler.run_startup_systems(&mut self.world);
        while self.run {
            self.run = self.scheduler.run_systems(&mut self.world);
            self.world.run_commands();
        }
    }
}
