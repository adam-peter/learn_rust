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
    let num = largest(&number_list);
    println!("The largest number is {}", num);
}

fn largest(number_list: &[i32]) -> &i32 {
    let mut largest = &number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
