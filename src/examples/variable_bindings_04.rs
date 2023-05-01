pub fn variable_bindings_04() {
    // ============================ Variable Bindings ================================
    // let an_integer = 1u32;
    // let a_boolean = true;
    // let unit = ();
    // let copied_integer = an_integer;
    // println!("An integer: {:?}", copied_integer);
    // println!("A boolean: {:?}", a_boolean);
    // println!("Meet the unit value: {:?}", unit);
    // let _unused_variable = 3u32;
    // let noisy_unused_variable = 2u32;

    // ============================ Mutability ================================
    // let _immutable_binding = 1;
    // let mut mutable_binding = 1;
    // println!("Before mutation: {}", mutable_binding);
    // mutable_binding += 1;
    // println!("After mutation: {}", mutable_binding);

    // ============================ Scope And Shadowing ================================
    // let long_lived_binding = 1;
    // {
    //     let short_lived_binding = 2;
    //     println!("inner short: {}", short_lived_binding);
    // }
    // println!("outer long: {}", long_lived_binding);
    // let shadowed_binding = 1;
    // {
    //     println!("before being shadowed: {}", shadowed_binding);
    //     let shadowed_binding = "abc";
    //     println!("shadowed in inner block: {}", shadowed_binding);
    // }
    // println!("outside inner block: {}", shadowed_binding);
    // let shadowed_binding = 2;
    // println!("shadowed in outer block: {}", shadowed_binding);

    // ============================ Declare First ================================
    // let a_binding;
    // {
    //     let x = 2;
    //     a_binding = x * x;
    // }
    // println!("a binding: {}", a_binding);
    // let another_binding;
    // // println!("another binding: {}", another_binding);
    // another_binding = 1;
    // println!("another binding: {}", another_binding);

    // ============================ Freezing ================================
    // let mut _mutable_integer = 7i32;
    // {
    //     let _mutable_integer = _mutable_integer;
    //     // _mutable_integer = 50;
    // }
    // _mutable_integer = 3;
}
