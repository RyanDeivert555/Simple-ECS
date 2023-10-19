#![allow(dead_code)]
use crate::components::Component;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

pub type EntityId = usize;
pub type ComponentMap = HashMap<TypeId, RefCell<Box<dyn Any>>>;

#[derive(Debug)]
pub struct Entity(EntityId);
impl Component for Entity {}

impl Entity {
    pub fn id(&self) -> EntityId {
        self.0
    }
}

#[derive(Default, Debug)]
pub struct Entities {
    entities: Vec<Option<ComponentMap>>,
    max_slot: usize,
}

impl Entities {
    pub fn valid_entity(&self, entity: EntityId) -> bool {
        entity < self.max_slot && self.entities[entity].is_some()
    }

    pub fn next_slot(&self) -> EntityId {
        self.entities
            .iter()
            .position(|c| c.is_none())
            .unwrap_or(self.max_slot)
    }

    pub fn new_entity(&mut self) -> EntityId {
        let slot = self.next_slot();
        if slot == self.entities.len() {
            self.entities.push(Some(HashMap::new()));
            self.max_slot += 1;
        } else {
            self.entities[slot] = Some(HashMap::new());
        }
        self.add_component(slot, Entity(slot));
        slot
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        // ok if an "invalid" entity is removed
        assert!(
            entity < self.max_slot,
            "Invalid Entity Id: {}\nMax Slot: {}",
            entity,
            self.max_slot
        );
        self.entities[entity] = None;
    }

    pub fn remove_entities(&mut self, entities: impl Iterator<Item = EntityId>) {
        for entity in entities {
            self.remove_entity(entity);
        }
    }

    pub fn available_slots(&self) -> impl Iterator<Item = EntityId> + '_ {
        (0..self.max_slot).filter(|e| self.valid_entity(*e))
    }

    pub fn new_entities(&mut self, count: usize) -> impl Iterator<Item = EntityId> + '_ {
        (0..count).map(|_| self.new_entity())
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        assert!(
            self.valid_entity(entity),
            "Invalid Entity Id: {}\nMax Slot: {}",
            entity,
            self.max_slot
        );
        self.entities[entity]
            .as_mut()
            .unwrap()
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(component)));
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: EntityId) {
        assert!(
            self.valid_entity(entity),
            "Invalid Entity Id: {}\nMax Slot: {}",
            entity,
            self.max_slot
        );
        self.entities[entity]
            .as_mut()
            .unwrap()
            .remove(&TypeId::of::<T>());
    }

    pub fn entities(&self) -> std::ops::Range<EntityId> {
        0..self.max_slot
    }

    pub fn component_map(&self, entity: EntityId) -> Option<&ComponentMap> {
        assert!(
            self.valid_entity(entity),
            "Invalid Entity Id: {}\nMax Slot: {}",
            entity,
            self.max_slot
        );
        self.entities[entity].as_ref()
    }
}
