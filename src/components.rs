use crate::entities::EntityId;
use crate::world::World;
use std::cell::{Ref, RefMut};

pub trait Component {}

pub trait Query {
    type Output<'lt>;
    type OutputMut<'lt>;
    fn query_components(world: &World, entity: EntityId) -> Option<Self::Output<'_>>;
    fn query_components_mut(world: &World, entity: EntityId) -> Option<Self::OutputMut<'_>>;
}

impl<T: Component + 'static> Query for T {
    type Output<'lt> = Ref<'lt, T>;
    type OutputMut<'lt> = RefMut<'lt, T>;

    fn query_components(world: &World, entity: EntityId) -> Option<Self::Output<'_>> {
       world.get_component::<T>(entity)
    }
    
    fn query_components_mut(world: &World, entity: EntityId) -> Option<Self::OutputMut<'_>> {
        world.get_component_mut::<T>(entity)
    }
}

#[macro_export]
macro_rules! __impl_query {
    ($($generic_type:ident),+) => {
        impl<$($generic_type),*> $crate::components::Query for ($($generic_type,)*)
        where
        $(
            $generic_type: 'static + $crate::components::Component,
        )*
        {
            type Output<'lt> = ($(::core::cell::Ref<'lt, $generic_type>,)*);
            type OutputMut<'lt> = ($(::core::cell::RefMut<'lt, $generic_type>,)*);

            fn query_components(world: &$crate::world::World, entity: $crate::entities::EntityId) -> Option<Self::Output<'_>> {
                Some(
                    (
                        $(world.get_component::<$generic_type>(entity)?, )*
                    )
                )
            }

            fn query_components_mut(world: &$crate::world::World, entity: $crate::entities::EntityId) -> Option<Self::OutputMut<'_>> {
                Some(
                    (
                        $(world.get_component_mut::<$generic_type>(entity)?, )*
                    )
                )
            }
        }
    };
}

#[macro_export]
macro_rules! __recurse_without_last_arg {
    ($macro_name:ident [$($init_arg:ident),*] $last_arg:ident) => {
        $macro_name!($($init_arg),*);
    };
    ($macro_name:ident [$($init_arg:ident),*] $not_last_arg:ident, $($tail_arg:ident),+) => {
        __recurse_without_last_arg!($macro_name [$($init_arg,)* $not_last_arg] $($tail_arg),*);
    };
}

#[macro_export]
macro_rules! __impl_all_query {
    () => {};
    ($($generic_type:ident),*) => {
        __recurse_without_last_arg!(__impl_all_query [] $($generic_type),*);
        __impl_query!($($generic_type),*);
    };
}

#[macro_export]
macro_rules! register_components {
    ($($generic_type:ident),*) => {
        $(
            impl $crate::components::Component for $generic_type {}
        )*
        __impl_all_query!($($generic_type),*);
    };
}
pub use register_components;
