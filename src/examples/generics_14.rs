use std::marker::PhantomData;

pub fn functions() {
    struct A;
    struct S(A);
    struct SGen<T>(T);
    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}

pub fn implementation() {
    struct S;
    struct GenericVal<T>(T);
    impl GenericVal<f32> {}
    impl GenericVal<S> {}
    impl<T> GenericVal<T> {}
    struct Val {
        val: f64,
    }
    struct GenVal<T> {
        gen_val: T,
    }
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}

pub fn traits() {
    struct Empty;
    struct Null;
    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
}

pub fn bounds() {
    // use std::fmt::Debug;
    // trait HasArea {
    //     fn area(&self) -> f64;
    // }
    // impl HasArea for Rectangle {
    //     fn area(&self) -> f64 {
    //         self.length * self.height
    //     }
    // }
    // #[derive(Debug)]
    // struct Rectangle {
    //     length: f64,
    //     height: f64,
    // }
    // #[allow(dead_code)]
    // struct Triangle {
    //     length: f64,
    //     height: f64,
    // }
    // fn print_debug<T: Debug>(t: &T) {
    //     println!("{:?}", t);
    // }
    // fn area<T: HasArea>(t: &T) -> f64 {
    //     t.area()
    // }
    // let rectangle = Rectangle {
    //     length: 3.0,
    //     height: 4.0,
    // };
    // let triangle = Triangle {
    //     length: 3.0,
    //     height: 4.0,
    // };
    // print_debug(&rectangle);
    // println!("area: {}", rectangle.area());
    // // print_debug(&triangle);

    // ================== Empty Bounds =================
    struct Cardinal;
    struct BlueJay;
    struct Turkey;
    trait Red {}
    trait Blue {}
    impl Red for Cardinal {}
    impl Blue for BlueJay {}
    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }
    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;
    println!("a cardinal is {}", red(&cardinal));
    println!("a bluejay is {}", blue(&blue_jay));
}

pub fn multiple_bounds() {
    use std::fmt::{Debug, Display};
    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("debug: `{:?}`", t);
        println!("display: `{}`", t);
    }
    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];
    compare_prints(&string);
    compare_types(&array, &vec);
}

pub fn where_clauses() {
    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(self);
    }
    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self))
        }
    }
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

pub fn new_type_idiom() {
    struct Years(i64);
    struct Days(i64);
    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }
    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }
    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }
    let age = Years(5);
    let age_days = age.to_days();
    println!("old enough {}", old_enough(&age));
    println!("old enough {}", old_enough(&age_days.to_years()));
    let Years(age_as_primitive) = age;
    println!("age: {}", age_as_primitive);
}

pub fn associated_items() {
    // struct Container(i32, i32);
    // trait Contains<A, B> {
    //     fn contains(&self, _: &A, _: &B) -> bool;
    //     fn first(&self) -> i32;
    //     fn last(&self) -> i32;
    // }
    // impl Contains<i32, i32> for Container {
    //     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    //         (&self.0 == number_1) && (&self.1 == number_2)
    //     }
    //     fn first(&self) -> i32 {
    //         self.0
    //     }
    //     fn last(&self) -> i32 {
    //         self.1
    //     }
    // }
    // fn difference<A, B, C>(container: &C) -> i32
    //     where
    //         C: Contains<A, B>,
    // {
    //     container.last() - container.first()
    // }

    // struct Container(i32, i32);
    // trait Contains {
    //     type A;
    //     type B;
    //     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    //     fn first(&self) -> i32;
    //     fn last(&self) -> i32;
    // }
    // impl Contains for Container {
    //     type A = i32;
    //     type B = i32;
    //     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
    //         (&self.0 == number_1) && (&self.1 == number_2)
    //     }
    //     fn first(&self) -> i32 {
    //         self.0
    //     }
    //     fn last(&self) -> i32 {
    //         self.1
    //     }
    // }
    // fn difference<C: Contains>(container: &C) -> i32 {
    //     container.last() - container.first()
    // }
    // let number_1 = 3;
    // let number_2 = 10;
    // let container = Container(number_1, number_2);
    // println!(
    //     "does container contains {} and {}: {}",
    //     number_1,
    //     number_2,
    //     container.contains(&number_1, &number_2)
    // );
    // println!("first number: {}", container.first());
    // println!("second number: {}", container.last());
    // println!("the difference is: {}", difference(&container));
}

pub fn phantom_type_parameters() {
    // ===================== examples ===================
    // use std::marker::PhantomData;
    // #[derive(PartialEq)]
    // struct PhantomTuple<A, B>(A, PhantomData<B>);
    // #[derive(PartialEq)]
    // struct PhantomStruct<A, B> {
    //     first: A,
    //     phantom: PhantomData<B>,
    // }
    // let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    // let _struct1: PhantomStruct<char, f32> = PhantomStruct {
    //     first: 'Q',
    //     phantom: PhantomData,
    // };
    // let _struct2: PhantomStruct<char, f64> = PhantomStruct {
    //     first: 'Q',
    //     phantom: PhantomData,
    // };

    // ===================== unit clarification ===================
    use std::marker::PhantomData;
    use std::ops::Add;
    #[derive(Debug, Clone, Copy)]
    enum Inch {}
    #[derive(Debug, Clone, Copy)]
    enum Mm {}
    #[derive(Debug, Clone, Copy)]
    struct Length<Unit>(f64, PhantomData<Unit>);
    impl<Unit> Add for Length<Unit> {
        type Output = Length<Unit>;
        fn add(self, rhs: Length<Unit>) -> Length<Unit> {
            Length(self.0 + rhs.0, PhantomData)
        }
    }
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;
    println!("one foot + one foot = {:?} in", two_feet.0);
    println!("one meter + one meter = {:?} mm", two_meters.0);
    // println!("one foot + one meter = {:?} ??", (one_foot + one_meter).0);
}

pub fn generics_14() {
    // functions();
    // implementation();
    // traits();
    // bounds();
    // multiple_bounds();
    // where_clauses();
    // new_type_idiom();
    // associated_items();
    phantom_type_parameters();
}
