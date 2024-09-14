fn largest<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let n = largest(1, 9);
    println!("{n}");
}


fn mix<T, U>(t: T, u: U) -> (T, U) {
    (t, u)
}

fn main() {
    let n: (i32, u8) = mix(1001, b'a');
    println!("{n:?}");
}


#[derive(Debug, Clone)]
struct Config {
    width: u32,
    height: u32,
}

fn create_window(config: Config) {
    println!("Width: {}, Height: {}", config.width, config.height);
}

fn main() {
    let config = Config { width: 800, height: 600 };
    create_window(config.clone());
    println!("{:?}", config);
}


struct Config {
    width: u32,
    height: u32,
    title: Option<String>,
}

fn create_window(config: Config) {
    let title = config.title.unwrap_or_else(|| "Untitled".to_string());
    println!("Creating window: {} ({}x{})", title, config.width, config.height);
}

fn main() {
    let config = Config { width: 800, height: 600, title: None };
    create_window(config);
    
    let config_with_title = Config { width: 800, height: 600, title: Some("My App".to_string()) };
    create_window(config_with_title);
}


macro_rules! create_window {
    // 宏的不同模式匹配，模拟参数默认值
    ($width:expr, $height:expr) => {
        create_window_inner($width, $height, "Untitled");
    };
    ($width:expr, $height:expr, $title:expr) => {
        create_window_inner($width, $height, $title);
    };
}

fn create_window_inner(width: u32, height: u32, title: &str) {
    println!("Creating window: {} ({}x{})", title, width, height);
}

fn main() {
    create_window!(800, 600); // 使用默认标题
    create_window!(800, 600, "My App"); // 自定义标题
}
