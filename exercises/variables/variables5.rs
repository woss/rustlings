// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "3";
    println!("Number {}", number);
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html?highlight=sha#shadowing
    let number = 3;
    println!("Number {}", number);
}
