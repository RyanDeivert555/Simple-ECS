#![allow(dead_code)]
use crate::components::{Component, Query};
use crate::resources::Resource;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
// add entity bus, discord 9/5/23 at 11:00pm
// maybe try commands?

pub type EntityId = usize;
type ComponentMap = HashMap<TypeId, RefCell<Box<dyn Any>>>;

#[derive(Default)]
pub struct World {
    max_slot: usize,
    components: Vec<Option<ComponentMap>>,
    resources: ComponentMap,
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
    fn valid_entity(&self, entity: EntityId) -> bool {
        self.components[entity].is_some()
    }

    fn find_next_slot(&self) -> EntityId {
        self.components
            .iter()
            .position(|c| c.is_none())
            .unwrap_or(self.max_slot)
    }

    pub fn new_entity(&mut self) -> EntityId {
        let slot = self.find_next_slot();
        if slot == self.components.len() {
            self.components.push(Some(HashMap::new()));
            self.max_slot += 1;
        } else {
            self.components[slot] = Some(HashMap::new());
        }
        self.add_component(slot, Entity(slot));
        slot
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        assert!(entity < self.max_slot, "Invalid entity id");
        self.components[entity] = None;
    }

    pub fn available_slots(&self) -> impl Iterator<Item = EntityId> + '_ {
        (0..self.max_slot).filter(|e| self.valid_entity(*e))
    }

    pub fn new_entities(&mut self, count: usize) -> impl Iterator<Item = EntityId> + '_ {
        (0..count).map(|_| self.new_entity())
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
        (0..self.max_slot).filter_map(|e| self.get_components::<Q>(e))
    }

    pub fn query_components_mut<Q: Query>(&self) -> impl Iterator<Item = <Q>::OutputMut<'_>> {
        (0..self.max_slot).filter_map(|e| self.get_components_mut::<Q>(e))
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
