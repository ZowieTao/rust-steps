fn main() {
    owner_clone_test();
}

fn owner_clone_test() {
    {
        let _mascot = String::from("ferris");
    }

    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        let _ferris = mascot;
        // ferris dropped here. The string data memory will be freed here.
        println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }

    // println!("{}", _mascot);

    // fn process(input: String) {}
    // fn caller() {
    //     let s = String::from("Hello, world!");
    //     process(s); // Ownership of the string in `s` moved into `process`
    //     process(s); // Error! ownership already moved.
    // }
    // caller();

    fn process(_input: u32) {}
    fn caller() {
        let n = 1u32;
        process(n); // Ownership of the number in `n` copied into `process`
        process(n); // `n` can be used again because it wasn't moved, it was copied.
    }

    caller();

    fn processStr(_s: String) {}
    fn main() {
        let s = String::from("Hello, world!");
        processStr(s.clone()); // Passing another value, cloned from `s`.
        processStr(s); // s was never moved and so it can still be used.
    }
    main()
}

fn borrowed_test() {
    fn what_is_borrowed() {
        let greeting = String::from("hello");
        let _greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
        println!("Greeting: {}", greeting); // We can still use `greeting`
    }
    what_is_borrowed();

    fn function_borrowed() {
        fn print_greeting(message: &String) {
            println!("Greeting: {}", message);
        }

        fn main() {
            let greeting = String::from("Hello");
            print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
            print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
        }
        main()
    }
    function_borrowed();

    fn change_borrowed() {
        fn change(message: &mut String) {
            message.push_str("!"); // We try to add a "!" to the end of our message
        }

        fn main() {
            let mut greeting = String::from("Hello");
            change(&mut greeting);
        }
        main();
    }
}
