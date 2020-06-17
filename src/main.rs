extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs;
use std::io::Write;
use clap::{Arg, App};
use std::process::Command;
use serde_json::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Service {
    name: String,
    urls: Vec<String>
}

impl Service {
    fn push_url(&mut self, url: String) {
        self.urls.push(String::from(url))
    }
}

fn open_url(urls: String) {
    let o = Command::new("open")
        .arg(urls)
        .output()
        .expect("failed to execute process");
    match String::from_utf8(o.stdout) {
        Ok(v) => println!("open url success"),
        Err(e) => panic!("Fail to open url {}", e),
    };
    
}

fn init_config() -> String {
    let file = fs::OpenOptions::new().write(true).create_new(true).open(".a");
    match file {
        Ok(mut f) => {
            f.write("[]".as_bytes()).expect("init config file fail");
            String::from(".a")
        }
        Err(_e) => {
            String::from(".a")
        }
    }
}

fn load_services() -> Result<Vec<Service>, Error>{
    init_config();
    let content = fs::read_to_string(".a").expect("fail to read config file");
    let services: Vec<Service> = serde_json::from_str(content.as_str()).unwrap();
    Ok(services)
}

fn save_services(services: &Vec<Service>) -> Result<(), serde_json::Error>{
    let json = serde_json::to_string(&services).expect("save service fail");
    let mut f = fs::OpenOptions::new().write(true).open(".a").unwrap();
    f.write_all(json.as_bytes()).unwrap();
    Ok(())
}

fn main() {
    let matches = App::new("a")
        .version("0.1")
        .author("Leoython, leoython@gmail.com")
        .about("some tool")
        .subcommand(App::new("open")
            .about("open url when alert")
            .arg(
                Arg::new("app")
                    .value_name("APP NAME")
                    .about("URL to be opened when opening the configured app alarm")
                    .required(true)
                    .takes_value(true)
            )
        )
        .subcommand(
            App::new("add")
                .about("add app to config")
                .arg(
                    Arg::with_name("app")
                        .value_name("APP NAME")
                        .about("app name")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::new("urls")
                        .value_name("URLS")
                        .short('u')
                        .about("Set urls for the app, if there are multiple, please use comma to separate")
                )
        )
        .get_matches();

    if let Some(open_matches) = matches.subcommand_matches("open") {
        let app_name = open_matches.value_of("app").unwrap();
        let services = load_services().unwrap();
        for s in services.into_iter() {
            if s.name == app_name {
                for url in s.urls.into_iter() {
                    open_url(url.to_owned());
                }
                return;
            }
        }
        eprintln!("app does not found");
    }
    if let Some(add_matches) = matches.subcommand_matches("add") {
        let app_name = add_matches.value_of("app").unwrap();
        let urls = add_matches.value_of("urls").unwrap_or("");
        let mut services = load_services().unwrap();

        for s in services.iter_mut() {
            if s.name == app_name {
                eprintln!("app does exist");
                return;
            }
        }
        
        let mut service = Service {
            name: String::from(app_name),
            urls: Vec::new(),
        };
        for u in urls.split(",") {
            service.push_url(String::from(u))
        }
        services.push(service);
        save_services(&services).unwrap();
    }
}