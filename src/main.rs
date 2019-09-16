use rksuid::rksuid::{deserialize, new, Ksuid};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CliOpts {
    action: String,
    serialized: Option<String>,
}
fn main() {
    let args = CliOpts::from_args();
    match args.action {
        action if action == "create" => println!("{:#?}", create()),
        action if action == "inspect" => println!("{:#?}", inspect(args.serialized.unwrap().as_str())),
        _ => {},
    }
}

fn inspect(serialized: &str) -> Ksuid {
    let sanitized = serialized.trim();
    deserialize(sanitized)
}

fn create() -> Ksuid {
    new(None, None)
}
