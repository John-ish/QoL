// Crates used brutha
use anyhow::{Ok, Result};
use clap::{Arg, ArgMatches, Command, command};
mod package_management;
mod template_manager;
mod init;

fn main() -> anyhow::Result<()> {
    // Creates the config file if the cli is run for the first time
    init::config_file()?;

    // parsing arguments cro
    let match_result: ArgMatches = command!()
    .subcommand(
        Command::new("pkg")            //parsing arguments for the pkg subcommand                
        .arg(
            //parses argument; returns list of updated packages
            Arg::new( "CHECK_PKG_UPDATES")
            .short('u')
            .long("checkup")
            .help("Argument returns list of last 10 UPDATED packages")
            .default_missing_value("10")
            .num_args(0..=1)
        )
        .arg(
            //parses arguments; returns list of installed packages
            Arg::new( "CHECK_PKG_INSTALLS")
            .short('i')
            .long("checkin")
            .help("Argument returns list of last 10 INSTALLED packages")
            .default_missing_value("10")
            .num_args(0..=1)
        )
        .arg(
            //parses arguments; returns list of logs for a specific package
            Arg::new("SPECIFIC_PACKAGE")
            .short('s')
            .long("search")
            .help("Argument returns the pacman logs of the SPECIFIED package")
            .num_args(1)
        )
    )
    .subcommand(
        Command::new("temp")    //parsing arguments for the template_manager subcommand
        .arg(
            //lists existing templates and user defined templates 
            Arg::new("LIST_EXISTING_TEMPLATES")
            .short('l')
            .long("list")
            .help("Returns a list of existing templates")
            .action(clap::ArgAction::Set)
            .num_args(0)
        )
        .arg(
            Arg::new("INIT_TEMP")
            .short('i')
            .long("init")
            .help("Creates projects using the templates created in the config file")
            .num_args(2)
            .value_names(["TEMPLATE", "NAME"])
        )
    )
    .about("Quality of Life CLI")
    .get_matches();

    // checks which subcomand was parsed 
    match match_result.subcommand() {
        Some(("pkg", update_flags)) => handle_pkg(update_flags)?,
        Some(("temp", temp_flags)) => handle_templates(temp_flags)?,
        _ => println!("No subcommand given"),
    }

    Ok(())

} 

//handles the 'pkg' subcommand
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

// parses user defined 'limit' to functions
fn parse_limit(flags: &ArgMatches, id: &str) -> usize {
    flags.get_one::<String>(id)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

// puts the result to the terminal
fn print_results(list: Vec<String>) {
    if list.is_empty() {
        println!("No logs found");  
    } else {
        for item in list {
            println!("{}", item);
        }
    }
}


// handles the "temp" subcommand
fn handle_templates(flags: &ArgMatches) -> Result<()> {
    if flags.contains_id("LIST_EXISTING_TEMPLATES") {
        template_manager::interactive_select_template()?;
    } 
    else if flags.contains_id("INIT_TEMP"){
        if let Some(mut values) = flags.get_many::<String>("INIT_TEMP") {
        let template_name = values.next().unwrap();
        let project_name = values.next().unwrap();

        template_manager::initialize_templates(template_name, project_name)?;
        return Ok(());
        }

        
    } 

    Ok(())
}

