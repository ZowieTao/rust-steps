mod authentication {
    pub struct User {
        username: String,
        password_hash: u64,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password),
            }
        }
        pub fn get_username(&self) -> &String {
            &self.username
        }
        pub fn get_password_hash(&self) -> u64 {
            self.password_hash
        }
        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(new_password)
        }
    }
    fn hash_password(input: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher: DefaultHasher = DefaultHasher::new();
        input.hash(&mut hasher);
        hasher.finish()
    }
}

fn main() {
    let mut user = authentication::User::new("zowie", "super-secret");

    println!("The username is: {}", user.get_username());
    println!("The password is: {}", user.get_password_hash());
    user.set_password("other-super-secret");
    println!("The password is: {}", user.get_password_hash());
}
