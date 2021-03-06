extern crate impose;

pub use audio::audio_loop;

pub mod audio;

pub fn parse_args() -> (String, String) {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: duel NAME HOST");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    (name, host)
}
