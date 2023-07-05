// use std::fmt::format;

fn main() {
    generic_data_type();

    define_shared_behavior_with_trait();

    derive();

    trait_bounds();
}

fn generic_data_type() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let boolean: Point<bool> = Point { x: true, y: false };
    let integer: Point<i32> = Point { x: 1, y: 9 };
    let float: Point<f64> = Point { x: 1.7, y: 4.3 };
    let string_slice: Point<&str> = Point {
        x: "high",
        y: "low",
    };
    // let mix = Point { x: 1.7, y: false };

    #[derive(Debug)]
    struct Point2<T, U> {
        x: U,
        y: T,
    }

    let integer_and_boolean: Point2<bool, i32> = Point2 { x: 5, y: false };

    println!(
        "{:?} {:?} {:?} {:?} {:?} {:?}",
        boolean.x, integer.y, float, string_slice, integer_and_boolean.x, integer_and_boolean.y
    )
}

fn define_shared_behavior_with_trait() {
    trait Area {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Area for Circle {
        fn area(&self) -> f64 {
            use std::f64::consts::PI;
            PI * self.radius.powf(2.0)
        }
    }

    impl Area for Rectangle {
        fn area(&self) -> f64 {
            self.height * self.width
        }
    }

    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 10.0,
        height: 20.0,
    };

    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}

fn derive() {
    use std::fmt;

    trait Distance {
        fn distance_to_zero(&self) -> i32;
    }

    impl Distance for Point {
        fn distance_to_zero(&self) -> i32 {
            (self.x.pow(2) + self.y.pow(2)).pow(1 / 2)
        }
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 4, y: -3 };

    if p1 == p2 {
        // can't compare two Point values!
        println!("equal!");
    } else {
        println!("not equal!");
    }

    println!("{:}", p1); // can't print using the '{}' format specifier!
    println!("{:?}", p1); //  can't print using the '{:?}' format specifier!
    println!("{}", p1.distance_to_zero());
}

fn trait_bounds() {
    trait AsJson {
        fn as_json(&self) -> String;
    }

    // fn send_data_as_json(value: &impl AsJson) {
    fn send_data_as_json<T: AsJson>(value: &T) {
        println!("Sending JSON data to server...");
        println!("-> {}", value.as_json());
        println!("Done!\n");
    }

    struct Person {
        name: String,
        age: u8,
        favorite_fruit: String,
    }

    struct Dog {
        name: String,
        color: String,
        likes_petting: bool,
    }

    #[derive(Debug)]
    struct Cat {
        name: String,
        sharp_claws: bool,
    }

    impl AsJson for Person {
        fn as_json(&self) -> String {
            format!(
                r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
                self.name, self.age, self.favorite_fruit
            )
        }
    }

    impl AsJson for Dog {
        fn as_json(&self) -> String {
            format!(
                r#"{{"type": "dog", "name": "{}", "color": "{}", "likes petting": "{}"}}"#,
                self.name, self.color, self.likes_petting
            )
        }
    }

    let zowie = Person {
        name: String::from("Zowie Tao"),
        age: 24,
        favorite_fruit: String::from("apples"),
    };

    let beautiful_man = Dog {
        name: String::from("Beautiful Man"),
        color: String::from("Black"),
        likes_petting: true,
    };

    let kitty = Cat {
        name: String::from("Kitty"),
        sharp_claws: false,
    };

    send_data_as_json(&zowie);
    send_data_as_json(&beautiful_man);

    // send_data_as_json(&kitty);
    println!("cat {:?}", kitty)
}
