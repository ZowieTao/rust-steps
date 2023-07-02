fn main() {
    hash_map_test();

    hash_map_unit_test();

    let loop_time = 164000;

    loop_test(loop_time);

    while_test(loop_time);

    for_test();

    car_test();
}

fn hash_map_test() {
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

fn hash_map_unit_test() {
    #[derive(PartialEq, Debug)]
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
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
    // Return tuple with car age ("New" or "Used") and mileage
    fn car_quality(miles: u32) -> (Age, u32) {
        // Check if car has accumulated miles
        // Return tuple early for Used car
        if miles > 0 {
            return (Age::Used, miles);
        }

        // Return tuple for New car, no need for "return" keyword or semicolon
        (Age::New, miles)
    }

    // Build "Car" using input arguments
    fn car_factory(order: i32, miles: u32) -> Car {
        let colors: [&str; 4] = ["Blue", "Green", "Red", "Silver"];

        // Prevent panic: Check color index for colors array, reset as needed
        // Valid color = 1, 2, 3, or 4
        // If color > 4, reduce color to valid index
        let mut color: usize = order as usize;
        if color > 4 {
            // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
            color = if color % 4 == 0 { 1 } else { color % 4 };
        }

        // Add variety to orders for motor type and roof type
        let mut motor = Transmission::Manual;
        let mut roof = true;
        if order % 3 == 0 {
            // 3, 6, 9
            motor = Transmission::Automatic;
        } else if order % 2 == 0 {
            // 2, 4, 8, 10
            motor = Transmission::SemiAuto;
            roof = false;
        } // 1, 5, 7, 11

        // Return requested "Car"
        Car {
            color: String::from(colors[(color - 1) as usize]),
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    // Corrected code: To create a hash map, use HashMap::new()
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 6 cars
    // - Increment "order" after each request
    // - Add each order <K, V> pair to "orders" hash map
    // - Corrected code: Use ".insert()" method to add each order
    // - Adjust println call to show order details from the hash map

    // Car order #1: Used, Hard top
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
}

fn loop_test(loop_time: u32) {
    use std::time::Instant;
    let mut count: u32 = 0;

    let start: Instant = Instant::now();
    let stop_loop: u32 = loop {
        count += 1;

        if count == loop_time {
            break count;
        }
    };

    let elapsed: std::time::Duration = start.elapsed();

    println!("Time elapsed: {:?}, loop_stop time: {}", elapsed, stop_loop);
}

fn while_test(loop_time: u32) {
    use std::time::Instant;
    let mut counter = 0;

    let start: Instant = Instant::now();
    while counter < loop_time {
        counter = counter + 1;
    }

    let elapsed: std::time::Duration = start.elapsed();
    println!("While Time elapsed: {:?}", elapsed);
}

fn for_test() {
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }

    for number in 0..5 {
        println!("{}", number * 2);
    }
}

fn car_test() {
    #[derive(PartialEq, Debug)]
    struct Car {
        color: String,
        motor: Transmission,
        roof: bool,
        age: (Age, u32),
    }

    #[derive(PartialEq, Debug)]
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
    // Return tuple with car age ("New" or "Used") and mileage
    fn car_quality(miles: u32) -> (Age, u32) {
        // Check if car has accumulated miles
        // Return tuple early for Used car
        if miles > 0 {
            return (Age::Used, miles);
        }

        // Return tuple for New car, no need for "return" keyword or semicolon
        (Age::New, miles)
    }

    // Build "Car" using input arguments
    fn car_factory(order: i32, miles: u32) -> Car {
        let colors = ["Blue", "Green", "Red", "Silver"];

        // Prevent panic: Check color index, reset as needed
        // Valid colors = 1, 2, 3, or 4
        // Corrected code: Replace if/else with loop to reduce color to lowest divisor of 4
        let mut color = order as usize;
        while color > 4 {
            // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
            color = color - 4;
        }

        // Add variety to orders for motor type and roof type
        let mut motor = Transmission::Manual;
        let mut roof = true;
        if order % 3 == 0 {
            // 3, 6, 9
            motor = Transmission::Automatic;
        } else if order % 2 == 0 {
            // 2, 4, 8, 10
            motor = Transmission::SemiAuto;
            roof = false;
        } // 1, 5, 7, 11

        // Return requested "Car"
        Car {
            color: String::from(colors[(color - 1) as usize]),
            motor: motor,
            roof: roof,
            age: car_quality(miles),
        }
    }

    // Initialize a hash map for the car orders
    // - Key: Car order number, i32
    // - Value: Car order details, Car struct
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 11 cars
    // Corrected code: Use "for" loop to fulfill orders for 11 cars
    // Corrected code: "order" variable initialized and incremented in "for" loop

    // Start with zero miles
    let mut miles = 0;

    for order in 1..12 {
        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}
