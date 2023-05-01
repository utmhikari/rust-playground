use crate::examples::modules_10::my_mod::nested;

mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        println!("called `my_mod::indirect_access()`");
        private_function()
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(in crate::examples::modules_10::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`")
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

pub fn visibility() {
    // my_mod::function();
    // my_mod::indirect_access();
    // my_mod::nested::function();
    // my_mod::call_public_function_in_my_mod();
    // my_mod::public_function_in_crate()
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }
    pub struct ClosedBox<T> {
        contents: T,
    }
    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

pub fn struct_visibility() {
    // let open_box = my::OpenBox { contents: "public information" };
    // println!("the openbox contains: {}", open_box.contents);
    // let _closed_box = my::ClosedBox::new("public information");
    // // println!("the closedbox contains: {}", open_box.contents);
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

pub fn the_use_declaration() {
    use deeply::nested::function as other_function;
    other_function();
    println!("entering block");
    {
        use crate::examples::modules_10::deeply::nested::function;
        function();
        println!("leaving block");
    }
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod mymy {
    fn function() {
        println!("called `mymy::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `mymy::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called mymy::indirect_call(), that\n> ");
        self::function();
        function();
        self::cool::function();
        // super::function();
        {
            use crate::examples::modules_10::cool::function as root_function;
            root_function();
        }
    }
}

pub fn super_and_self() {
    mymy::indirect_call();
}

pub fn file_hierarchy() {
    // no need to implement..
}

pub fn modules_10() {
    // visibility();
    // struct_visibility();
    // the_use_declaration();
    // super_and_self();
    // file_hierarchy();
}
