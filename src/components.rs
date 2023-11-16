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

__impl_query!(T1, T2);
__impl_query!(T1, T2, T3);
__impl_query!(T1, T2, T3, T4);
__impl_query!(T1, T2, T3, T4, T5);
__impl_query!(T1, T2, T3, T4, T5, T6);
__impl_query!(T1, T2, T3, T4, T5, T6, T7);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

#[macro_export]
macro_rules! register_components {
	($($generic_type:ident),*) => {
		$(
			impl $crate::components::Component for $generic_type {}
		)*
	};
}
pub use register_components;
