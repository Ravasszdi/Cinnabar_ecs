use crate::ecs::{
    component::ComponentTest,
    system::{SystemTest, SystemsTest},
};

pub mod component;
pub mod system;
pub struct EcsTest {
    id: ComponentTest<usize>,
    test: ComponentTest<i64>,
    //Systems in Ecs ar for Spawners and Destroiers ONLY
    system: SystemsTest<EcsTest>,
}

pub struct EntityTest<'a> {
    pub test: &'a i64,
}

impl EcsTest {
    pub fn new() -> EcsTest {
        EcsTest {
            id: ComponentTest::new(),
            test: ComponentTest::new(),
            system: SystemsTest::new(),
        }
    }

    pub fn to_entity<'a>(&'a self, id: usize) -> EntityTest {
        self.test.component[id];
        EntityTest {
            test: &self.test.component[id],
        }
    }

    pub fn add_system(&mut self, system: SystemTest<EcsTest>) {
        self.system.system.push(system);
        self.system.system.shrink_to_fit();
    }

    pub fn run(&mut self) {
        for sys in &self.system.system.clone() {
            for id in &self.id.component.clone() {
                sys(self.to_entity(*id), &mut self);
            }
        }
    }
}

#[macro_export]
macro_rules! Ecs_builder {
      {$SName:ident {$($name:ident: $type:ty), +}} => {



            pub struct $SName{
                  id: Component<usize>,
                  $($name: Component<$type>),+,

                  system: Systems<$SName>,
            }

            struct Entity<'a>{
                  $(pub $name: &'a $type),+
            }

            impl $SName {
                  pub fn new() -> $SName{
                        $SName{
                              id: Component::new(),
                              $($name: Component::new()),+,

                              system: Systems::new(),
                        }
                  }

                  pub fn to_entity(&self, id: usize) -> Entity{
                        Entity{
                              $($name: &self.$name.component[id]),+
                        }
                  }

                  pub fn add_system(&mut self, system: System<$SName>){
                        self.system.system.push(system);
                        self.system.system.shrink_to_fit();
                  }
            }

            /*--- System --- */

            type System<T> = fn(Entity, T);

            struct Systems<T> {
                pub system: Vec<System<T>>,
            }

            impl<T> Systems<T> {
                pub fn new() -> Systems<T> {
                    Self { system: Vec::new() }
                }
            }


            /*--- Component ---*/

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

                pub fn add_system(&mut self, system: System<T>){
                      self.system.system.push(system);
                      self.system.system.shrink_to_fit();
                }
            }
    };
}
