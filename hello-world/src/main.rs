fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let x = 5 + 1;
    println!("{}", x);
    println!("the result is 6: {}", x == 6);
    println!("add({},{}) = {}", 1, 2, add(1, 2));
}

#[test]
fn should_add_correctly() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(2, 2), 4);
}
