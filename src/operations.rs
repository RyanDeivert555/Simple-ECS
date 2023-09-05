#![allow(dead_code)]
use crate::components::Component;
use crate::resources::Resource;
use crate::world::{EntityId, World};
use std::marker::PhantomData;

pub trait Operation {
    fn do_operation(self: Box<Self>, _world: &mut World);
}

pub struct NewEntity;
impl Operation for NewEntity {
    fn do_operation(self: Box<Self>, world: &mut World) {
        let _ = world.new_entity();
    }
}

pub struct RemoveEntity(EntityId);
impl Operation for RemoveEntity {
    fn do_operation(self: Box<Self>, world: &mut World) {
        world.remove_entity(self.0);
    }
}

pub struct AddComponent<T: Component + 'static>(EntityId, T);
impl<T: Component + 'static> Operation for AddComponent<T> {
    fn do_operation(self: Box<Self>, world: &mut World) {
        world.add_component(self.0, self.1);
    }
}

pub struct RemoveComponent<T: Component + 'static> {
    entity: EntityId,
    _marker: PhantomData<T>,
}
impl<T: Component + 'static> Operation for RemoveComponent<T> {
    fn do_operation(self: Box<Self>, world: &mut World) {
        world.remove_component::<T>(self.entity);
    }
}

pub struct StopRun;
impl Operation for StopRun {
    fn do_operation(self: Box<Self>, world: &mut World) {
        world.stop_run();
    }
}

pub struct AddResource<T: Resource + 'static>(pub T);
impl<T: Resource + 'static> Operation for AddResource<T> {
    fn do_operation(self: Box<Self>, world: &mut World) {
        world.add_resource(self.0);
    }
}

pub struct RemoveResource<T: Resource + 'static>(T);
impl<T: Resource + 'static> Operation for RemoveResource<T> {
    fn do_operation(self: Box<Self>, world: &mut World) {
        world.remove_resource::<T>();
    }
}

#[derive(Default)]
pub struct OperationStack {
    operations: Vec<Box<dyn Operation>>,
}

impl OperationStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_operations(&mut self, world: &mut World) {
        for operation in self.operations.drain(..) {
            operation.do_operation(world);
        }
    }

    pub fn new_entity(&mut self) {
        self.operations.push(Box::new(NewEntity));
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.operations.push(Box::new(RemoveEntity(entity)));
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        self.operations
            .push(Box::new(AddComponent(entity, component)));
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: EntityId) {
        self.operations.push(Box::new(RemoveComponent::<T> {
            entity,
            _marker: PhantomData,
        }));
    }

    pub fn stop_run(&mut self) {
        self.operations.push(Box::new(StopRun));
    }
}
