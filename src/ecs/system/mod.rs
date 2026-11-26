use crate::ecs::{EcsTest, component};

pub type SystemTest<T> = fn(&EcsTest, &T) -> T;
pub type VoidSystemTest<T> = fn(&EcsTest, &T);

pub enum SystemTypesTest<T> {
    MutSystem(SystemTest<T>),
    VoidSystem(VoidSystemTest<T>),
}

pub struct SystemsTest<T> {
    pub system: Vec<SystemTypesTest<T>>,
}

impl<T> SystemsTest<T> {
    pub fn new() -> SystemsTest<T> {
        Self { system: Vec::new() }
    }

    pub fn add_mut_system(&mut self, system: SystemTest<T>) {
        self.system.push(SystemTypesTest::MutSystem(system));
        self.system.shrink_to_fit();
    }

    pub fn add_void_system(&mut self, system: VoidSystemTest<T>) {
        self.system.push(SystemTypesTest::VoidSystem(system));
        self.system.shrink_to_fit();
    }

    pub fn update(&self, ecs: &EcsTest, components: Vec<T>) -> Vec<T> {
        let mut components = components;
        for sys in &self.system {
            match sys {
                SystemTypesTest::MutSystem(sys) => {
                    let mut new_components = Vec::new();
                    new_components.reserve(components.len());

                    for component in &components {
                        new_components.push(sys(&ecs, component));
                    }

                    components = new_components;
                }

                SystemTypesTest::VoidSystem(sys) => {
                    for component in &components {
                        sys(&ecs, component);
                    }
                }
            }
        }
        components
    }
}
