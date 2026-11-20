use crate::ecs::EntityTest;

pub type SystemTest<T> = fn(EntityTest, &mut T);

pub struct SystemsTest<T> {
    pub system: Vec<SystemTest<T>>,
}

impl<T> SystemsTest<T> {
    pub fn new() -> SystemsTest<T> {
        Self { system: Vec::new() }
    }
}
