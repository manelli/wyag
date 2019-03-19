#[macro_use]

extern crate clap;
use clap::App;

mod git;

fn main() {
    let yml = load_yaml!("cli.yml");
    let app = App::from_yaml(yml).get_matches();

    match app.subcommand() {
        ("init", Some(m)) => git::init(m.value_of("dir")),
        _ => (),
    }
}
