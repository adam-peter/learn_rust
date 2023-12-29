mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// external code before: restaurant::front_of_house::hosting::add_to_waitlist();
pub use crate::front_of_house::hosting; // external code after: restaurant::hosting::add_to_waitlist();

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    todo!()
}

fn function2() -> IoResult<()> {
    todo!()
}