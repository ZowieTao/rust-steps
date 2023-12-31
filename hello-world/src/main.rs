use std::fmt;

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

struct MyString {
    data: Vec<u8>,
}

impl MyString {
    fn from(s: &str) -> MyString {
        MyString {
            data: s.as_bytes().to_vec(),
        }
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.data).unwrap()
    }
}

// println to {}
impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// interface like
trait ToMyString {
    fn to_my_string(&self) -> MyString;
}

impl ToMyString for str {
    fn to_my_string(&self) -> MyString {
        MyString::from(self)
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

    println!("The shadow_num is {}.", shadow_num);

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u64 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger: bool = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';

    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1: &str = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    let string_0: String = "hello".to_string();

    println!(
        "{}, {} is a {}{}{}{}. ",
        string_0, smiley_face, character_1, string_1, character_2, string_2
    );

    // Tuple of length 3
    let mut tuple_e: (&str, f32, bool) = ("E", 5.0, 5.0 > 4.0);

    tuple_e.2 = 4.0 > 5.0;

    // Use tuple indexing and show the values of the elements in the tuple
    println!(
        "Is '{}' the {}th letter of the alphabet? {}",
        tuple_e.0, tuple_e.1, tuple_e.2
    );

    // Classic struct with named fields
    struct Student {
        name: MyString,
        level: u8,
        remote: bool,
    }

    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    let user_1 = Student {
        name: MyString::from("Jane Wang"),
        remote: true,
        level: 2,
    };

    let user_2 = Student {
        name: MyString::from("Zowie Tao"),
        level: 5,
        remote: false,
    };

    println!("{}", user_2.name);

    // shadow struct
    let user_2 = Student {
        //  use trail way to convert &str to MyString
        name: "HandsomeBoy Zowie Tao".to_my_string(),
        level: 5,
        remote: false,
    };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
    );
    println!(
        "{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}",
        user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
    );

    // enum WebEvent {
    //     // An enum variant can be like a unit struct without fields or data types
    //     WELoad,
    //     // An enum variant can be like a tuple struct with data types but no named fields
    //     WEKeys(String, char),
    //     // An enum variant can be like a classic struct with named fields and their data types
    //     WEClick { x: i64, y: i64 },
    // }

    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    // Define the WebEvent enum variants to use the data from the structs
    // and a boolean type for the page Load variant
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    // Instantiate a MouseClick struct and bind the coordinate values
    let click: MouseClick = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys: KeyPress = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load: WebEvent = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click: WebEvent = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key: WebEvent = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );

    println!(
        "\nWebEvent enum structure: use {{:?}} \n\n {:?} \n\n {:?} \n\n {:?}",
        we_load, we_click, we_key
    );

    let formal: &str = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));

    todo!("Display the message by using the println!() macro");
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0;
    }
    return num / 5;
}
