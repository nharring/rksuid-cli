use rksuid::rksuid::{deserialize, new, Ksuid};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CliOpts {
    action: String,
    serialized: Option<String>,
}

fn main() {
    let args = CliOpts::from_args();

    let ksuid: Option<Ksuid> = match args.action {
        ref action if action == "create" => Some(create().unwrap()),
        ref action if action == "inspect" =>
            Some(inspect(args.serialized.unwrap().as_str()).unwrap()),
        _ => None,
    };
    match ksuid {
        Some(ksuid) => {println!("{}",ksuid.get_formatted());},
        None => {},
    }
}

fn inspect(serialized: &str) -> Result<Ksuid, String> {
    let sanitized = serialized.trim();
    Ok(deserialize(sanitized))
}

fn create() -> Result<Ksuid, String> {
    Ok(new(None, None))
}
