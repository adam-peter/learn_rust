fn main() {
    // fixing returning a reference to the stack
    let mut s = String::from("");
    return_a_string(&mut s);
    println!("{s}");

    // fixing not enough permissions
    let name = vec![String::from("Adam"), String::from("Peter")];
    let full = stringify_name_with_title(&name);
    println!("{full}");

    // fixing aliasing & mutating a data structure
    let mut dst = vec![String::from("a"), String::from("bb")];
    let src = [String::from("ccc")];
    add_big_strings(&mut dst, &src);
    println!("{:?}", dst); // ["a", "bb", "ccc"]

    // fixing copying / moving out of a collection
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    // let s: String = *s_ref; - cannot move
    println!("{s_ref}!");

    // fixing mutating different tuple fields
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    let first = get_first(&name);
    name.1.push_str(" Esq.");
    println!("{first}, {}", name.1);

    // fixing mutating different array elements
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1]; // 1
    *x += 1;
    println!("{a:?}"); // [0, 2, 2, 3]

    // let y = &a[2];
     // println!("{x}, {y}"); - errors - immutable & mutable borrows occur at the same time

    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += 10;
    println!("- {x}\n\
    - {y}\n");
    println!("- {a_l:?}\n\
    - {a_r:?}");
}

fn return_a_string(s: &mut String)  {
    s.replace_range(.., "Hello world")
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn get_first(name: &(String, String)) -> String {
    name.0.clone()
}