extern crate clap;

use clap::{App, AppSettings, Arg};
use log::{error, LevelFilter};
use pacgen::{generate, serve, ServerConfig};
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::process;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn read_from_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_from_path(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut handle = File::open(&path)?;
    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn main() -> io::Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .filter(None, LevelFilter::Info)
        .init();

    let matches = App::new(PKG_NAME)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(PKG_VERSION)
        .author(PKG_AUTHORS)
        .about("Generates a PAC file from a TOML config.")
        .arg(
            Arg::with_name("serve")
                .short("s")
                .long("serve")
                .help("Serves the generated PAC file"),
        )
        .arg(
            Arg::with_name("host")
                .short("h")
                .requires("serve")
                .takes_value(true)
                .help("Host to bind the PAC server at [default: 127.0.0.1]"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .takes_value(true)
                .requires("serve")
                .help("Port to bind the PAC server at [default: 8080]"),
        )
        .arg(
            Arg::with_name("CONFIG")
                .help("Path to the config file to use (- for STDIN).")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config = {
        let path = matches.value_of("CONFIG").unwrap();
        let (verbose_path, result) = match path {
            "-" => ("STDIN", read_from_stdin()),
            path => (path, read_from_path(path)),
        };
        result.unwrap_or_else(|e| {
            error!("{}: {}", verbose_path, e);
            process::exit(1);
        })
    };

    let pac = match generate(config.as_ref()) {
        Ok(output) => output,
        Err(err) => {
            error!("Error: {}", err);
            process::exit(1);
        }
    };

    if matches.is_present("serve") {
        let port: u16 = matches
            .value_of("port")
            .unwrap_or("8080")
            .parse()
            .unwrap_or_else(|_| {
                error!("Specified port is not in the valid range (1-65535)");
                process::exit(1);
            });
        let addr: SocketAddr = {
            let default_host = format!("127.0.0.1:{}", port);
            matches
                .value_of("host")
                .map(|host| format!("{}:{}", host, port))
                .unwrap_or(default_host)
                .to_socket_addrs()
                .unwrap_or_else(|err| {
                    error!("Specified host is not valid: {}", err);
                    process::exit(1);
                })
                .next()
                .unwrap_or_else(|| {
                    error!("The given host was not resolvable");
                    process::exit(1);
                })
        };

        // Setup a SIGTERM handler
        ctrlc::set_handler(move || {
            process::exit(1);
        })
        .expect("Error setting Ctrl-C handler");

        let config = ServerConfig { addr, pac };
        serve(config);
    } else {
        println!("{}", pac);
    }
    Ok(())
}
