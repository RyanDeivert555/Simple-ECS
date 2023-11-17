#![allow(dead_code)]
use crate::commands::CommandQueue;
use crate::components::{Component, Query};
use crate::entities::{Entities, EntityId};
use crate::resources::{Resource, ResourcesMap};
use std::any::TypeId;
use std::cell::{Ref, RefMut};

#[derive(Default, Debug)]
pub struct World {
    entities: Entities,
    resources: ResourcesMap,
}

impl World {
    // entity operations
    fn valid_entity(&self, entity: EntityId) -> bool {
        self.entities.valid_entity(entity)
    }

    pub fn new_entity(&mut self) -> EntityId {
        self.entities.new_entity()
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        self.entities.remove_entity(entity);
    }

    pub fn remove_entities(&mut self, entities: impl Iterator<Item = EntityId>) {
        self.entities.remove_entities(entities);
    }

    pub fn available_slots(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.entities.available_slots()
    }

    pub fn new_entities(&mut self, count: usize) -> impl Iterator<Item = EntityId> + '_ {
        self.entities.new_entities(count)
    }

    // component operations
    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        self.entities.add_component(entity, component);
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: EntityId) {
        self.entities.remove_component::<T>(entity);
    }

    // command operations
    pub fn run_commands(&mut self) {
        let mut command_queue = {
            let mut command_queue = self.get_resource_mut::<CommandQueue>().unwrap();
            std::mem::take(&mut *command_queue) // deref coersion
        }; //we drop the refmut here, so it can't interfere with the `&mut` borrow we take on the next line
        command_queue.run_commands(self);
    }

    pub fn get_command_queue(&self) -> RefMut<'_, CommandQueue> {
        self.get_resource_mut::<CommandQueue>().unwrap()
    }

    // query operations
    pub fn get_component<T: Component + 'static>(&self, entity: EntityId) -> Option<Ref<'_, T>> {
        Some(Ref::map(
            self.entities
                .component_map(entity)?
                .get(&TypeId::of::<T>())?
                .borrow(),
            |b| b.downcast_ref::<T>().unwrap(),
        ))
    }

    pub fn get_component_mut<T: Component + 'static>(
        &self,
        entity: EntityId,
    ) -> Option<RefMut<'_, T>> {
        Some(RefMut::map(
            self.entities
                .component_map(entity)?
                .get(&TypeId::of::<T>())?
                .borrow_mut(),
            |b| b.downcast_mut::<T>().unwrap(),
        ))
    }

    pub fn get_components<Q: Query>(&self, entity: EntityId) -> Option<<Q>::Output<'_>> {
        // check to prevent expensive call
        if self.valid_entity(entity) {
            <Q>::query_components(self, entity)
        } else {
            None
        }
    }

    pub fn get_components_mut<Q: Query>(&self, entity: EntityId) -> Option<<Q>::OutputMut<'_>> {
        // check to prevent expensive call
        if self.valid_entity(entity) {
            <Q>::query_components_mut(self, entity)
        } else {
            None
        }
    }

    pub fn query_components<Q: Query>(&self) -> impl Iterator<Item = <Q>::Output<'_>> {
        self.entities
            .entities()
            .filter_map(|e| self.get_components::<Q>(e))
    }

    pub fn query_components_mut<Q: Query>(&self) -> impl Iterator<Item = <Q>::OutputMut<'_>> {
        self.entities
            .entities()
            .filter_map(|e| self.get_components_mut::<Q>(e))
    }

    pub fn query_single<Q: Query>(&self) -> Option<<Q>::Output<'_>> {
        self.query_components::<Q>().next()
    }

    pub fn query_single_mut<Q: Query>(&self) -> Option<<Q>::OutputMut<'_>> {
        self.query_components_mut::<Q>().next()
    }

    // resource operations
    pub fn add_resource<T: Resource + 'static>(&mut self, resource: T) {
        self.resources.add_resource(resource);
    }

    pub fn remove_resource<T: Resource + 'static>(&mut self) {
        self.resources.remove_resource::<T>();
    }

    pub fn get_resource<T: Resource + 'static>(&self) -> Option<Ref<'_, T>> {
        self.resources.get_resource::<T>()
    }

    pub fn get_resource_mut<T: Resource + 'static>(&self) -> Option<RefMut<'_, T>> {
        self.resources.get_resource_mut::<T>()
    }
}
