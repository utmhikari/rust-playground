pub fn dead_code() {
    fn used_function() {}

    #[allow(dead_code)]
    fn unused_function() {}

    fn noisy_unused_function() {}

    used_function();
}

pub fn crates() {
    // #![crate_type = "lib"]
    // #![crate_name = "rary"]

    pub fn public_function() {
        println!("called rary's public function")
    }

    pub fn private_function() {
        println!("called rary's private function")
    }

    pub fn indirect_access() {
        println!("called rary's indirect access");
        private_function();
    }
}

pub fn cfg() {
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("you are running linux");
    }

    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("you are not running linux");
    }

    are_you_on_linux();

    if cfg!(target_os = "linux") {
        println!("actually is linux!")
    } else {
        println!("not linux!")
    }
}

pub fn attributes_13() {
    dead_code();
    crates();
    cfg();
}
