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

struct User{
    name:String
}

impl User {
    fn hello(){
        println!("hello world");
    }
    fn say(&self) {
        println!("{}", self.name);
    }
}


fn main() {
    let user =  User{
        name: String::from("hello user")
    };
    User::hello();
    user.say();

    
    todo!("Display the message by using the println!() macro");
}
