use redis::Commands;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} set|get key [value]", args[0]);
        return;
    }

    let cmd = &args[1];
    let key = &args[2];

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    match cmd.as_str() {
        "set" => {
            if args.len() != 4 {
                println!("The 'set' command requires a value");
                return;
            }
            let value = &args[3];
            let _: () = con.set(key, value).unwrap();
            println!("Set key {} to value {}", key, value);
        }
        "get" => {
            if args.len() != 3 {
                println!("The 'get' command requires only a key");
                return;
            }
            let fetched_value: String = con.get(key).unwrap();
            println!("Fetched value for key {}: {}", key, fetched_value);
        }
        _ => println!("Unknown command. Use 'set' or 'get'"),
    }
}
