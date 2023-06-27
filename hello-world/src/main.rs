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
}
