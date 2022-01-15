mod client;
mod utils;

use crate::utils::read_from_toml;
use crate::client::Client;
use clap::{App, Arg, AppSettings};
use dirs::config_dir;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");

fn main() {
    let config_path = config_dir().unwrap()
        .join("mgclient")
        .join("mgclient.conf");
    if !config_path.exists(){
        println!("Configure Mailgun Client");
        return;
    }
    let config = read_from_toml(config_path.to_str().unwrap());
    let protocol = config.get("PROTOCOL").unwrap();
    let base_uri = config.get("BASE_URI").unwrap();
    let domain = config.get("DOMAIN").unwrap();
    let token = config.get("TOKEN").unwrap();
    let client = Client::new(protocol, base_uri, domain, token);
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("simple")
                    .about("Send simple messages")
                    .arg(Arg::new("from")
                         .short('f')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("to")
                         .short('t')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("subject")
                         .short('s')
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("body")
                         .short('b')
                         .required(true)
                         .takes_value(true))
                    )
        .get_matches();
    if let Some(sub) = matches.subcommand_matches("simple"){
        let from = sub.value_of("from").unwrap();
        let to = sub.value_of("to").unwrap();
        let subject = sub.value_of("subject").unwrap();
        let body = sub.value_of("body").unwrap();
        match client.send_simple_message(from, to, subject, body){
            Ok(result) => println!("{}", result.text().unwrap()),
            Err(result) => println!("{}", result.to_string())
        }
    }
}
