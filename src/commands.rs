use crate::world::World;

type WorldFn = dyn FnMut(&mut World);

#[derive(Default)]
pub struct CommandQueue {
    commands: Vec<Box<WorldFn>>,
}

impl CommandQueue {
    pub fn add(&mut self, command: impl FnMut(&mut World) + 'static) {
        self.commands.push(Box::new(command));
    }

    pub fn run_commands(&mut self, world: &mut World) {
        for mut command in self.commands.drain(..) {
            command(world);
        }
    }
}