use crate::examples::attributes_13::dead_code;

pub fn raii() {
    // fn create_box() {
    //     let _box1 = Box::new(3i32);
    // }
    // let _box2 = Box::new(5i32);
    // {
    //     let _box3 = Box::new(4i32);
    // }
    // for _ in 0u32..1_000 {
    //     create_box();
    // }
    // struct ToDrop;
    // impl Drop for ToDrop {
    //     fn drop(&mut self) {
    //         println!("todrop is being dropped");
    //     }
    // }
    // let x = ToDrop;
    // println!("made a todrop");
}

pub fn ownership_and_moves() {
    // fn destroy_box(c: Box<i32>) {
    //     println!("destroying a box that contains {}", c);
    // }
    // let x = 5u32;
    // let y = x;
    // println!("x is {}, and y is {}", x, y);
    // let a = Box::new(5i32);
    // println!("a contains {}", a);
    // let b = a;
    // destroy_box(b);
    // // println!("b now is {}", b);

    // ========================== Mutability ==============================
    // let immutable_box = Box::new(5u32);
    // println!("immutable box contains {}", immutable_box);
    // let mut mutable_box = immutable_box;
    // *mutable_box = 4;
    // println!("mutable box now contains {}", mutable_box);

    // ========================== Partial Moves ===============================
    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: Box<u8>,
    // }
    // let person = Person {
    //     name: String::from("Alice"),
    //     age: Box::new(20),
    // };
    // let Person { name, ref age } = person;
    // println!("the person's age is {}", age);
    // println!("the person's name is {}", name);
    // println!("the person's age from person struct is {}", person.age);
}

pub fn borrowing() {
    // fn eat_box_i32(boxed_i32: Box<i32>) {
    //     println!("destroying box that contains {}", boxed_i32);
    // }
    // fn borrow_i32(borrowed_i32: &i32) {
    //     println!("this int is {}", borrowed_i32);
    // }
    // let boxed_i32 = Box::new(5_i32);
    // let stacked_i32 = 6_i32;
    // borrow_i32(&boxed_i32);
    // borrow_i32(&stacked_i32);
    // {
    //     let _ref_to_i32: &i32 = &boxed_i32;
    //     // eat_box_i32(boxed_i32);
    //     borrow_i32(_ref_to_i32);
    // }
    // // eat_box_i32(boxed_i32);

    // ======================== Mutability ==========================
    // #[allow(dead_code)]
    // #[derive(Clone, Copy)]
    // struct Book {
    //     author: &'static str,
    //     title: &'static str,
    //     year: u32,
    // }
    //
    // fn borrow_book(book: &Book) {
    //     println!(
    //         "i immutably borrowed {} - {} edition",
    //         book.title, book.year
    //     );
    // }
    // fn new_edition(book: &mut Book) {
    //     book.year = 2014;
    //     println!("i mutably borrowed {} - {} edition", book.title, book.year);
    // }
    // let immutabook = Book {
    //     author: "douglas hofstadter",
    //     title: "godel, escher, bach",
    //     year: 1979,
    // };
    // let mut mutabook = immutabook;
    // borrow_book(&immutabook);
    // borrow_book(&mutabook);
    // new_edition(&mut mutabook);

    // ======================== Aliasing ==========================
    // struct Point {
    //     x: i32,
    //     y: i32,
    //     z: i32,
    // };
    // let mut point = Point { x: 0, y: 0, z: 0 };
    // let borrowed_point = &point;
    // let another_borrow = &point;
    // println!(
    //     "point as coordinates: ({}, {}, {})",
    //     borrowed_point.x, borrowed_point.y, borrowed_point.z
    // );
    // println!(
    //     "point as coordinates: ({}, {}, {})",
    //     borrowed_point.x, another_borrow.y, point.z
    // );
    // let mutable_borrow = &mut point;
    // mutable_borrow.x = 5;
    // mutable_borrow.y = 2;
    // mutable_borrow.z = 1;
    // println!(
    //     "point as coordinates: ({}, {}, {})",
    //     mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    // );
    // let new_borrowed_point = &point;
    // println!(
    //     "point as coordinates: ({}, {}, {})",
    //     new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    // )

    // ======================== the ref pattern ==========================
    // #[derive(Clone, Copy)]
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // let c = 'Q';
    // let ref ref_c1 = c;
    // let ref_c2 = &c;
    // println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    // let point = Point { x: 0, y: 0 };
    // let _copy_of_x = {
    //     let Point {
    //         x: ref ref_to_x,
    //         y: _,
    //     } = point;
    //     *ref_to_x
    // };
    // let mut mutable_point = point;
    // {
    //     let Point {
    //         x: _,
    //         y: ref mut mut_ref_to_y,
    //     } = mutable_point;
    //     *mut_ref_to_y = 1;
    // }
    // println!("point is ({}, {})", point.x, point.y);
    // println!(
    //     "mutable point is ({}, {})",
    //     mutable_point.x, mutable_point.y
    // );
    //
    // let mut mutable_tuple = (Box::new(5u32), 3u32);
    // {
    //     let (_, ref mut last) = mutable_tuple;
    //     *last = 2u32;
    // }
    // println!("tuple is {:?}", mutable_tuple);
}

