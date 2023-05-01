use crate::examples::attributes_13::dead_code;
use std::rc::Rc;
use std::sync::Arc;

pub fn box_stack_and_heap() {
    use std::mem;
    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn boxed_origin() -> Box<Point> {
        Box::new(Point { x: 0.0, y: 0.0 })
    }

    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });
    let boxed_point: Box<Point> = Box::new(origin());
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );
    println!(
        "boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    let unboxed_point: Point = *boxed_point;
    println!(
        "unboxed point occupies {} byted on the stack",
        mem::size_of_val(&unboxed_point)
    );
}

pub fn vectors() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collected (0..10) into: {:?}", collected_iterator);
    let mut xs = vec![1i32, 2, 3];
    println!("initial vector: {:?}", xs);
    println!("push 4 into the vector");
    xs.push(4);
    println!("vector: {:?}", xs);
    println!("vector length: {:?}", xs.len());
    println!("second element: {}", xs[1]);
    println!("pop last element: {:?}", xs.pop());
    // println!("fourth element: {}", xs[3]);
    println!("contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }
    for (i, x) in xs.iter().enumerate() {
        println!("in position {} we have value {}", i, x);
    }
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("updated vector: {:?}", xs);
}

pub fn strings() {
    let pangram: &'static str = "the quick brown fox jumped over the lazy dog";
    println!("pangram: {}", pangram);
    println!("words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("used characters: {}", trimmed_str);
    let alice = String::from("i like dogs");
    let bob: String = alice.replace("dog", "cat");
    println!("alice says: {}", alice);
    println!("bob says: {}", bob);
}

pub fn option() {
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }
    fn try_division(dividend: i32, divisor: i32) {
        match checked_division(dividend, divisor) {
            None => println!("{} / {} failed", dividend, divisor),
            Some(quotient) => {
                println!("{} / {} = {}", dividend, divisor, quotient)
            }
        }
    }
    try_division(4, 2);
    try_division(1, 0);
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;
    let optional_float = Some(0f32);
    println!(
        "{:?} unwraps to {:?}",
        optional_float,
        optional_float.unwrap()
    );
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}

pub fn result() {
    mod checked {
        #[derive(Debug)]
        pub enum MathError {
            DivisionByZero,
            NonPositiveLogarithm,
            NegativeSquareRoot,
        }
        pub type MathResult = Result<f64, MathError>;
        pub fn div(x: f64, y: f64) -> MathResult {
            if y == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(x / y)
            }
        }
        pub fn sqrt(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }
        pub fn ln(x: f64) -> MathResult {
            if x <= 0.0 {
                Err(MathError::NonPositiveLogarithm)
            } else {
                Ok(x.ln())
            }
        }
        pub fn op_(x: f64, y: f64) -> MathResult {
            let ratio = div(x, y)?;
            let ln = ln(ratio)?;
            sqrt(ln)
        }
    }
    pub fn op(x: f64, y: f64) -> f64 {
        // match checked::div(x, y) {
        //     Err(why) => panic!("{:?}", why),
        //     Ok(ratio) => match checked::ln(ratio) {
        //         Err(why) => panic!("{:?}", why),
        //         Ok(ln) => match checked::sqrt(ln) {
        //             Err(why) => panic!("{:?}", why),
        //             Ok(sqrt) => sqrt,
        //         },
        //     },
        // }
        match checked::op_(x, y) {
            Err(why) => panic!(
                "{:?}",
                match why {
                    checked::MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                    checked::MathError::DivisionByZero => "division by zero",
                    checked::MathError::NegativeSquareRoot => "square root of negative number",
                }
            ),
            Ok(value) => {
                println!("{}", value);
                value
            }
        }
    }
    println!("{}", op(1.0, 10.0));
}

pub fn panic_() {
    fn division(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            panic!("division by zero")
        } else {
            dividend / divisor
        }
    }
    let _x = Box::new(0i32);
    division(3, 0);
    println!("this point won't be reached")
}

pub fn hashmap() {
    use std::collections::HashMap;
    fn call(number: &str) -> &str {
        match number {
            "798-1364" => "the call cannot be completed as dialed",
            "645-7689" => "this is pizza",
            _ => "hello world",
        }
    }
    let mut contacts = HashMap::new();
    contacts.insert("daniel", "798-1364");
    contacts.insert("ashley", "645-7689");
    contacts.insert("katie", "435-8291");
    contacts.insert("robert", "956-1745");
    match contacts.get(&"daniel") {
        Some(&number) => println!("calling daniel: {}", call(number)),
        _ => println!("don't have daniel's number"),
    }
    contacts.insert("daniel", "164-6743");
    match contacts.get(&"ashley") {
        Some(&number) => println!("calling ashley: {}", call(number)),
        _ => println!("don't have ashley's number"),
    }
    contacts.remove(&"ashley");
    for (contact, &number) in contacts.iter() {
        println!("calling {}: {}", contact, call(number));
    }

    // ======================== alternate/custom key types =======================
    #[derive(PartialEq, Eq, Hash)]
    struct Account<'a> {
        username: &'a str,
        password: &'a str,
    }
    struct AccountInfo<'a> {
        name: &'a str,
        email: &'a str,
    }
    type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;
    fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
        println!("logon -> username: {}, password: {}", username, password);
        let logon = Account { username, password };
        match accounts.get(&logon) {
            Some(account_info) => {
                println!(
                    "logon successfully -> name: {}, email: {}",
                    account_info.name, account_info.email
                );
            }
            _ => println!("logon failed"),
        }
    }
    let mut accounts: Accounts = HashMap::new();
    let account = Account {
        username: "j.everyman",
        password: "password123",
    };
    let account_info = AccountInfo {
        name: "john everyman",
        email: "j.everyman@gmail.com",
    };
    accounts.insert(account, account_info);
    try_logon(&accounts, "j.everyman", "psasword123");
    try_logon(&accounts, "j.everyman", "password123");

    // ========================== hashset ==============================
    use std::collections::HashSet;
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();
    assert!(a.insert(4));
    assert!(a.contains(&4));
    // assert!(b.insert(4), "value 4 is already in hashset b");
    b.insert(5);
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("diff: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!(
        "intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );
    println!(
        "symmetric diff: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );
}

pub fn rc() {
    use std::rc::Rc;
    let rc_examples = "rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("reference count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("reference count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("reference count of rc_a: {}", Rc::strong_count(&rc_a));

            println!("rc_a & rc_b are equal: {}", rc_a.eq(&rc_b));

            println!("length of the value inside rc_a: {}", rc_a.len());
            println!("value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }
        println!("reference count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
}

pub fn arc() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;
    let s = "the same apple".to_string();
    let apple = Arc::new(s);
    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?} ({})", apple, Arc::strong_count(&apple));
        });
    }
    thread::sleep(Duration::from_secs(1));
    println!("LAST -> {:?} ({})", apple, Arc::strong_count(&apple));
}

pub fn std_library_types_19() {
    // box_stack_and_heap();
    // vectors();
    // strings();
    // option();
    // result();
    // panic_();
    // hashmap();
    // rc();
    arc();
}
