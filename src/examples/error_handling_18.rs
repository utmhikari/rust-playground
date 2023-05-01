use crate::examples::traits_16::derive;
use std::num::ParseIntError;

pub fn panic_() {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            panic!("aaaaaaaaaa");
        }
        println!("some refreshing {} is all i need", beverage);
    }
    drink("water");
    drink("lemonade");
}

pub fn abort_and_unwind() {
    #[cfg(panic = "unwind")]
    fn ah() {
        println!("spit it out");
    }

    #[cfg(not(panic = "unwind"))]
    fn ah() {
        println!("this is not your party, run")
    }

    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            ah();
        }
        println!("some refreshing {} is all i need", beverage);
    }

    drink("water");
    drink("lemonade");
}

pub fn option_and_unwrap() {
    // fn give_adult(drink: Option<&str>) {
    //     match drink {
    //         Some("lemonade") => println!("yuck, to sugary"),
    //         Some(inner) => println!("{}? nice!", inner),
    //         None => println!("no drink? on well.."),
    //     }
    // }
    // fn drink(drink: Option<&str>) {
    //     let inside = drink.unwrap();
    //     if inside == "lemonade" {
    //         panic!("aaaaaaaaa");
    //     }
    //     println!("i love {}s!!!", inside);
    // }
    // let water = Some("water");
    // let lemonade = Some("lemonade");
    // let void = None;
    // give_adult(water);
    // give_adult(lemonade);
    // give_adult(void);
    // let coffee = Some("coffee");
    // let nothing = None;
    // drink(coffee);
    // drink(nothing);

    // =========================== unpacking options with ? ===========================
    // struct Person {
    //     job: Option<Job>,
    // }
    // #[derive(Clone, Copy)]
    // struct Job {
    //     phone_number: Option<PhoneNumber>,
    // }
    // #[derive(Clone, Copy)]
    // struct PhoneNumber {
    //     area_code: Option<u8>,
    //     number: u32,
    // }
    // impl Person {
    //     fn work_phone_area_code(&self) -> Option<u8> {
    //         self.job?.phone_number?.area_code
    //     }
    // }
    // let p = Person {
    //     job: Some(Job {
    //         phone_number: Some(PhoneNumber {
    //             area_code: Some(61),
    //             number: 4392222,
    //         }),
    //     }),
    // };
    // assert_eq!(p.work_phone_area_code(), Some(61));

    // =========================== combinators: map ====================================
    // #![allow(dead_code)]
    // #[derive(Debug)]
    // enum Food {
    //     Apple,
    //     Carrot,
    //     Potato,
    // }
    // #[derive(Debug)]
    // struct Peeled(Food);
    // #[derive(Debug)]
    // struct Chopped(Food);
    // #[derive(Debug)]
    // struct Cooked(Food);
    // fn peel(food: Option<Food>) -> Option<Peeled> {
    //     match food {
    //         Some(food) => Some(Peeled(food)),
    //         None => None,
    //     }
    // }
    // fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    //     match peeled {
    //         Some(Peeled(food)) => Some(Chopped(food)),
    //         None => None,
    //     }
    // }
    // fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    //     match chopped {
    //         Some(Chopped(food)) => Some(Cooked(food)),
    //         None => None,
    //     }
    // }
    // fn process(food: Option<Food>) -> Option<Cooked> {
    //     food.map(|f| Peeled(f))
    //         .map(|Peeled(f)| Chopped(f))
    //         .map(|Chopped(f)| Cooked(f))
    // }
    // fn eat(food: Option<Cooked>) {
    //     match food {
    //         Some(food) => println!("mmm, i love {:?}", food),
    //         None => println!("not edible"),
    //     }
    // }
    // let apple = Some(Food::Apple);
    // let carrot = Some(Food::Carrot);
    // let potato = None;
    // // let cooked_apple = cook(chop(peel(apple)));
    // let cooked_apple = process(apple);
    // let cooked_carrot = cook(chop(peel(carrot)));
    // let cooked_potato = process(potato);
    // eat(cooked_apple);
    // eat(cooked_carrot);
    // eat(cooked_potato);

    // ===================== combinators: and_then =====================
    // #![allow(dead_code)]
    // #[derive(Debug)]
    // enum Food {
    //     CordonBleu,
    //     Steak,
    //     Sushi,
    // }
    // #[derive(Debug)]
    // enum Day {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    // }
    // fn have_ingredients(food: Food) -> Option<Food> {
    //     match food {
    //         Food::Sushi => None,
    //         _ => Some(food),
    //     }
    // }
    // fn have_recipe(food: Food) -> Option<Food> {
    //     match food {
    //         Food::CordonBleu => None,
    //         _ => Some(food),
    //     }
    // }
    // fn cookable_v1(food: Food) -> Option<Food> {
    //     match have_recipe(food) {
    //         None => None,
    //         Some(food) => have_ingredients(food),
    //     }
    // }
    // fn cookable_v2(food: Food) -> Option<Food> {
    //     have_recipe(food).and_then(have_ingredients)
    // }
    // fn eat(food: Food, day: Day) {
    //     match cookable_v2(food) {
    //         Some(food) => println!("yay! on {:?} we get to eat {:?}", day, food),
    //         None => println!("oh no, we don't get to eat on {:?}", day),
    //     }
    // }
    // let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
    // eat(cordon_bleu, Day::Monday);
    // eat(steak, Day::Tuesday);
    // eat(sushi, Day::Wednesday);

    // ========================= unpacking options and defaults ==========================
    #[derive(Debug)]
    enum Fruit {
        Apple,
        Orange,
        Banana,
        Kiwi,
        Lemon,
    }
    // ---------------------------------------
    // let apple = Some(Fruit::Apple);
    // let orange = Some(Fruit::Orange);
    // let no_fruit: Option<Fruit> = None;
    // let first_available_fruit = no_fruit.or(orange).or(apple);
    // println!("first available fruit: {:?}", first_available_fruit);
    // -------------------------------------
    // let apple = Some(Fruit::Apple);
    // let no_fruit: Option<Fruit> = None;
    // let get_kiwi_as_fallback = || {
    //     println!("providing kiwi as fallback");
    //     Some(Fruit::Kiwi)
    // };
    // let get_lemon_as_fallback = || {
    //     println!("providing lemon as fallback");
    //     Some(Fruit::Lemon)
    // };
    // let first_available_fruit = no_fruit
    //     .or_else(get_kiwi_as_fallback)
    //     .or_else(get_lemon_as_fallback);
    // println!("first available fruit: {:?}", first_available_fruit);
    // -----------------------------------------
    // let mut my_fruit: Option<Fruit> = None;
    // let apple = Fruit::Apple;
    // let first_available_fruit = my_fruit.get_or_insert(apple);
    // println!("first available fruit: {:?}", first_available_fruit);
    // println!("my fruit is: {:?}", my_fruit)
    // -------------------------------------
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_callback = || {
        println!("providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_callback);
    println!("first available fruit: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_callback);
    println!("should be apple: {:?}", should_be_apple);
    println!("my_apple is: {:?}", my_apple);
}

pub fn result_() {
    // use std::num::ParseIntError;
    // fn test_result() -> Result<(), ParseIntError> {
    //     let number_str = "10b";
    //     let number = match number_str.parse::<i32>() {
    //         Ok(number) => number,
    //         Err(e) => return Err(e),
    //     };
    //     println!("{}", number);
    //     Ok(())
    // }
    // let result = test_result();
    // println!("result is {:?}", result);

    // =========================== map for Result ===========================
    // use std::num::ParseIntError;
    // fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    //     match first_number_str.parse::<i32>() {
    //         Ok(first_number) => match second_number_str.parse::<i32>() {
    //             Ok(second_number) => Ok(first_number * second_number),
    //             Err(e) => Err(e),
    //         },
    //         Err(e) => Err(e),
    //     }
    // }
    // fn print(result: Result<i32, ParseIntError>) {
    //     match result {
    //         Ok(n) => println!("n is {}", n),
    //         Err(e) => println!("error: {}", e),
    //     }
    // }
    // let twenty = multiply("10", "2");
    // print(twenty);
    // let tt = multiply("t", "2");
    // print(tt);

    // ============================ alias for Result ==========================
    // use std::num::ParseIntError;
    // type AliasedResult<T> = Result<T, ParseIntError>;
    // fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    //     first_number_str.parse::<i32>().and_then(|first_number| {
    //         second_number_str
    //             .parse::<i32>()
    //             .map(|second_number| first_number * second_number)
    //     })
    // }
    // fn print(result: Result<i32, ParseIntError>) {
    //     match result {
    //         Ok(n) => println!("n is {}", n),
    //         Err(e) => println!("error: {}", e),
    //     }
    // }
    // print(multiply("10", "2"));
    // print(multiply("t", "2"));

    // ================================= early returns ==================================
    // use std::num::ParseIntError;
    // fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    //     let first_number = match first_number_str.parse::<i32>() {
    //         Ok(first_number) => first_number,
    //         Err(e) => return Err(e),
    //     };
    //     let second_number = match second_number_str.parse::<i32>() {
    //         Ok(second_number) => second_number,
    //         Err(e) => return Err(e),
    //     };
    //     Ok(first_number * second_number)
    // }
    // fn print(result: Result<i32, ParseIntError>) {
    //     match result {
    //         Ok(n) => println!("n is {}", n),
    //         Err(e) => println!("error: {}", e),
    //     }
    // }
    // print(multiply("10", "2"));
    // print(multiply("t", "2"));

    // ============================ introducing ? =================================
    // fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    //     let first_number = first_number_str.parse::<i32>()?;
    //     let second_number = second_number_str.parse::<i32>()?;
    //     Ok(first_number * second_number)
    // }
    // fn multiply_try(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    //     let first_number = try!(first_number_str.parse::<i32>());
    //     let second_number = try!(second_number_str.parse::<i32>());
    //     Ok(first_number * second_number)
    // }
}

pub fn multiple_error_types() {
    // =========================== pulling results out of options =========================
    // use std::num::ParseIntError;
    // fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    //     let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
    //     opt.map_or(Ok(None), |r| r.map(Some))
    // }
    // let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    // let strings = vec!["tofu", "93", "18"];
    // println!("the first doubled is {:?}", double_first(numbers));
    // println!("the first doubled is {:?}", double_first(empty));
    // println!("the first doubled is {:?}", double_first(strings));

    // ============================= defining an error type ===================================
    // use std::fmt;
    // type Result<T> = std::result::Result<T, DoubleError>;
    // #[derive(Debug, Clone)]
    // struct DoubleError;
    // impl fmt::Display for DoubleError {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "invalid first item to double")
    //     }
    // }
    // fn double_first(vec: Vec<&str>) -> Result<i32> {
    //     vec.first()
    //         .ok_or(DoubleError)
    //         .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i))
    // }
    // fn print(result: Result<i32>) {
    //     match result {
    //         Ok(n) => println!("the first doubled is {}", n),
    //         Err(e) => println!("error: {}", e),
    //     }
    // }
    // let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    // let strings = vec!["tofu", "93", "18"];
    // print(double_first(numbers));
    // print(double_first(empty));
    // print(double_first(strings));

    // =============================== Boxing Errors ==============================
    // use std::error;
    // use std::fmt;
    // type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
    // #[derive(Debug, Clone)]
    // struct EmptyVec;
    // impl fmt::Display for EmptyVec {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "invalid first item to double")
    //     }
    // }
    // impl error::Error for EmptyVec {}
    // fn double_first(vec: Vec<&str>) -> Result<i32> {
    //     vec.first()
    //         .ok_or_else(|| EmptyVec.into())
    //         .and_then(|s| s.parse::<i32>().map_err(|e| e.into()).map(|i| 2 * i))
    // }
    // fn print(result: Result<i32>) {
    //     match result {
    //         Ok(n) => println!("the first doubled is {}", n),
    //         Err(e) => println!("error: {}", e),
    //     }
    // }
    // let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    // let strings = vec!["tofu", "93", "18"];
    // print(double_first(numbers));
    // print(double_first(empty));
    // print(double_first(strings));

    // ============================ other uses of ? ===============================
    // use std::error;
    // use std::fmt;
    // type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
    // #[derive(Debug, Clone)]
    // struct EmptyVec;
    // impl fmt::Display for EmptyVec {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "invalid first item to double")
    //     }
    // }
    // impl error::Error for EmptyVec {}
    // fn double_first(vec: Vec<&str>) -> Result<i32> {
    //     let first = vec.first().ok_or(EmptyVec)?;
    //     let parsed = first.parse::<i32>()?;
    //     Ok(2 * parsed)
    // }
    // fn print(result: Result<i32>) {
    //     match result {
    //         Ok(n) => println!("the first doubled is {}", n),
    //         Err(e) => println!("error: {}", e),
    //     }
    // }
    // let numbers = vec!["42", "93", "18"];
    // let empty = vec![];
    // let strings = vec!["tofu", "93", "18"];
    // print(double_first(numbers));
    // print(double_first(empty));
    // print(double_first(strings));

    // ============================ wrapping errors ===================================
    use std::error;
    use std::error::Error;
    use std::fmt;
    use std::num::ParseIntError;
    type Result<T> = std::result::Result<T, DoubleError>;
    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        Parse(ParseIntError),
    }
    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                DoubleError::Parse(..) => {
                    write!(f, "the provided string could not be parsed to int")
                }
            }
        }
    }
    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }
    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("the first doubled is {}", n),
            Err(e) => {
                println!("error: {}", e);
                if let Some(source) = e.source() {
                    println!("\tcaused by: {}", source)
                }
            }
        }
    }
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];
    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

pub fn iterating_over_results() {
    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .filter_map(|s| s.parse::<i32>().ok())
    //     .collect();
    // println!("results: {:?}", numbers);
    // ----------------------------------------------------------------
    // let strings = vec!["tofu", "93", "18"];
    // let mut errors = vec![];
    // let numbers: Vec<_> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<u8>())
    //     .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
    //     .collect();
    // println!("numbers: {:?}", numbers);
    // println!("errors: {:?}", errors);
    // ----------------------------------------------------------------
    // let strings = vec!["tofu", "93", "18"];
    // let numbers: Result<Vec<_>, _> = strings
    //     .into_iter()
    //     .map(|s| s.parse::<i32>())
    //     .collect();
    // println!("results: {:?}", numbers);
    // ----------------------------------------------------------------
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);
}

pub fn error_handling_18() {
    // panic_();
    // abort_and_unwind();
    // option_and_unwrap();
    // result_();
    // multiple_error_types();
    iterating_over_results();
}
