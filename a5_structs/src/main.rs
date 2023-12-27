#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, rect: Rectangle) -> Rectangle { // takes ownership
        Rectangle {
            width: self.width.max(rect.width),
            height: self.height.max(rect.height),
        }
    }
}

// associated functions
impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The are of the rectangle is {} square pixels.", area(&rect));
    println!("The rectangle is {:?}", rect);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The are of the rectangle is {} square pixels.", rect2.area());

    if rect2.width() { // calling the width() method
        println!("rect2 has a width of {}", rect2.width); // accessing the width field
    }

    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    let r2 = Rectangle {
        width: 10,
        height: 40,
    };
    let r3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", r1.can_hold(&r2));
    println!("Can rect1 hold rect3? {}", r1.can_hold(&r3));

    // using associated functions
    let s1 = Rectangle::square(10);
    println!("We made a new square: {s1:#?}");
    println!("The area of our square is: {}", s1.area());

    // methods == syntactic sugar for associated functions
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };
    let a1 = r.area();
    let a2 = Rectangle::area(&r);
    assert_eq!(a1, a2);

    println!("{r:?}");
    r.set_width(2);
    println!("{r:?}");
    Rectangle::set_width(&mut r, 3);
    println!("{r:?}");

    let r = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", r.area()); // can read
    let r = r.max(Rectangle { // can own
        width: 10,
        height: 0,
    });
    // r.set_width(20); // errors - cannot mutate immutable r

}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}