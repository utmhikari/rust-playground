use std::io::{BufReader, Read};

pub fn threads() {
    use std::thread;
    const NTHREADS: u32 = 10;
    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
            i + NTHREADS * 10
        }))
    }
    for child in children {
        let o = child.join();
        println!("thread returns: {}", o.unwrap());
    }

    // ======================= map-reduce ========================
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    let mut children = vec![];
    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is '{}'", i, data_segment);
        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {} -> {}", data_segment, result);
            result
        }))
    }
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    println!("final result: {}", final_result);
}

pub fn channels() {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    static NTHREADS: i32 = 3;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();
    for id in 0..NTHREADS {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
        children.push(child);
    }
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }
    for child in children {
        child.join().expect("oops, the child thread panicked");
    }
    println!("{:?}", ids);
}

pub fn path() {
    use std::path::Path;
    let path = Path::new(".");
    let _display = path.display();
    let mut new_path = path.join("a").join("b");
    new_path.push("c");
    new_path.push("myfile.tar.gz");
    new_path.set_file_name("package.tgz");
    match new_path.to_str() {
        None => panic!("invalid path"),
        Some(s) => println!("new path is {}", s),
    }
}

pub fn file_io() {
    // ======================= open ========================
    // use std::fs::File;
    // use std::io::prelude::*;
    // use std::path::Path;
    // let path = Path::new("misc/hello.txt");
    // let display = path.display();
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("could not open {}: {}", display, why),
    //     Ok(file) => file,
    // };
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("could not read {}: {}", display, why),
    //     Ok(_) => println!("{} contains:\n{}", display, s),
    // }

    // ======================= create ==========================
    //     static LOREM_IPSUM: &str =
    //         "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
    // tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
    // quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
    // consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
    // cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
    // proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    // ";
    //     use std::fs::File;
    //     use std::io::prelude::*;
    //     use std::path::Path;
    //     let path = Path::new("misc/lorem_ipsum.txt");
    //     let display = path.display();
    //     let mut file = match File::create(&path) {
    //         Err(why) => panic!("could not create {}: {}", display, why),
    //         Ok(file) => file,
    //     };
    //     match file.write_all(LOREM_IPSUM.as_bytes()) {
    //         Err(why) => panic!("could not write to {}: {}", display, why),
    //         Ok(_) => println!("successfully wrote to {}", display),
    //     }

    // ======================== read_lines ============================
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
        let file = File::open(filename).unwrap();
        return io::BufReader::new(file).lines();
    }
    let filename = "./misc/lorem_ipsum.txt";
    let lines = read_lines(filename.to_string());
    for (i, line) in lines.enumerate() {
        println!("line {}: {}", i + 1, line.unwrap());
    }
}

pub fn child_processes() {
    // use std::process::Command;
    // let output = Command::new("rustc")
    //     .arg("--version")
    //     .output()
    //     .unwrap_or_else(|e| {
    //         panic!("failed to execute process: {}", e);
    //     });
    // if output.status.success() {
    //     let s = String::from_utf8_lossy(&output.stdout);
    //     print!("rustc succeeded and stdout was:\n{}", s);
    // } else {
    //     let s = String::from_utf8_lossy(&output.stderr);
    //     println!("rustc failed and stderr was: {}", s);
    // }

    // ======================= pipes ========================
    // use std::io::prelude::*;
    // use std::process::{Command, Stdio};
    // static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";
    // let process = match Command::new("wc")
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()
    // {
    //     Err(why) => panic!("couldn't spawn wc: {}", why),
    //     Ok(process) => process,
    // };
    // match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
    //     Err(why) => panic!("couldn't write to wc stdin: {}", why),
    //     Ok(_) => println!("send program to wc"),
    // }
    // let mut s = String::new();
    // match process.stdout.unwrap().read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read wc stdout: {}", why),
    //     Ok(_) => println!("wc responded with:\n{}", s),
    // }

    // ======================== wait ==========================
    use std::process::Command;
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reach end...")
}

pub fn filesystem_operations() {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::io::prelude::*;
    use std::os::unix;
    use std::path::Path;
    fn cat(path: &Path) -> io::Result<String> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    fn echo(s: &str, path: &Path) -> io::Result<()> {
        let mut f = File::create(path)?;
        f.write_all(s.as_bytes())
    }
    fn touch(path: &Path) -> io::Result<()> {
        match OpenOptions::new().create(true).write(true).open(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    println!("`mkdir misc/a`");
    match fs::create_dir("misc/a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {}
    }
    println!("`echo hello > misc/a/b.txt`");
    echo("hello", &Path::new("misc/a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("`mkdir -p misc/a/c/d`");
    fs::create_dir_all("misc/a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("`touch misc/a/c/e.txt`");
    touch(&Path::new("misc/a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("`ln -s misc/a/b.txt misc/a/c/b.txt`");
    if cfg!(target_family = "unix") {
        unix::fs::symlink("misc/a/b.txt", "isc/a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }
    println!("`cat misc/a/c/b.txt`");
    match cat(&Path::new("misc/a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }
    println!("`ls misc/a`");
    match fs::read_dir("misc/a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                println!("> {:?}", path.unwrap().path());
            }
        }
    }
    println!("`rm misc/a/c/e.txt");
    fs::remove_file("misc/a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("`rmdir misc/a/c/d`");
    fs::remove_dir("misc/a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn program_arguments() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    println!("my path is {}", args[0]);
    println!("i got {:?} arguments: {:?}", args.len() - 1, &args[1..]);

    // ======================== argument parsing =========================
    fn increase(number: i32) {
        println!("{}", number + 1);
    }
    fn decrease(number: i32) {
        println!("{}", number - 1);
    }
    fn help() {
        println!(
            "usage:
match_args <string>
    Check whether given string is the answer.
match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
        );
    }
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("my name is 'match_args'. try passing some args...");
        }
        2 => match args[1].parse() {
            Ok(42) => println!("this is the answer!"),
            _ => println!("this is not the answer"),
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("err: second arg not a number");
                    help();
                    return;
                }
            };
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        _ => {
            help();
        }
    }
}

pub fn foreign_function_interface() {
    use std::fmt;
    #[link(name = "m")]
    extern "C" {
        fn csqrtf(z: Complex) -> Complex;
        fn ccosf(z: Complex) -> Complex;
    }
    fn cos(z: Complex) -> Complex {
        unsafe { ccosf(z) }
    }
    let z = Complex { re: -1., im: 0. };
    let z_sqrt = unsafe { csqrtf(z) };
    println!("the square root of {:?} is {:?}", z, z_sqrt);
    println!("cos({:?}) = {:?}", z, cos(z));
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Complex {
        re: f32,
        im: f32,
    }
    impl fmt::Debug for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.im < 0. {
                write!(f, "{}-{}i", self.re, self.im)
            } else {
                write!(f, "{}+{}i", self.re, self.im)
            }
        }
    }
}

pub fn std_misc_20() {
    // threads();
    // channels();
    // path();
    // file_io();
    // child_processes();
    // filesystem_operations();
    // program_arguments();
    foreign_function_interface();
}
