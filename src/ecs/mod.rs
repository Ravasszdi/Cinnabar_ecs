pub mod component;
pub mod system;

#[macro_export]
macro_rules! Ecs_builder {
      {$SName:ident {$($name:ident: $type:ty), +}} => {


            type ID =  usize;
            type Index =  usize;

            #[allow(dead_code)]
            enum IDex {
                ID(ID),
                Index(Index),
            }

            pub struct $SName{
                  id: Component<ID>,
                  $($name: Component<$type>),+,

                  system: Systems<$SName>,
            }

            struct RefEntity<'a>{
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

                  #[allow(dead_code)]
                  fn to_ref_entity(&self, id: usize) -> RefEntity{
                        RefEntity{
                              $($name: &self.$name.component[id]),+
                        }
                  }

                  pub fn add_mut_system(&mut self, system: MutSystem<$SName>){
                        self.system.add_mut_system(system)
                  }

                  pub fn add_void_system(&mut self, system: VoidSystem<$SName>){
                        self.system.add_void_system(system)
                  }

                  #[allow(dead_code)]
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

                  #[allow(dead_code)]
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

            System_Builder!($SName);

            /*--- Component ---*/

            Component_Builder!();

    }
}
