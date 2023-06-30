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
}
