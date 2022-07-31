pub fn print_type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

pub mod core;