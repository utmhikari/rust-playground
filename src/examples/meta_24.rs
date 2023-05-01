pub fn documentation() {
    // =========================== doc comments =========================
    #![crate_name = "doc"]
    pub struct Person {
        name: String,
    }
    impl Person {
        /// # returns a person with name name given
        ///
        /// - input: name
        /// - output: new Person instance with name attribute as ${name}
        pub fn new(name: &str) -> Person {
            Person {
                name: name.to_string(),
            }
        }

        /// **Gives a friendly hello!**
        pub fn hello(&self) {
            println!("hello, {}", self.name);
        }
    }
    let john = Person::new("john");
    john.hello();

    // =========================== doc attributes =========================
    // #[doc(inline)]
    // pub use bar::Bar;
    // /// bar docs
    // mod bar {
    //     /// the docs for Bar
    //     pub struct Bar;
    // }
    // #[doc(no_inline)];
    // pub use crate::mem::drop;
    // #[doc(hidden)]
    // pub use self::async_await::*;
}

pub fn playground() {}

pub fn meta_24() {
    documentation();
    playground();
}
