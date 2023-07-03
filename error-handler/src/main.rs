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

    person_task();

    result_type_handle_err();

    result_type_task();
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
    // assert_eq!(Some("doggy").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat"); // for none case
}

fn person_task() {
    struct Person {
        first: String,
        middle: Option<String>,
        last: String,
    }

    fn build_full_name(person: &Person) -> String {
        let mut full_name = String::new();
        full_name.push_str(&person.first);
        full_name.push_str(" ");

        // for middle in &person.middle {
        //     full_name.push_str(middle);
        //     full_name.push_str(" ")
        // }

        if let Some(middle) = &person.middle {
            full_name.push_str(&middle);
            full_name.push_str(" ")
        }

        full_name.push_str(&person.last);
        full_name
    }

    fn main() {
        let john = Person {
            first: String::from("James"),
            middle: Some(String::from("Oliver")),
            last: String::from("Smith"),
        };
        assert_eq!(build_full_name(&john), "James Oliver Smith");

        let alice = Person {
            first: String::from("Alice"),
            middle: None,
            last: String::from("Stevens"),
        };
        assert_eq!(build_full_name(&alice), "Alice Stevens");

        let bob = Person {
            first: String::from("Robert"),
            middle: Some(String::from("Murdock")),
            last: String::from("Jones"),
        };
        assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
    }

    main();
}

fn result_type_handle_err() {
    #[derive(Debug)]
    struct DivisionByZeroError;

    fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
        if divisor == 0.0 {
            Err(DivisionByZeroError)
        } else {
            Ok(dividend / divisor)
        }
    }

    fn main() {
        println!("{:?}", safe_division(9.0, 3.0));
        println!("{:?}", safe_division(4.0, 0.0));
        println!("{:?}", safe_division(0.0, 2.0));
    }

    main();
}

fn result_type_task() {
    use std::fs::File;
    use std::io::{Error, Read};
    use std::path::PathBuf;

    fn read_file_contents(path: PathBuf) -> Result<String, Error> {
        let mut string = String::new();

        // Access a file at a specified path
        // ---------------------------------
        // TODO #1:
        // - Pass variable to `file` variable on success, or
        // - Return from function early if there's an error
        let mut file: File = match File::open(path) {
            Ok(file_handle) => file_handle,
            Err(io_error) => return Err(io_error),
        };

        // Read file contents into `String` variable with `read_to_string`
        // ---------------------------------
        // Success path is already filled in
        // TODO #2: Return from function early if there's an error
        match file.read_to_string(&mut string) {
            Ok(_) => (),
            Err(io_error) => return Err(io_error),
        };

        // TODO #3: Return `string` variable as expected by function signature
        Ok(string)
    }

    fn main() {
        if read_file_contents(PathBuf::from("src/main.rs")).is_ok() {
            println!("The program found the main file.");
        }
        if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
            println!("The program reported an error for the file that doesn't exist.");
        }
    }

    main();
}
