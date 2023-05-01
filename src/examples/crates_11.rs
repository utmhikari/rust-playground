pub fn public_function() {
    println!("called rary's public function");
}

fn private_function() {
    println!("called rary's private function");
}

pub fn indirect_access() {
    println!("called rary's indirect_access");
    private_function();
}

pub fn crates_11() {
    indirect_access();
}
