// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// Approach 1
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// Approach 2
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main_no_ownership() {
    let mut vec0 = Vec::new();

    fill_vec_no_ownership(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec_no_ownership(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
