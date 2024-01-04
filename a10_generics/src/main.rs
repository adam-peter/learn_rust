use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut num = &number_list[0];
    for number in &number_list {
        if number > num {
            num = number
        }
    }
    println!("The largest number is {num}");

    // extracting a function to reduce duplication
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let num = largest_i32(&number_list);
    println!("The largest number is {}", num);

    let char_list = vec!['y', 'm', 'c', 'a'];
    let c = largest_char(&char_list);
    println!("The largest char in the list is '{c}'");

    let nums = vec![1, 2, 3, 4, 5];
    let chars = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    println!("Largest num: {}", largest(&nums));
    println!("Largest char: {}", largest(&chars));

    // generics in structs
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer: Point<i32> = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let different_types = Point { x: 1, y: 10.1 }; - errors - we have only 1 generic - it can be anything, but it must be same for the same construct
    println!("{integer:?}, {float:?}");

    #[derive(Debug)]
    struct PointMultiple<T, U> {
        x: T,
        y: U,
    }
    let different_types: PointMultiple<i32, f64> = PointMultiple {
        x: 10,
        y: 20.,
    };
    println!("{different_types:?}");

    // generics in enums
    enum MyOption<T> {
        Some(T),
        None,
    }
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // generics in methods
    impl<T> Point<T> {
        // impl<T> signifies, that the type T must be same for all methods on the generic struct
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 1, y: 2 };
    println!("{}", p.x());
    // we can specify type constraints - implement methods on structs that use only a specific type as their generic
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    impl<X1, Y1> PointMultiple<X1, Y1> {
        // the impl generics can be different than the method definition generics
        fn mixup<X2, Y2>(self, other: PointMultiple<X2, Y2>) -> PointMultiple<X1, Y2> { // X2 & Y2 are only relevant to this exact method
            PointMultiple {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = PointMultiple { x: 5, y: 10.4 };
    let p2 = PointMultiple { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixed up point: {p3:?}");
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}