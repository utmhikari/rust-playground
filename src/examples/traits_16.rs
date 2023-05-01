pub fn derive() {
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }

    struct Seconds(i32);

    let _one_seconds = Seconds(1);
    let foot = Inches(12);
    println!("one foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("one foot is {} than one meter", cmp);
}

pub fn returning_traits_with_dyn() {
    struct Sheep {}
    struct Cow {}
    trait Animal {
        fn noise(&self) -> &'static str;
    }
    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaah"
        }
    }
    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "mooooo"
        }
    }
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("the animal says {}", animal.noise())
}

pub fn operator_overloading() {
    use std::ops;
    struct Foo;
    struct Bar;
    #[derive(Debug)]
    struct FooBar;
    #[derive(Debug)]
    struct BarFoo;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;
        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");
            FooBar
        }
    }
    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;
        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");
            BarFoo
        }
    }
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

pub fn drop_() {
    struct Droppable {
        name: &'static str,
    }
    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> dropping {}", self.name);
        }
    }
    let _a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("exit block B");
        }
        println!("just exited block B");

        println!("exit block A");
    }
    drop(_a);
    println!("exit block")
}

pub fn iterators() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }
    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;
            self.curr = self.next;
            self.next = current + self.next;
            Some(current)
        }
    }
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
    let mut sequence = 0..3;
    println!("for consecutive next calls on 0 .. 3:");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("iterate through 0 .. 3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("the first four terms of the Fibonacci sequence are:");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("the next four terms of the Fibonacci sequence are:");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

pub fn impl_trait() {
    fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line| {
                line.map(|line| {
                    line.split(',')
                        .map(|entry| String::from(entry.trim()))
                        .collect()
                })
            })
            .collect()
    }
    fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("ok")
}

pub fn clone_() {
    #[derive(Debug, Clone, Copy)]
    struct Unit;
    #[derive(Clone, Debug)]
    struct Pair(Box<i32>, Box<i32>);

    let unit = Unit;
    let copied_unit = unit;
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    println!("clone: {:?}", cloned_pair);
}

pub fn supertraits() {
    trait Person {
        fn name(&self) -> String;
    }
    trait Student: Person {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favourite language is {}. My git-username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username(),
        )
    }
}

pub fn disambiguating_overlapping_traits() {
    trait UsernameWidget {
        fn get(&self) -> String;
    }
    trait AgeWidget {
        fn get(&self) -> u8;
    }
    struct Form {
        username: String,
        age: u8,
    }
    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }
    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}

pub fn traits_16() {
    // derive();
    // returning_traits_with_dyn();
    // operator_overloading();
    // drop_();
    // iterators();
    // impl_trait();
    // clone_();
    // supertraits();
    disambiguating_overlapping_traits();
}
