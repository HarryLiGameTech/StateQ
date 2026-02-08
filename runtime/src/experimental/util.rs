
use std::any::{Any, TypeId};

pub trait Is: Any {
    fn is<T: Any>(&self) -> bool {
        self.type_id() == TypeId::of::<T>()
    }
}

impl Is for dyn Any {}
