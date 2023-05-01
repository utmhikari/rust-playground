pub fn hello_world_01() {
    // println!("hello world");

    // =============== formatted prints ================================

    // let x = 5 + /* 90 + */ 5;
    // println!("is `x` 10 or 100? x = {}", x);
    // println!("{} days", 31);
    // println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // println!(
    //     "{subject} {verb} {object}",
    //     object = "the lazy dog",
    //     verb = "jumps over",
    //     subject = "the quick brown fox"
    // );
    // println!("Base 10: {}", 69420);
    // println!("Base 2: {:b}", 69420);
    // println!("Base 8: {:o}", 69420);
    // println!("Base 16: {:x}", 69420);
    // println!("Base 16: {:X}", 69420);

    // println!("{number:>5}", number = 1);
    // println!("{number:0<5}", number = 1);
    // println!("{number:0>width$}", number = 1, width = 5);

    // println!("my name is {0}, {1} {0}", "bond", "james");

    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("this struct `{}` won't print...", Structure(3));

    // let number: f64 = 1.0;
    // let width: usize = 5;
    // println!("{number:>width$}", number = number, width = width);

    // ======= Debug =======

    // #[derive(Debug)]
    // struct Structure(i32);
    // #[derive(Debug)]
    // struct Deep(Structure);
    // println!("{:?} months in a year", 12);
    // println!(
    //     "{1:?} {0:?} is the {actor:?} name",
    //     "Slater",
    //     "Christian",
    //     actor = "actor's"
    // );
    // println!("Now {:?} will print!", Structure(3));
    // println!("Now {:?} will print!", Deep(Structure(7)));

    // #[derive(Debug)]
    // struct Person<'a> {
    //     name: &'a str,
    //     age: u8,
    // }
    // let name = "Peter";
    // let age = 27;
    // let peter = Person { name, age };
    // println!("{:#?}", peter);

    // ======== Display ======
    // use std::fmt; // import fmt
    // struct Structure(i32);
    // impl fmt::Display for Structure {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "Structure Is -> {}", self.0)
    //     }
    // }
    // println!("{}", Structure(3));

    // use std::fmt;
    // #[derive(Debug)]
    // struct MinMax(i64, i64);
    // impl fmt::Display for MinMax {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "({}, {})", self.0, self.1)
    //     }
    // }
    // #[derive(Debug)]
    // struct Point2D {
    //     x: f64,
    //     y: f64,
    // }
    // impl fmt::Display for Point2D {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "x: {}, y: {}", self.x, self.y)
    //     }
    // }
    // let minmax = MinMax(0, 14);
    // println!("compare structures:");
    // println!("Display: {}", minmax);
    // println!("Debug: {:?}", minmax);
    // let big_range = MinMax(-300, 300);
    // let small_range = MinMax(-3, 3);
    // println!(
    //     "the big range is {big} and the small range is {small}",
    //     big = big_range,
    //     small = small_range
    // );
    // let point = Point2D { x: 3.3, y: 7.2 };
    // println!("compare points:");
    // println!("Display: {}", point);
    // println!("Debug: {:?}", point);

    // ====== List =====
    // use std::fmt;
    // struct List(Vec<i32>);
    // impl fmt::Display for List {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         let vec = &self.0;
    //         write!(f, "[")?;
    //         for (count, v) in vec.iter().enumerate() {
    //             if count != 0 {
    //                 write!(f, ", ")?;
    //             }
    //             write!(f, "{}", v)?;
    //         }
    //         write!(f, "]")
    //     }
    // }
    // let v = List(vec![1, 2, 3]);
    // println!("{}", v);

    // ======= Formatting ========
    // #[allow(unused_imports)]
    // use std::fmt::{self, Display, Formatter};
    // struct City {
    //     name: &'static str,
    //     lat: f32,
    //     lon: f32,
    // }
    // impl fmt::Display for City {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    //         let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
    //         write!(
    //             f,
    //             "{}: {:.3}{} {:.3}{}",
    //             self.name,
    //             self.lat.abs(),
    //             lat_c,
    //             self.lon.abs(),
    //             lon_c
    //         )
    //     }
    // }
    // #[allow(dead_code)]
    // #[derive(Debug)]
    // struct Color {
    //     red: u8,
    //     green: u8,
    //     blue: u8,
    // }
    // for city in [
    //     City {
    //         name: "Dublin",
    //         lat: 53.347778,
    //         lon: -6.259722,
    //     },
    //     City {
    //         name: "Oslo",
    //         lat: 59.95,
    //         lon: 10.75,
    //     },
    //     City {
    //         name: "Vancouver",
    //         lat: 49.25,
    //         lon: -123.1,
    //     },
    // ]
    // .iter()
    // {
    //     println!("{}", *city);
    // }
    // for color in [
    //     Color {
    //         red: 128,
    //         green: 255,
    //         blue: 90,
    //     },
    //     Color {
    //         red: 0,
    //         green: 3,
    //         blue: 254,
    //     },
    //     Color {
    //         red: 0,
    //         green: 0,
    //         blue: 0,
    //     },
    // ]
    // .iter()
    // {
    //     println!("{:?}", *color);
    // }
}
