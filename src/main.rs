use std::process::Command;
use std::{error::Error, path::Path};
mod app;
use app::*;
mod completer;
mod configstore;
mod searcher;
mod term;
use term::*;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() -> Result<(), Box<dyn Error>> {
    let mut config_path_str = "~/.ssh/config";
    let mut search_str = "";
    let args = parse_args();
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            "-h" | "-help" | "--help" => return print_usage(),
            "-v" | "-version" | "--version" => return print_version(),
            "-s" | "-search" | "--search" => {
                if let Some(search) = args.next() {
                    search_str = search;
                } else {
                    return print_usage();
                }
            }
            "-c" | "-config" | "--config" => {
                if let Some(path) = args.next() {
                    config_path_str = path;
                } else {
                    return print_usage();
                }
            }
            _ => return print_usage(),
        }
    }
    let expand_str = shellexpand::tilde(config_path_str);
    let config_str = expand_str.as_ref();
    let config_path = Path::new(config_str);

    let mut app = match App::new(config_path, search_str) {
        Ok(app) => app,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let mut terminal = init_terminal()?;

    app.state.select(Some(0));
    let res = run(&mut terminal, &mut app);

    // restore terminal
    restore_terminal(&mut terminal)?;

    if app.should_spawn_ssh {
        let selected_config = match app.get_selected_item() {
            Some(item) => item,
            None => std::process::exit(1),
        };
        let host_name = &&selected_config.host;
        let jumpers = app.completer.display_string.trim_end_matches(',');
        match jumpers.is_empty() {
            true => Command::new("ssh")
                .arg(host_name.split(' ').take(1).collect::<Vec<&str>>().join(""))
                .spawn()?
                .wait()?,
            false => Command::new("ssh")
                .arg("-J")
                .arg(jumpers)
                .arg(host_name.split(' ').take(1).collect::<Vec<&str>>().join(""))
                .spawn()?
                .wait()?,
        };
    }
    if let Err(err) = res {
        eprintln!("{}", err)
    }
    Ok(())
}

fn parse_args() -> Vec<String> {
    let mut args = vec![];
    for arg in std::env::args().skip(1).collect::<Vec<String>>() {
        if arg.starts_with('-') && arg.contains('=') {
            for part in arg.split("=") {
                args.push(part.to_string());
            }
        } else {
            args.push(arg);
        }
    }
    args
}
fn print_usage() -> Result<(), Box<dyn Error>> {
    println!(
        "ssh clients manager in rust. Usage: sshrs [options]
Options:
    -c, --config path    SSH config file (default \"~/.ssh/config\")
    -s, --search string  Host search filter
    -v, --version        version for sshrs
    -h, --help           help for sshrs"
    );
    Ok(())
}
fn print_version() -> Result<(), Box<dyn Error>> {
    println!("sshrs v{}", VERSION);
    Ok(())
}
