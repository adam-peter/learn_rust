use rand;
use rand::Rng;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WashingtonDC,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let my_coin = Coin::Nickel;
    println!("{}", value_in_cents(&my_coin));

    let my_coin = Coin::Penny;
    println!("{}", value_in_cents(&my_coin));

    let my_quarter = Coin::Quarter(UsState::NewYork);
    println!("{}", value_in_cents(&my_quarter));

    let five = Some(5);
    let six = plus_one(&five);
    let none = plus_one(&None);
    println!("{five:?}, {six:?}, {none:?}");

    let dice_roll = rand::thread_rng().gen_range(1..10);
    handle_roll(dice_roll);

    // care when binding heap-allocated data to match patterns
    let opt: Option<String> = Some(String::from("Hello world"));
    match &opt { // match on a reference, else "s" will be moved
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    };
    println!("{:?}", opt);
}

fn value_in_cents(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(num: &Option<i32>) -> Option<i32> {
    match num {
        Some(n) => Some(n + 1),
        None => None
    }
}

fn handle_roll(roll: i32) {
    println!("You rolled {roll}!");
    match roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        1 | 2 | 4 | 5 => move_player(roll),
        _ => println!("You must roll again!")
        // _ => () - also possible - nothing happens
    }
}

fn add_fancy_hat() { println!("You got a fancy hat!"); }

fn remove_fancy_hat() { println!("Oh no, you lost your fancy hat!"); }

fn move_player(spaces: i32) { println!("You moved {spaces} spaces."); }