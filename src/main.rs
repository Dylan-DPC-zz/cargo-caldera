use crate::runner::{Project, Runner};
use std::{io::Write, path::PathBuf};
use structopt::StructOpt;

pub mod runner;

#[derive(StructOpt, Debug)]
#[structopt(name = "stabilize")]
struct Cli {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,
}

fn main() {
    std::env::set_current_dir(std::path::Path::new("/home/dpc/Code/uuid")).expect("cannot set dir");

    let path = std::env::current_dir().expect("cannot find path of the project");

    let opt = Cli::from_args();
    let project = Project::try_new(path.as_ref());

    let runner = Runner::new(project.expect("cannot create project"));
    println!("{:#?}", runner);
}

pub fn prompt_reply_stdout(prompt: &str) -> std::io::Result<String> {
    let mut reply = String::new();
    let mut stdout = std::io::stdout();
    write!(stdout, "{}", prompt)?;
    stdout.flush()?;
    std::io::stdin().read_line(&mut reply)?;

    Ok(reply)
}
