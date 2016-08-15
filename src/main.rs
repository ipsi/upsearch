fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];

    let user_home = std::env::home_dir().unwrap();

    let initial_path = match std::env::current_dir() {
        Ok(p) => p,
        Err(e) => panic!(format!("Unable to get current directory {}", e)),
    };

    let mut path = initial_path.as_path();

    loop {
        if path.with_file_name(&file).exists() {
            std::process::exit(0);
        } else if path.parent().unwrap().eq(&user_home) {
            std::process::exit(1);
        } else {
            path = match path.parent() {
                Some(p) => p,
                None => std::process::exit(1),
            };
        }
    }
}
