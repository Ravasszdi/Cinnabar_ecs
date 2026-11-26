#[macro_export]
macro_rules! System_Builder {
    ($SName: ident) => {
        type MutSystem<T> = fn(&$SName, &T) -> T;
        type VoidSystem<T> = fn(&$SName, &T);

        #[allow(dead_code)]
        enum SystemTypes<T> {
            MutSystem(MutSystem<T>),
            VoidSystem(VoidSystem<T>),
        }

        struct Systems<T> {
            pub system: Vec<SystemTypes<T>>,
        }

        impl<T> Systems<T> {
            pub fn new() -> Systems<T> {
                Self { system: Vec::new() }
            }

            pub fn add_mut_system(&mut self, system: MutSystem<T>) {
                self.system.push(SystemTypes::MutSystem(system));
                self.system.shrink_to_fit();
            }

            pub fn add_void_system(&mut self, system: VoidSystem<T>) {
                self.system.push(SystemTypes::VoidSystem(system));
                self.system.shrink_to_fit();
            }

            #[allow(dead_code)]
            pub fn update(&self, ecs: &$SName, components: Vec<T>) -> Vec<T> {
                let mut components = components;
                for sys in &self.system {
                    match sys {
                        SystemTypes::MutSystem(sys) => {
                            let mut new_components = Vec::new();
                            new_components.reserve(components.len());

                            for component in &components {
                                new_components.push(sys(ecs, component));
                            }

                            components = new_components;
                        }

                        SystemTypes::VoidSystem(sys) => {
                            for component in &components {
                                sys(ecs, component);
                            }
                        }
                    }
                }
                components
            }
        }
    };
}