pub fn lifetimes() {
    // ===================== Explicit annotation =======================
    // fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    //     println!("x is {} and y is {}", x, y);
    // }
    // fn failed_borrow<'c>() {
    //     let _x = 12;
    //     let y: &'c i32 = &_x;
    // }
    // let (four, nine) = (4, 9);
    // print_refs(&four, &nine);
    // failed_borrow();

    // ===================== Functions =======================
    // fn print_one<'a>(x: &'a i32) {
    //     println!("`print_one`: x is {}", x)
    // }
    // fn add_one<'a>(x: &'a mut i32) {
    //     *x += 1;
    // }
    // fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    //     println!("`print_multi`: x is {} and y is {}", x, y);
    // }
    // fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }
    // let x = 7;
    // let y = 9;
    // print_one(&x);
    // print_multi(&x, &y);
    // let z = pass_x(&x, &y);
    // print_one(z);
    // let mut t = 3;
    // add_one(&mut t);
    // print_one(&t);

    // ===================== Methods =======================
    // struct Owner(i32);
    // impl Owner {
    //     fn add_one<'a>(&'a mut self) {
    //         self.0 += 1;
    //     }
    //     fn print<'a>(&'a self) {
    //         println!("`print`: {}", self.0);
    //     }
    // }
    // let mut owner = Owner(18);
    // owner.add_one();
    // owner.print();

    // ===================== Structs =======================
    // #[derive(Debug)]
    // struct Borrowed<'a>(&'a i32);
    // #[derive(Debug)]
    // struct NamedBorrowed<'a> {
    //     x: &'a i32,
    //     y: &'a i32,
    // }
    // #[derive(Debug)]
    // enum Either<'a> {
    //     Num(i32),
    //     Ref(&'a i32),
    // }
    // let x = 18;
    // let y = 15;
    // let single = Borrowed(&x);
    // let double = NamedBorrowed { x: &x, y: &y };
    // let reference = Either::Ref(&x);
    // let number = Either::Num(y);
    // println!("x is borrowed in {:?}", single);
    // println!("x and y is borrowed in {:?}", double);
    // println!("x is borrowed in {:?}", reference);
    // println!("y is not borrowed in {:?}", number);

    // ===================== Traits =======================
    // #[derive(Debug)]
    // struct Borrowed<'a> {
    //     x: &'a i32,
    // }
    // impl<'a> Default for Borrowed<'a> {
    //     fn default() -> Self {
    //         Self { x: &10 }
    //     }
    // }
    // let b: Borrowed = Default::default();
    // println!("b is {:?}", b);

    // ===================== Bounds =======================
    // use std::fmt::Debug;
    // #[derive(Debug)]
    // struct Ref<'a, T: 'a>(&'a T);
    // fn print<T>(t: T)
    // where
    //     T: Debug,
    // {
    //     println!("print: t is {:?}", t);
    // }
    // fn print_ref<'a, T>(t: &'a T)
    // where
    //     T: Debug + 'a,
    // {
    //     println!("print_ref: t is {:?}", t);
    // }
    // let x = 7;
    // let ref_x = Ref(&x);
    // print_ref(&ref_x);
    // print(ref_x);

    // ===================== Coercion =======================
    // fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    //     first * second
    // }
    // fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    //     first
    // }
    // let first = 2;
    // {
    //     let second = 3;
    //     println!("the product is {}", multiply(&first, &second));
    //     println!("{} is the first", choose_first(&first, &second));
    //     println!("{} is the first", choose_first(&second, &first));
    // }

    // ===================== Static =======================
    // let s: &'static str = "helloworld";
    // fn generic<T>(x: T)
    // where
    //     T: 'static,
    // {
    // }
    // static NUM: i32 = 18;
    // fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    //     &NUM
    // }
    // {
    //     let static_string = "i am in read-only memory";
    //     println!("static string: {}", static_string);
    // }
    // {
    //     let lifetime_num = 9;
    //     let coerced_static = coerce_static(&lifetime_num);
    //     println!("coerced_static: {}", coerced_static);
    // }
    // println!("NUM {} stays accessible", &NUM);
    // use std::fmt::Debug;
    // fn print_it(input: impl Debug + 'static) {
    //     println!("'static value passed in is: {:?}", input);
    // }
    // let i = 5;
    // print_it(i);
    // // print_it(&i);

    // ===================== Elision =======================
    fn elided_input(x: &i32) {
        println!("elided input: {}", x);
    }
    fn annotated_input<'a>(x: &'a i32) {
        println!("annotated_input: {}", x);
    }
    fn elided_pass(x: &i32) -> &i32 {
        x
    }
    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
        x
    }
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("elided_pass: {}", elided_pass(&x));
    println!("annotated_pass: {}", annotated_pass(&x));
}

pub fn scoping_rules_15() {
    // raii();
    // ownership_and_moves();
    // borrowing();
    // lifetimes();
}
