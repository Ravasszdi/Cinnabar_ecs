pub mod ecs;
pub mod macros;

fn main() {
    /*
    let mut test = Test::new();

    fn asd(_item: Entity, _owned: Test) {
        println!("Benn van");
    }

    */
}

Ecs_builder! {
      Test {
            a: i64
      }
}
