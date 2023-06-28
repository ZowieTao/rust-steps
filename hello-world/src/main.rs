macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        // actually print logic
        std::io::Write::write_fmt(&mut std::io::stdout(), format_args!($($arg)*)).unwrap();
    });
}

struct User {
    name: String,
}

impl User {
    fn hello() {
        println!("hello world");
    }
    fn say(&self) {
        println!("{}", self.name);
    }
}

fn main() {
    let user = User {
        name: String::from("hello user"),
    };
    User::hello();
    user.say();

    println!(
        "The first letter of the English alphabet is {} and the last letter is {}.",
        'A', 'Z'
    );

    // VARIABLE
    // Declare a variable
    let a_number: i32;

    // Declare a second variable and bind the value
    let a_word: &str = "Ten";

    // Bind a value to the first variable
    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    // mutable and immutable
    // a_number = 20

    let mut b_number = 10;

    println!("The number b is {}.", b_number);

    b_number = 20;
    println!("Now the number b is {}.", b_number);

    // variable hidden

    // Declare first variable binding with name "shadow_num"
    let shadow_num: i32 = 5;

    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num: i32 = shadow_num + 5;

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num: i32 = shadow_num * 2;

    // let number_32 = 5.0;

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u64 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    println!("The shadow_num is {}.", shadow_num);

    todo!("Display the message by using the println!() macro");
}
