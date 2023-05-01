use std::iter::Iterator;

pub fn methods() {
    // struct Point {
    //     x: f64,
    //     y: f64,
    // }
    // impl Point {
    //     fn origin() -> Point {
    //         Point { x: 0.0, y: 0.0 }
    //     }
    //     fn new(x: f64, y: f64) -> Point {
    //         Point { x, y }
    //     }
    // }
    // struct Rectangle {
    //     p1: Point,
    //     p2: Point,
    // }
    // impl Rectangle {
    //     fn area(&self) -> f64 {
    //         let Point { x: x1, y: y1 } = self.p1;
    //         let Point { x: x2, y: y2 } = self.p2;
    //         ((x1 - x2) * (y1 - y2)).abs()
    //     }
    //     fn perimeter(&self) -> f64 {
    //         let Point { x: x1, y: y1 } = self.p1;
    //         let Point { x: x2, y: y2 } = self.p2;
    //         2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    //     }
    //     fn translate(&mut self, x: f64, y: f64) {
    //         self.p1.x += x;
    //         self.p2.x += x;
    //         self.p1.y += y;
    //         self.p2.y += y;
    //     }
    // }
    // struct Pair(Box<i32>, Box<i32>);
    // impl Pair {
    //     fn destroy(self) {
    //         let Pair(first, second) = self;
    //         println!("destroying Pair({}, {})", first, second);
    //     }
    // }
    // let rectangle = Rectangle {
    //     p1: Point::origin(),
    //     p2: Point::new(3.0, 4.0),
    // };
    // println!("Reactangle perimeter: {}", rectangle.perimeter());
    // println!("Reactangle area: {}", rectangle.area());
    // let mut square = Rectangle {
    //     p1: Point::origin(),
    //     p2: Point::new(1.0, 1.0),
    // };
    // square.translate(1.0, 1.0);
    // let pair = Pair(Box::new(1), Box::new(2));
    // pair.destroy();
}

pub fn closures() {
    // let outer_var = 42;
    // let closure_annotated = |i: i32| -> i32 { i + outer_var };
    // // let closure_inferred = |i| i + outer_var;
    // println!("closure annotated: {}", closure_annotated(1));
    // // println!("closure inferred: {}", closure_inferred(1));
    // let one = || 1;
    // println!("closure returning one: {}", one());

    // ================ capturing ======================
    // use std::mem;
    // let color = String::from("green");
    // let print = || println!("`color`: {}", color);
    // print();
    // let _reborrow = &color;
    // print();
    // let _color_moved = color;
    // // print();
    // let mut count = 0;
    // let mut inc = || {
    //     count += 1;
    //     println!("`count`: {}", count);
    // };
    // inc();
    // inc();
    // let movable = Box::new(3);
    // let consume = || {
    //     println!("`movable`: {:?}", movable);
    //     mem::drop(movable);
    // };
    // consume();
    // // consume();
    // let haystack = vec![1, 2, 3];
    // let contains = move |needle| haystack.contains(needle);
    // println!("{}", contains(&1));
    // println!("{}", contains(&4));

    // ================ as input parameters ======================
    // fn apply<F>(f: F)
    // where
    //     F: FnOnce(),
    // {
    //     f();
    // }
    // fn apply_to_3<F>(f: F) -> i32
    // where
    //     F: Fn(i32) -> i32,
    // {
    //     f(3)
    // }
    // use std::mem;
    // let greeting = "hello";
    // let mut farewell = "goodbye".to_owned();
    // let diary = || {
    //     println!("i said {}.", greeting);
    //     farewell.push_str("!!!");
    //     println!("then i screamed {}.", farewell);
    //     println!("now i can sleep. zzzzz");
    //     mem::drop(farewell);
    // };
    // apply(diary);
    // let double = |x| 2 * x;
    // println!("3 doubled: {}", apply_to_3(double));

    // ================ type anonymity ======================
    // fn apply<F>(f: F)
    // where
    //     F: Fn(),
    // {
    //     f()
    // }
    // let x = 7;
    // let print = || println!("{}", x);
    // apply(print);

    // ================ input functions ======================
    // fn call_me<F: Fn()>(f: F) {
    //     f();
    // }
    // fn function() {
    //     println!("i am a function");
    // }
    // let closure = || println!("i'm a closure");
    // call_me(closure);
    // call_me(function);

    // ================ as output parameters ======================
    // fn create_fn() -> impl Fn() {
    //     let text = "Fn".to_owned();
    //     move || println!("this is a {}", text)
    // }
    // fn create_fnmut() -> impl FnMut() {
    //     let text = "FnMut".to_owned();
    //     move || println!("this is a {}", text)
    // }
    // fn create_fnonce() -> impl FnOnce() {
    //     let text = "FnOnce".to_owned();
    //     move || println!("this is a {}", text)
    // }
    // let fn_plain = create_fn();
    // let mut fn_mut = create_fnmut();
    // let fn_once = create_fnonce();
    // fn_plain();
    // fn_mut();
    // fn_once();

    // ================ std.iterator::any ======================
    // use std::iter::Iterator;
    // // pub trait Iterator {
    // //     type Item;
    // //     fn any<F>(&mut self, f: F) -> bool
    // //     where
    // //         F: FnMut(Self::Item) -> bool;
    // // }
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6];
    // println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    // println!("vec1 len: {}", vec1.len());
    // println!("first element of vec1 is: {}", vec1[0]);
    //
    // let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];
    // println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    // ================ std.searching_through_iterators ======================
    // use std::iter::Iterator;
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6];
    // let mut iter = vec1.iter();
    // let mut into_iter = vec2.into_iter();
    // println!("find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // println!("find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
    // let array1 = [1, 2, 3];
    // let array2 = [4, 5, 6];
    // println!("find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    // println!(
    //     "find 2 in array2: {:?}",
    //     array2.into_iter().find(|&x| x == 2)
    // );
}

pub fn higher_order_functions() {
    // fn is_odd(n: u32) -> bool {
    //     n % 2 == 1
    // }
    // println!("find the sum of all the squared odd numbers under 1000");
    // let upper = 1000;
    // let mut acc = 0;
    // for n in 0.. {
    //     let n_squared = n * n;
    //     if n_squared >= upper {
    //         break;
    //     } else if is_odd(n_squared) {
    //         acc += n_squared;
    //     }
    // }
    // println!("imperative style: {}", acc);
    // let sum_of_squared_odd_numbers: u32 = (0..)
    //     .map(|n| n * n)
    //     .take_while(|&n_squared| n_squared < upper)
    //     .filter(|&n_squared| is_odd(n_squared))
    //     .sum();
    // println!("functional style: {}", sum_of_squared_odd_numbers);
}

pub fn diverging_functions() {
    // fn foo() -> ! {
    //     panic!("this call never returns")
    // }
    // fn some_fn() {
    //     ()
    // }
    // let _a: () = some_fn();
    // println!("this fn returns & you can see this line");
    //
    // // #![feature(never_type)]
    // // let x: ! = panic!("this call never returns");
    // // println!("you will never see this line");
    //
    // fn sum_odd_numbers(up_to: u32) -> u32 {
    //     let mut acc = 0;
    //     for i in 0..up_to {
    //         let addition: u32 = match i % 2 == 1 {
    //             true => i,
    //             false => continue,
    //         };
    //         acc += addition;
    //     }
    //     acc
    // }
    // println!(
    //     "some of odd numbers up to 9 (excluding): {}",
    //     sum_odd_numbers(9)
    // );
}

pub fn functions_09() {
    methods();
    closures();
    higher_order_functions();
    diverging_functions();
}
