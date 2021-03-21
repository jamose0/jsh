use std::error::Error;
use structopt::StructOpt;
use rustyline::Editor;

#[derive(StructOpt)]
#[structopt(name = "jsh", version = "v0.1.0")]
pub struct Config {
    #[structopt(short, long)]
    pub debug: bool,
}

pub struct Jsh {
    config: Config,
}

impl Jsh {
    pub fn new(config: Config) -> Jsh {
        Jsh {
            config,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut r = Editor::<()>::new();

        loop {
            let readline = r.readline("jsh > ");
            match readline {
                Ok(line) => {
                    println!("line: {}", line);
                },
                Err(e) => {
                    eprintln!("Err: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
