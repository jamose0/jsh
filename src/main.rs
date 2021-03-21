use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "jsh", version = "v0.0")]
struct Config {
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let config = Config::from_args();
}
