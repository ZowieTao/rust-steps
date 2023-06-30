use std::vec;

fn main() {
    array_test();

    vector_test();

    car_factory_test();

    condition_test()
}

fn array_test() {
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

fn vector_test() {
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
    // let beyond = index_vec[10];
    // println!("{}", beyond);
}

fn car_factory_test() {
    #[derive(PartialEq, Debug)]
    // Declare Car struct to describe vehicle with four named fields
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
    // Declare enum for Car transmission type
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }

    #[derive(PartialEq, Debug)]
    enum Age {
        New,
        Used,
    }

    // Get the car quality by testing the value of the input argument
    // - miles (u32)
    // Create a tuple for the car quality with the Age ("New" or "Used") and mileage
    // Return a tuple with the arrow `->` syntax
    fn car_quality(miles: u32) -> (Age, u32) {
        // Declare and initialize the return tuple value
        // For a new car, set the miles to 0
        let car_type: Age = if miles == 0 { Age::New } else { Age::Used };

        let quality: (Age, u32) = (car_type, miles);

        // Return the completed tuple to the caller
        quality
    }

    // Build a new "Car" using the values of four input arguments
    // - color (String)
    // - motor (Transmission enum)
    // - roof (boolean, true if the car has a hard top roof)
    // - miles (u32)
    // Call the car_quality(miles) function to get the car age
    // Return an instance of a "Car" struct with the arrow `->` syntax
    fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
        // Create a new "Car" instance as requested
        // - Bind first three fields to values of input arguments
        // Corrected code: "mileage" is replaced with "age"
        // Corrected code: Bind "age" to tuple returned from car_quality(miles)
        Car {
            color: color,
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut car: Car;
    let mut engine = Transmission::Manual;

    //////////////////////////////////////////////////

    // Order 3 cars, one car for each type of transmission
    // Corrected code: Index into `colors` array and vary color for the orders

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!(
        "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: New, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!(
        "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: New, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!(
        "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
        car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}

fn condition_test() {
    if 1 == 2 {
        println!("True, the number are equal.")
    } else {
        println!("False, the number are not equal.")
    }

    let formal = true;
    let greeting = if formal { "Good day to you." } else { "Hey!" };
    println!("{}", greeting);

    fn rang_512_condition(num: i32) -> bool {
        let out_of_range: bool;
        if num < 0 {
            out_of_range = true;
        } else if num == 0 {
            out_of_range = true;
        } else if num > 512 {
            out_of_range = true;
        } else {
            out_of_range = false;
        }

        out_of_range
    }

    println!(
        "1234 {} out of range 512",
        if rang_512_condition(1234) {
            "is"
        } else {
            "not"
        }
    );
}
