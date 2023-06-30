fn main() {
    // Declare array, initialize all values, compiler infers length = 7
    let days: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Declare array, initialize all values to 0, length = 5
    let bytes: [i32; 5] = [0i32; 5];
    println!("Hello, world!");

    // Get the first day of the week
    let first: &str = days[0];

    // Get the second day of the week
    let second: &str = days[1];

    // let seventh = days[7];
    println!(
        "bytes: {:?} \n first: {} \n second: {}",
        bytes, first, second
    );

    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    let zeros = vec![3.1415926; 50];
    println!("Zeros: {:?}", zeros);

    // Create empty vector, declare vector mutable so it can grow and shrink, MUST SAME TYPE LIKE ARRAY, NOT TUPLE
    let mut fruit: Vec<&str> = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Push integer value, but vector expects String (&str) type value
    // fruit.push(1);

    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Pop off: {:?}", fruit.pop());
    println!("Pop off: {:?}", fruit.pop());
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);

    // Access vector with out-of-bounds index
    let beyond = index_vec[10];
    println!("{}", beyond);
}
