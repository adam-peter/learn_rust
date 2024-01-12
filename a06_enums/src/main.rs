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

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    }
    if let Some(max) = config_max { // same thing
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let my_coin = Coin::Quarter(UsState::NewYork);
    match &my_coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1
    }
    if let Coin::Quarter(state) = &my_coin { // same thing
        println!("State quarter from {state:?}!");
    } else { // we can also include code for the catch-all part
        count += 1
    }
}