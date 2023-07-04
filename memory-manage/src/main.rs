fn main() {
    owner_clone_test();

    borrowed_test();

    validate_references_with_lifetimes();
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
        // println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
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
    change_borrowed();

    fn borrowed_mut_reference() {
        fn scene1() {
            let mut value = String::from("hello");

            let ref1: &mut String = &mut value;
            // let ref2: &mut String = &mut value;
            let ref2 = ref1.clone();

            println!("{}, {}", ref1, ref2);
        }
        scene1();

        fn scene2() {
            // let mut value = String::from("hello");

            // let ref1 = &value;
            // let ref2 = &mut value;

            // println!("{}, {}", ref1, ref2);
        }
        scene2();
    }
    borrowed_mut_reference();
}

fn validate_references_with_lifetimes() {
    // fn scene1() {
    //     let x;
    //     // let y = 42;
    //     {
    //         let y = 42;
    //         x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    //     }
    //     println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
    // }
    // scene1();

    fn scene2() {
        fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let magic1 = String::from("abracadabra!");
        let magic2 = String::from("shazam!");

        let result = longest_word(&magic1, &magic2);
        println!("The longest magic word is {}", result);
    }
    scene2();

    // fn scene3() {
    //     fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    //         if x.len() > y.len() {
    //             x
    //         } else {
    //             y
    //         }
    //     }

    //     let magic1 = String::from("abracadabra!");
    //     // let magic2 = String::from("shazam!");

    //     let result;
    //     {
    //         let magic2 = String::from("shazam!");
    //         result = longest_word(&magic1, &magic2);
    //     }
    //     println!("The longest magic word is {}", result);
    // }
    // scene3();

    fn scene4() {
        // Whenever a struct or enum holds a reference in one of its fields, we must annotate that type definition with the lifetime of each reference that it carries along with it.
        #[derive(Debug)]
        struct Highlight<'document>(&'document str);

        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        println!("{:?}", fox);
        println!("{:?}", dog);
    }
    scene4();

    fn scene5() {
        #[derive(Debug)]
        struct Highlight<'document>(&'document str);

        fn erase(_: String) {}

        let text: String = String::from("The quick brown fox jumps over the lazy dog.");
        let fox: Highlight<'_> = Highlight(&text[4..19]);
        let dog: Highlight<'_> = Highlight(&text[35..43]);

        // erase(text);

        println!("{:?}", fox);
        println!("{:?}", dog);
    }
    scene5()
}
