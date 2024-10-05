use class::Classes;

#[derive(Classes)]
pub enum Thing {
  #[class(animal, alive, organic)]
  Dog,
  #[class(animal, alive, organic, sharp)]
  Cat,
  #[class(animal, alive, organic)]
  Bird,
  #[class(plant, alive, organic)]
  Tree,
  Rock,
  #[class(organic)]
  Paper,
  #[class(sharp)]
  Scissors,
}

pub fn thing(x: Thing) {
  match x {
    Thing![!(sharp || organic && !animal)] => unreachable!(),
    Thing::Cat => todo!(),
    Thing::Tree => todo!(),
    Thing::Paper => todo!(),
    Thing::Scissors => todo!(),
  }
}
