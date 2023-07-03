fn main() {
    // let v = vec![0, 1, 2, 3];
    // println!("{}", v[6]);
    // panic!("Farewell!");

    let fruits: Vec<&str> = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first: Option<&&str> = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third: Option<&&str> = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent: Option<&&str> = fruits.get(99);
    println!("{:?}", non_existent);

    match_test();

    if_let_test();

    unwrap_expect();
}

fn match_test() {
    let fruits: Vec<&str> = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}

fn if_let_test() {
    let a_number: Option<u8> = Some(7);
    match a_number {
        Some(7) => println!("That's my lucky number!"),
        _ => {}
    }

    //better way for single match case
    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }
}

fn unwrap_expect() {
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    // let b: Option<&str> = None;
    // b.expect("fruits are healthy"); // panics with `fruits are healthy`

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(Some("doggy").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat"); // for none case
}
