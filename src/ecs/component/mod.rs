use crate::ecs::system::SystemsTest;

pub struct ComponentTest<T> {
    pub component: Vec<T>,

    //Systems in Component are for updateing not for spawning or destroing
    system: SystemsTest<T>,
}

impl<T> ComponentTest<T> {
    pub fn new() -> ComponentTest<T> {
        ComponentTest {
            component: Vec::new(),
            system: SystemsTest::new(),
        }
    }
}
