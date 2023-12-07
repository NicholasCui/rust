fn main() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user = User {
        active: true,
        username: String::from("username"),
        email: String::from("someone@exmaple.com"),
        sign_in_count: 1,
    };

    let user1 = User {
        sign_in_count: 2,
        ..user
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    struct ArticleModule;

    let module = ArticleModule;


    // Partial Move
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };
    let email = user2.email;
    println!("email: {}", email);


    // oop
    #[derive(Debug, Default)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 实例所有权
        fn area(self) -> u32 {
            self.width * self.height
        }
        // 实例不可变引用
        fn area1(&self) -> u32 {
            self.width * self.height
        }
        // 实例可变引用
        fn area2(&mut self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("rect1 area: {}", rect1.area());

    impl Rectangle {
        fn numbers(row: u32, column: u32) -> u32 {
            row * column
        }
    }
    println!("rect1 numbers: {}", Rectangle::numbers(10, 10));

    // constructor
    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle {
                width,
                height,
            }
        }
    }

    let rect2 = Rectangle::new(10, 20);
    println!("rect2 area: {}", rect2.area());

    // default
    let react3: Rectangle = Default::default();
    let react4 = Rectangle::default();
    println!("rect3 area: {:?}", react3);
    println!("rect4 area: {:?}", react4);
}
