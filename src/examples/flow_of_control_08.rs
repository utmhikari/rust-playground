pub fn if_else() {
    // let n = 5;
    // if n < 0 {
    //     print!("{} is negative", n);
    // } else if n > 0 {
    //     print!("{} is positive", n);
    // } else {
    //     print!("{} is zero", n);
    // }
    // let big_n = if n < 10 && n > -10 {
    //     println!(", and is a small number, increase ten-fold");
    //     10 * n
    // } else {
    //     println!(", and is a big number, halve the number");
    //     n / 2
    // };
    // println!("{} -> {}", n, big_n)
}

pub fn loops() {
    // =========================== loop ================================
    // let mut count = 0u32;
    // println!("let's count until infinity");
    // loop {
    //     count += 1;
    //     if count == 3 {
    //         println!("three");
    //         continue;
    //     }
    //     println!("{}", count);
    //     if count == 5 {
    //         println!("ok, that's through");
    //         break;
    //     }
    // }

    // =========================== loop.Nesting and Labels ================================
    // #![allow(unreachable_code)]
    // 'outer: loop {
    //     println!("entered the outer loop");
    //     'inner: loop {
    //         println!("entered the inner loop");
    //         break 'outer;
    //     }
    //     println!("never reach");
    // }
    // println!("exited outer loop")

    // =========================== loop.Returning from loops ================================
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // assert_eq!(result, 20);
}

pub fn while_loop() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

pub fn for_and_range() {
    // =========================== for .. in ================================
    // for n in 1 .. 101 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }
    // for n in 1 ..= 100 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }

    // =========================== for & iterators ================================
    // let names = vec!["bob", "frank", "ferris"];
    // for name in names.iter() {
    //     match name {
    //         &"ferris" => println!("there is a rustacean among us!"),
    //         _ => println!("hello {}", name),
    //     }
    // }
    // println!("names: {:?}", names);
    // let mut names = vec!["bob", "frank", "ferris"];
    // for name in names.iter_mut() {
    //     *name = match name {
    //         &mut "ferris" => "there is a rustacean among us",
    //         _ => "hello",
    //     }
    // }
    // println!("names: {:?}", names);
}

