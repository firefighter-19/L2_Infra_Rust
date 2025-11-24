mod generator;

mod prelude {
    pub const MAX_VALUE: i32 = 100;
    pub const MIN_VALUE: i32 = 1;
    pub use crate::generator::{generate_random_number};
    pub use rand::Rng;
}

use prelude::*;

fn main() {
    let random_number = generate_random_number();
    println!("Random number: {}", random_number.value);
}
