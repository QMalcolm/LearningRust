fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string

    println!("{}", s); // This will print `hello, world!`
}