pub fn match_case() {
    // let number = 13;
    // println!("tell me about {}", number);
    // match number {
    //     1 => println!("one!"),
    //     2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
    //     13..=19 => println!("a teen"),
    //     _ => println!("ain't special"),
    // }
    // let boolean = true;
    // let binary = match boolean {
    //     false => 0,
    //     true => 1,
    // };
    // println!("{} => {}", boolean, binary);

    // =========================== match.destructing.tuples ================================
    // let triple = (0, -2, 3);
    // println!("tell me about {:?}", triple);
    // match triple {
    //     (0, y, z) => println!("first is `0`, `y` is {:?}, and `z` is {:?}", y, z),
    //     (1, ..) => println!("first is `1`, rest does not matter"),
    //     (.., 2) => println!("last is `2`, rest does not matter"),
    //     (3, .., 4) => println!("first is `3`, last is `4`, rest does not matter"),
    //     _ => println!("does not matter anywhere"),
    // }

    // =========================== match.destructing.arrays/slices ================================
    // let array = [1, -2, 6];
    // match array {
    //     [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
    //     [1, _, third] => println!("array[0] = 0, array[2] = {}", third),
    //     [-1, second, ..] => println!("array[0] = -1, array[1] = {}, others ignored", second),
    //     [3, second, tail @ ..] => println!(
    //         "array[0] = 3, array[1] = {}, and the other elements were {:?}",
    //         second, tail
    //     ),
    //     [first, middle @ .., last] => println!(
    //         "array[0] = {}, middle = {:?}, array[-1] = {}",
    //         first, middle, last
    //     ),
    // }

    // =========================== match.destructing.enums ================================
    // #[allow(dead_code)]
    // enum Color {
    //     Red,
    //     Blue,
    //     Green,
    //     RGB(u32, u32, u32),
    //     HSV(u32, u32, u32),
    //     HSL(u32, u32, u32),
    //     CMY(u32, u32, u32),
    //     CMYK(u32, u32, u32, u32),
    // }
    // let color = Color::RGB(122, 17, 40);
    // println!("what color is it?");
    // match color {
    //     Color::Red => println!("color red"),
    //     Color::Blue => println!("color blue"),
    //     Color::Green => println!("color green"),
    //     Color::RGB(r, g, b) => println!("red: {}, green: {}, blue: {}", r, g, b),
    //     Color::HSV(h, s, v) => println!("hue: {}, satuation: {}, value: {}", h, s, v),
    //     Color::HSL(h, s, l) => println!("hue: {}, satuation: {}, lightness: {}", h, s, l),
    //     Color::CMY(c, m, y) => println!("cyan: {}, magenta: {}, yellow: {}", c, m, y),
    //     Color::CMYK(c, m, y, k) => println!(
    //         "cyan: {}, magenta: {}, yellow: {}, key(black): {}!",
    //         c, m, y, k
    //     ),
    // }

    // =========================== match.destructing.pointers/ref ================================
    // let reference = &4;
    // match reference {
    //     &val => println!("got a value via destructuring: {:?}", val),
    // }
    // match *reference {
    //     val => println!("got a value via destructuring: {:?}", val),
    // }
    // let _not_a_reference = 3;
    // let ref _is_a_reference = 3;
    // let value = 5;
    // let mut mut_value = 6;
    // match value {
    //     ref r => println!("got a value via destructuring: {:?}", r),
    // }
    // match mut_value {
    //     ref mut m => {
    //         *m += 10;
    //         println!("we added 10. `mut_value`: {:?}", m);
    //     }
    // }

    // =========================== match.destructing.structs ================================
    // struct Foo {
    //     x: (u32, u32),
    //     y: u32,
    // }
    // let foo = Foo { x: (1, 2), y: 3 };
    // match foo {
    //     Foo { x: (1, b), y } => println!("first of x is 1, b = {}, y = {}", b, y),
    //     Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
    //     Foo { y, .. } => println!("y = {}, x ignored", y),
    // }

    // =========================== match.Guards ================================
    // enum Temperature {
    //     Celsius(i32),
    //     Fahrenheit(i32),
    // }
    // let temperature = Temperature::Celsius(35);
    // match temperature {
    //     Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
    //     Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
    //     Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
    //     Temperature::Fahrenheit(t) => println!("{}F is below 86b Fahrenheit", t),
    // }
    // let number: u8 = 4;
    // match number {
    //     i if i == 0 => println!("zero"),
    //     i if i > 0 => println!("greater than zero"),
    //     _ => println!("unreachable"),
    // }

    // =========================== match.Binding ================================
    // fn age() -> u32 {
    //     15
    // }
    // println!("tell me what type of person you are");
    // match age() {
    //     0 => println!("i haven't celebrated my first birthday yet"),
    //     n @ 1..=12 => println!("i'm a child of age {}", n),
    //     n @ 13..=19 => println!("i'm a teen of age {}", n),
    //     n => println!("i'm an old person of age {}", n),
    // }
    // fn some_number() -> Option<u32> {
    //     Some(42)
    // }
    // match some_number() {
    //     Some(n @ 42) => println!("the answer: {}!", n),
    //     Some(n) => println!("not interesing... {}", n),
    //     _ => (),
    // }
}

pub fn if_let() {
    // let optional = Some(7);
    // match optional {
    //     Some(i) => {
    //         println!("this is a really long string and `{:?}`", i)
    //     }
    //     _ => {}
    // }
    // let number = Some(7);
    // let letter: Option<i32> = None;
    // let emoticon: Option<i32> = None;
    // if let Some(i) = number {
    //     println!("matched {:?}!", i);
    // }
    // if let Some(i) = letter {
    //     println!("matched {:?}!", i);
    // } else {
    //     println!("did not match a number, go with letter...")
    // }
    // let i_like_letters = false;
    // if let Some(i) = emoticon {
    //     println!("matched {:?}!", i);
    // } else if i_like_letters {
    //     println!("did not match a number, go with letter!")
    // } else {
    //     println!("i do not like letters, go with emoticon :)");
    // }
    // enum Foo {
    //     Bar,
    //     Baz,
    //     Qux(u32),
    // }
    // let a = Foo::Bar;
    // let b = Foo::Baz;
    // let c = Foo::Qux(100);
    // if let Foo::Bar = a {
    //     println!("a is foobar");
    // }
    // if let Foo::Bar = b {
    //     println!("b is foobar");
    // }
    // if let Foo::Qux(value) = c {
    //     println!("c is {}", value);
    // }
    // if let Foo::Qux(value @ 100) = c {
    //     println!("c is 100")
    // }
}

pub fn let_else() {
    // use std::str::FromStr;
    // fn get_count_item(s: &str) -> (u64, &str) {
    //     let mut it = s.split(' ');
    //     let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
    //         panic!("cannot segment count item pair: '{s}'");
    //     };
    //     let Ok(count) = u64::from_str(count_str) else {
    //         panic!("cannot parse integer: '{count_str}'");
    //     };
    //     (count, item)
    // }
    // assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
}

pub fn while_let() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`, try again...", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }
    optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`, try again...", i);
            optional = Some(i + 1);
        }
    }
}

pub fn flow_of_control_08() {
    // if_else()
    // loops()
    // while_loop()
    // for_and_range()
    // match_case();
    // if_let();
    // let_else();
    // while_let()
}
