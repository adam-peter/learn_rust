mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    // since all of the fields aren't public - we need a pub associated function on Breakfast to construct it (new, here it's summer)
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // if enum is pub - all of enum variants are pub
    pub enum Appetizer {
        Soup,
        Salad
    }
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
    println!("{meal:?}");
    // meal.seasonal_fruit = String::from("blueberries"); - error - seasonal_fruit field is private

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}