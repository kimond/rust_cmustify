extern crate cmustify;

use std::env;
use cmustify::{DbusNotifier, run};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cmus_data = args.join(" ");
    let notifier = DbusNotifier{};
    run(&notifier, cmus_data);
}
