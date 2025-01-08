use std::mem;
fn main() {
    let s = "hello";
    assert_eq!(s.len(), 5);
    let arr = ['h', 'e', 'l', 'l', 'o'];

    let size: usize = arr.iter().map(|c| mem::size_of_val(&c)).sum();

    println!("{size}");
}
