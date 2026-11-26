use crate::ecs::{
    component::ComponentTest,
    system::{SystemTest, SystemsTest},
};
pub mod component;
pub mod system;
/*
pub struct EcsTest {
    id: ComponentTest<usize>,
    test: ComponentTest<i64>,
    //Systems in Ecs ar for Spawners and Destroiers ONLY
    system: SystemsTest<EcsTest>,
}

type ID = usize;
type Index = usize;
enum IDex {
    ID(ID),
    Index(Index),
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

    fn spawn_entity(&mut self) -> ID {
          todo!()
    }

    fn destroy_entity(&mut self, idex: IDex) -> Result<(), ()> {
      let mut index:Index;
          match idex {
                IDex::ID(id)=>{
                      match self.id.component.binary_search(&idex.id) {
                            Ok(i)=> index = i,
                            Err(_) => return Err(()),
                      }
                },
                IDex::Index(i)=>{
                      if self.id.component.len()<i {
                            return Err(());
                      }
                      index = i;
                }
          }

          self.id.component.swap_remove(index);
          self.test.component.swap_remove(index);

          Ok(())
    }

    pub fn run_tick(&mut self) {}
*/

#[macro_export]
macro_rules! Ecs_builder {
      {$SName:ident {$($name:ident: $type:ty), +}} => {


            type ID =  usize;
            type Index =  usize;

            enum IDex {
                ID(ID),
                Index(Index),
            }

            pub struct $SName{
                  id: Component<ID>,
                  $($name: Component<$type>),+,

                  system: Systems<$SName>,
            }

            struct Entity<'a>{
                  $(pub $name: &'a $type),+
            }

            struct OwnedEntity{
                  $(pub $name: $type),+
            }

            static mut ID_COUNT : ID = 0;

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

                  pub fn add_mut_system(&mut self, system: MutSystem<$SName>){
                        self.system.add_mut_system(system)
                  }

                  pub fn add_void_system(&mut self, system: VoidSystem<$SName>){
                        self.system.add_void_system(system)
                  }

                  fn destroy_entity(&mut self, idex: IDex) -> Result<(), ()> {
                    let index:Index;
                        match idex {
                              IDex::ID(id)=>{
                                    match self.id.component.binary_search(&id) {
                                          Ok(i)=> index = i,
                                          Err(_) => return Err(()),
                                    }
                              },
                              IDex::Index(i)=>{
                                    if self.id.component.len()<i {
                                          return Err(());
                                    }
                                    index = i;
                              }
                        }

                        self.id.component.swap_remove(index);
                        $(
                        self.$name.component.swap_remove(index);
                        )+
                        Ok(())
                  }

                  fn spawn_entity(&mut self, entity: OwnedEntity) -> ID {
                        unsafe{
                        let id = ID_COUNT;
                        ID_COUNT += 1;

                        self.id.component.push(id);
                        $(
                        self.$name.component.push(entity.$name);
                        )+
                        id
                        }
                  }

            }

            /*--- System --- */

            type MutSystem<T> = fn(&$SName, &T)->T;
            type VoidSystem<T> = fn(&$SName, &T);

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

                pub fn add_mut_system(&mut self, system: MutSystem<T>){
                      self.system.add_mut_system(system)
                }

                pub fn add_void_system(&mut self, system: VoidSystem<T>){
                      self.system.add_void_system(system)
                }
            }
    }
}
