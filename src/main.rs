use structopt::StructOpt;

use jsh::{Config, Jsh};

fn main() {
    let config = Config::from_args();
    let jsh = Jsh::new(config);
    if let Err(e) = jsh.run() {
        eprintln!("Error: {}", e);
    }
}
