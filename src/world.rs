#![allow(dead_code)]
use crate::components::{Component, Query};
use crate::operations::OperationStack;
use crate::resources::Resource;
use crate::systems::System;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;

pub type EntityId = usize;
type ComponentMap = HashMap<TypeId, RefCell<Box<dyn Any>>>;

#[derive(Default)]
pub struct World {
    max_slot: usize,
    components: Vec<Option<ComponentMap>>,
    empty_slot: Option<usize>,
    resources: ComponentMap,
    systems: Vec<Box<dyn System>>,
    run: bool,
}

pub struct Entity(EntityId);
impl Component for Entity {}

impl Entity {
    pub fn id(&self) -> EntityId {
        self.0
    }
}

// entity operations
impl World {
    pub fn new() -> Self {
        Self {
            max_slot: 0,
            components: Vec::new(),
            empty_slot: None,
            resources: HashMap::new(),
            systems: Vec::new(),
            run: true,
        }
    }

    fn valid_entity(&self, entity: EntityId) -> bool {
        self.components[entity].is_some()
    }

    pub fn next_slot(&self) -> EntityId {
        if let Some(slot) = self.empty_slot {
            slot
        } else {
            self.max_slot
        }
    }

    pub fn new_entity(&mut self) -> EntityId {
        if let Some(slot) = self.empty_slot {
            self.components[slot] = Some(HashMap::new());
            self.empty_slot = None;
            self.add_component(slot, Entity(slot));
            slot
        } else {
            let id = self.max_slot;
            self.components.push(Some(HashMap::new()));
            self.max_slot += 1;
            self.add_component(id, Entity(id));
            id
        }
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        // multiple removes might skip a slot
        assert!(entity < self.max_slot, "Invalid entity id");
        self.empty_slot = Some(entity);
        self.components[entity] = None;
    }

    pub fn get_entities(&self) -> std::ops::Range<EntityId> {
        0..self.max_slot
    }
}

// component operations
impl World {
    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        assert!(entity < self.max_slot, "Invalid entity id");
        assert!(self.valid_entity(entity), "Entity does not exist");
        if let Some(component_map) = self.components[entity].as_mut() {
            component_map.insert(TypeId::of::<T>(), RefCell::new(Box::new(component)));
        }
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: EntityId) {
        assert!(entity < self.max_slot, "Invalid entity id");
        assert!(self.valid_entity(entity), "Entity does not exist");
        if let Some(component_map) = self.components[entity].as_mut() {
            component_map.remove(&TypeId::of::<T>());
        }
    }

    pub fn get_component<T: Component + 'static>(&self, entity: EntityId) -> Option<Ref<'_, T>> {
        Some(Ref::map(
            self.components[entity]
                .as_ref()?
                .get(&TypeId::of::<T>())?
                .borrow(),
            |b| (**b).downcast_ref::<T>().unwrap(),
        ))
    }

    pub fn get_component_mut<T: Component + 'static>(
        &self,
        entity: EntityId,
    ) -> Option<RefMut<'_, T>> {
        Some(RefMut::map(
            self.components[entity]
                .as_ref()?
                .get(&TypeId::of::<T>())?
                .borrow_mut(),
            |b| (**b).downcast_mut::<T>().unwrap(),
        ))
    }

    pub fn get_components<Q: Query>(&self, entity: EntityId) -> Option<<Q>::Output<'_>> {
        // check to prevent expensive call
        if entity < self.max_slot && self.valid_entity(entity) {
            <Q>::query_components(self, entity)
        } else {
            None
        }
    }

    pub fn get_components_mut<Q: Query>(&self, entity: EntityId) -> Option<<Q>::OutputMut<'_>> {
        // check to prevent expensive call
        if entity < self.max_slot && self.valid_entity(entity) {
            <Q>::query_components_mut(self, entity)
        } else {
            None
        }
    }
}

// query operations
impl World {
    pub fn query_components<Q: Query>(&self) -> impl Iterator<Item = <Q>::Output<'_>> {
        self.get_entities()
            .filter_map(|e| self.get_components::<Q>(e))
    }

    pub fn query_components_mut<Q: Query>(&self) -> impl Iterator<Item = <Q>::OutputMut<'_>> {
        self.get_entities()
            .filter_map(|e| self.get_components_mut::<Q>(e))
    }

    pub fn query_single<Q: Query>(&self) -> Option<<Q>::Output<'_>> {
        self.query_components::<Q>().next()
    }

    pub fn query_single_mut<Q: Query>(&self) -> Option<<Q>::OutputMut<'_>> {
        self.query_components_mut::<Q>().next()
    }
}

// resource operations
impl World {
    pub fn add_resource<T: Resource + 'static>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        assert!(
            !self.resources.contains_key(&type_id),
            "Resource already added!"
        );
        self.resources
            .insert(type_id, RefCell::new(Box::new(resource)));
    }

    pub fn remove_resource<T: Resource + 'static>(&mut self) {
        self.resources.remove(&TypeId::of::<T>());
    }

    pub fn get_resource<T: Resource + 'static>(&self) -> Option<Ref<'_, T>> {
        Some(Ref::map(
            self.resources.get(&TypeId::of::<T>())?.borrow(),
            |b| (**b).downcast_ref::<T>().unwrap(),
        ))
    }

    pub fn get_resource_mut<T: Resource + 'static>(&self) -> Option<RefMut<'_, T>> {
        Some(RefMut::map(
            self.resources.get(&TypeId::of::<T>())?.borrow_mut(),
            |b| (**b).downcast_mut::<T>().unwrap(),
        ))
    }
}

// system operations
impl World {
    pub fn add_system(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }

    fn update_systems(&self, operation_stack: &mut OperationStack) {
        for system in self.systems.iter() {
            system.run(self, operation_stack);
        }
    }
}

// operation stack operations
impl World {
    fn update_operations(&mut self, operation_stack: &mut OperationStack) {
        operation_stack.update_operations(self);
    }
}

// run operations
impl World {
    pub fn stop_run(&mut self) {
        self.run = false;
    }

    pub fn run(&mut self, operation_stack: &mut OperationStack) {
        while self.run {
            self.update_operations(operation_stack);
            self.update_systems(operation_stack);
        }
    }
}
