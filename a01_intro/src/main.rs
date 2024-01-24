use std::collections::HashMap;

// macros
macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    };
}

fn main() {
    let nums: [i32; 3] = [1, 2, 3];
    for i in nums {
        println!("{i}");
    }
    println!("{}", &nums[1]);

    let mut mutable_arr = [1, 2, 0];
    mutable_arr[2] = 999;
    println!("{mutable_arr:?}");

    let zeros = [0; 100];
    println!("{:?}", &zeros[5..8]);

    let mut profile = HashMap::new();
    profile.insert("name", CharacterValue::Name(String::from("Adam")));
    profile.insert("age", CharacterValue::Age(18));
    profile.insert("items", CharacterValue::Items(vec![
        String::from("laptop"),
        String::from("book"),
        String::from("coat"),
    ]));
    println!("{profile:?}");

    let jeff = Human {
        name: "Jeff".to_string(),
        age: 26,
        current_thought: Some("coffee".to_string()),
        friend: Friend::Nil,
    };
    let developer = Human {
        name: String::from("Adam Peter"),
        age: 18,
        current_thought: Some(String::from("rust")),
        friend: Friend::HUMAN(Box::new(jeff)),
    };
    println!("{developer:?}");
    println!("{}", developer.name);

    let developer = Human::new("Adam", 18);
    println!("{developer:?}");

    let socrates = Human::new("Socrates", 68).with_thought("Thinking...");
    println!("{socrates:?}");

    let plato = Human::new("Plato", 41).with_friend(Box::new(socrates));
    println!("{plato:?}");

    let u = User {
        username: "user".to_string(),
        password: "password".to_string()
    };
    let a = AdminUser {
        username: "admin".to_string(),
        password: "password".to_string()
    };
    u.edit();
    a.edit();
    a.create();
    a.delete();

    let mut x = "test".to_string();
    capitalize!(x);
    println!("{x}");
}

#[derive(Debug)]
enum CharacterValue {
    Name(String),
    Age(i32),
    Items(Vec<String>),
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friend: Friend,
}

impl Human {
    fn new(name: &str, age: i8) -> Human {
        Human {
            name: name.to_string(),
            age: age,
            current_thought: None,
            friend: Friend::Nil,
        }
    }

    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = Some(thought.to_string());
        self
    }

    fn with_friend(mut self, friend: Box<Human>) -> Human {
        self.friend = Friend::HUMAN(friend);
        self
    }
}

#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    Nil,
}

struct AdminUser {
    username: String,
    password: String,
}

impl CanEdit for AdminUser {}

impl CanCreate for AdminUser {}

impl CanDelete for AdminUser {}

struct User {
    username: String,
    password: String,
}

impl CanEdit for User {
    fn edit(&self) {
        println!("a standard user {} is editing", self.username);
    }
}

trait CanEdit {
    fn edit(&self) {
        println!("admin is editing");
    }
}

trait CanCreate {
    fn create(&self) {
        println!("admin is creating");
    }
}

trait CanDelete {
    fn delete(&self) {
        println!("admin is deleting");
    }
}

// Questions
// 1 String - stored on the heap, pointer to memory; str - stored on the stack
// 2 we cant determine their size beforehand
// 3 with a get() function
// 4 it depends - if the function panics, the program crashes; if the function returns a result, we can match on it and run code in case it returns an error
// 5 mutating a heap-stored value might change its address (it could need more space) - which would invalidate the other reference
// 6 if the output reference only depends on one of the values - or if the condition determines on which of the inputs does the output depend
// 7 they must use indirection - some kind of a pointer (Box) - structs need exactly defined space for their fields, recurring structs could have infinite size
// 8 we attach it to a trait and implement it on our struct - we can then overwrite as needed
// 9 using generics in its definition - we can then control the generic type by bounding it to only be of values implementing specific traits
// 10 derive it #[derive(Copy)] - implements the default implementation