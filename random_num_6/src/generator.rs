use random_number::Generator;

mod random_number;

use crate::prelude::*;

pub fn generate_random_number() -> Generator {
  let random_number = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);
  Generator::new(random_number)
}