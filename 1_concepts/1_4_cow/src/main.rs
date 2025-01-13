use std::env;
use std::borrow::Cow;

fn main() {
    match get_config_path() {
        Some(config_path) => println!("Configuration file path: {}", config_path),
        None => eprintln!("Error: --conf argument specified but no path provided"),
    }
}

fn get_config_path() -> Option<Cow<'static, str>> {
    // Check if the --conf command line argument is specified
    let mut args = env::args();
    while let Some(arg) = args.next() {
        if arg == "--conf" {
            if let Some(path) = args.next() {
                if !path.is_empty() {
                    return Some(Cow::Owned(path));
                }
            }
            return None; // Return None if --conf is specified but no path is provided
        } 
    }

    // Check if the APP_CONF environment variable is set
    if let Ok(env_path) = env::var("APP_CONF") {
        if !env_path.is_empty() {
            return Some(Cow::Owned(env_path));
        }
    }

    // Use the default path
    Some(Cow::Borrowed("/etc/app/app.conf"))
}
