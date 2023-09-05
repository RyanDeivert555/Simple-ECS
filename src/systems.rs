use crate::operations::OperationStack;
use crate::world::World;

pub trait System {
    fn run(&self, world: &World, operation_stack: &mut OperationStack);
}
