fn main() {
    string_non_literal();
    string_clone();
    stack_only_data_copy();
}

fn string_non_literal() {
    println!("----In string_non_literal");
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s); // This will print `hello, world!`
}

fn string_clone() {
    println!("----In string_clone");
    let s1 = String::from("hello");
    let s2 = s1; // s1 now invalid (`move` not `shallow copy`)
    let s3 = s2.clone(); // s2 is still valid (deep copy created)

    println!("s2 = {}, s3 = {}", s2, s3);
}

fn stack_only_data_copy() {
    println!("----In stack_only_data_copy");
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
