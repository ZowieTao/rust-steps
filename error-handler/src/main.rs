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
}
