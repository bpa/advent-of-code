use std::env;

lazy_static! {
    pub static ref ENABLED: bool = {
        match env::var("DEBUG") {
            Ok(_) => true,
            Err(_) => false,
        }
    };
}

#[macro_export]
macro_rules! debug {
        // debug!("a {} event", "log")
        ($($arg:tt)+) => {
            if *aoc::debug::ENABLED {
                print!("{}:{} ", std::path::Path::new(file!()).file_name().unwrap().to_str().unwrap(), line!());
                println!($($arg)+)
            }
        };
}
