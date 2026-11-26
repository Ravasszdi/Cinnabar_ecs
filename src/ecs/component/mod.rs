#[macro_export]
macro_rules! Component_Builder {
    () => {
        struct Component<T> {
            pub component: Vec<T>,

            //Systems in Component are for updateing not for spawning or destroing
            system: Systems<T>,
        }

        impl<T> Component<T> {
            pub fn new() -> Component<T> {
                Component {
                    component: Vec::new(),
                    system: Systems::new(),
                }
            }

            pub fn add_mut_system(&mut self, system: MutSystem<T>) {
                self.system.add_mut_system(system)
            }

            pub fn add_void_system(&mut self, system: VoidSystem<T>) {
                self.system.add_void_system(system)
            }
        }
    };
}
