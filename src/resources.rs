use crate::entities::ComponentMap;
use std::any::TypeId;
use std::cell::{Ref, RefCell, RefMut};

pub trait Resource {}

#[derive(Default)]
pub struct ResourcesMap {
    resources: ComponentMap,
}

impl ResourcesMap {
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

#[macro_export]
macro_rules! register_resources {
    ($($generic_type:ident),*) => {
        $(
            impl $crate::resources::Resource for $generic_type {}
        )*
    };
}
pub use register_resources;
