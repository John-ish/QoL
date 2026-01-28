use anyhow::Result;
use clap::{Arg, ArgMatches, Command, command};
mod package_management;

// parsing arguments cro
fn main() -> anyhow::Result<()> {
    let match_result: ArgMatches = command!()
    .subcommand(
        Command::new("pkg") 
        .arg(
            Arg::new( "CHECK_PKG_UPDATES")
            .short('u')
            .long("checkup")
            .help("Argument returns list of last 10 UPDATED packages")
            .default_missing_value("10")
            .num_args(0..=1)
        )
        .arg(
            Arg::new( "CHECK_PKG_INSTALLS")
            .short('i')
            .long("checkin")
            .help("Argument returns list of last 10 INSTALLED packages")
            .default_missing_value("10")
            .num_args(0..=1)
        )
        .arg(
            Arg::new("SPECIFIC_PACKAGE")
            .short('s')
            .long("specify")
            .help("Argument returns the pacman logs of the SPECIFIED package")
            .num_args(1)
        )
    )
    .subcommand(
        Command::new("temp")
        .arg(
            Arg::new("LIST_EXISTING_TEMPLATES")
            .short('l')
            .long("list")
            .help("Returns a list of existing templates")
        )
    )
    .about("Quality of Life CLI")
    .get_matches();

    match match_result.subcommand() {
        Some(("pkg", update_flags)) => handle_pkg(update_flags)?,
        Some(("temp", _temp_flags)) => println!("Temp logic goes in temp.rs later!"),
        _ => println!("No subcommand given"),
    }

    Ok(())

} 
fn handle_pkg(flags: &ArgMatches) -> Result<()> {
    if flags.contains_id("CHECK_PKG_UPDATES") {
        let limit = parse_limit(flags, "CHECK_PKG_UPDATES");
        print_results(package_management::package_updates(limit)?);
    } 
    else if flags.contains_id("CHECK_PKG_INSTALLS") {
        let limit = parse_limit(flags, "CHECK_PKG_INSTALLS");
        print_results(package_management::package_installs(limit)?);
    }
    else if let Some(name) = flags.get_one::<String>("SPECIFIC_PACKAGE") {
        print_results(package_management::specific_package(name)?);
    }

    Ok(())
}


fn parse_limit(m: &ArgMatches, id: &str) -> usize {
    m.get_one::<String>(id)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

fn print_results(logs: Vec<String>) {
    if logs.is_empty() {
        println!("No logs found");
    } else {
        for log in logs {
            println!("{}", log);
        }
    }
}

