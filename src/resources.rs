pub trait Resource {}

#[macro_export]
macro_rules! register_resources {
    ($($generic_type:ident),*) => {
        $(
            impl $crate::resources::Resource for $generic_type {}
        )*
    };
}
pub use register_resources;
