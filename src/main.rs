use rksuid::rksuid::{deserialize, new, Ksuid};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CliOpts {
    action: String,
    serialized: Option<String>,
    pretty_print: Option<bool>,
}
fn main() {
    let args = CliOpts::from_args();
    let ksuid: Option<Ksuid>;
    match args.action {
        action if action == "create" => ksuid = Some(create().unwrap()),
        action if action == "inspect" => ksuid = Some(inspect(args.serialized.unwrap().as_str()).unwrap()),
        _ => ksuid = None,
    }
    match ksuid {
        Some(ksuid) => {let serialized = ksuid.serialize(); println!("Got ksuid: {}\n Contents: {:#?}", serialized, ksuid);},
        None => {}
    }
}

fn inspect(serialized: &str) -> Result<Ksuid, String> {
    let sanitized = serialized.trim();
    Ok(deserialize(sanitized))

}

fn create() -> Result<Ksuid, String> {
    Ok(new(None, None))
}
