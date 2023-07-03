fn main() {
    {
        let mascot = String::from("ferris");
    }

    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        let ferris = mascot;
        // ferris dropped here. The string data memory will be freed here.
        println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }

    // println!("{}", mascot);

    // fn process(input: String) {}
    // fn caller() {
    //     let s = String::from("Hello, world!");
    //     process(s); // Ownership of the string in `s` moved into `process`
    //     process(s); // Error! ownership already moved.
    // }
    // caller();

    fn process(input: u32) {}
    fn caller() {
        let n = 1u32;
        process(n); // Ownership of the number in `n` copied into `process`
        process(n); // `n` can be used again because it wasn't moved, it was copied.
    }

    caller();

    fn processStr(s: String) {}
    fn main() {
        let s = String::from("Hello, world!");
        processStr(s.clone()); // Passing another value, cloned from `s`.
        processStr(s); // s was never moved and so it can still be used.
    }
    main()
}
