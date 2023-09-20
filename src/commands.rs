#![allow(dead_code)]
use crate::resources::Resource;
use crate::world::World;

type WorldFn = dyn FnMut(&mut World);

#[derive(Default)]
pub struct CommandQueue {
    commands: Vec<Box<WorldFn>>,
}

impl Resource for CommandQueue {}

impl CommandQueue {
    pub fn add_command(&mut self, command: impl FnMut(&mut World) + 'static) {
        self.commands.push(Box::new(command));
    }

    pub fn run_commands(&mut self, world: &mut World) {
        for command in self.commands.iter_mut() {
            command(world);
        }
    }
}
