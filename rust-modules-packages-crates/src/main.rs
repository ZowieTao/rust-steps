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
    split_code_into_modules();

    third_party_crates();

    exercise_visibility();

    exercise_modules();
}

fn split_code_into_modules() {
    let mut user = authentication::User::new("zowie", "super-secret");

    println!("The username is: {}", user.get_username());
    println!("The password is: {}", user.get_password_hash());
    user.set_password("other-super-secret");
    println!("The password is: {}", user.get_password_hash());
}

fn third_party_crates() {
    use regex::Regex;

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2023-07-06"));
}

fn exercise_visibility() {
    mod car_factory {
        pub fn build_car() {
            println!("Honk honk!");
        }
    }

    car_factory::build_car();
}

fn exercise_modules() {
    mod text_processing {

        pub mod letters {
            pub fn count_letters(text: &str) -> usize {
                text.chars().filter(|ref c| c.is_alphabetic()).count()
            }
        }

        pub mod numbers {
            pub fn count_numbers(text: &str) -> usize {
                text.chars().filter(|ref c| c.is_numeric()).count()
            }
        }
    }

    fn count_letters_and_numbers(text: &str) -> (usize, usize) {
        let number_of_letters = text_processing::letters::count_letters(text);
        let number_of_numbers = text_processing::numbers::count_numbers(text);
        (number_of_letters, number_of_numbers)
    }

    fn main() {
        assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
        assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
        assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
    }

    main()
}
