// Crates used borochan
use anyhow::{Context, Result};
use std::fs::File;
use rev_lines::RevLines;
use colored::Colorize;

fn get_pacman_logs(keyword: &str, limit: usize, offset: usize) -> Result<Vec<String>> {
    let contents = File::open("/var/log/pacman.log")
        .context("Could not open /var/log/pacman.log. are u sure you're using arch")?; // checks if pacman logs exist

    let rev_lines = RevLines::new(contents); // uses the 'rev_lines' crate to read file in reverse using buffer

    let mut results = Vec::new(); // empty vector for the results to be pushed into

    for line in rev_lines.flatten() {
        if line.contains(keyword) {
            if line.len() > offset{
                let pkg_name = &line[offset..].red();
                let date = &line[1..11].green();
                let time = &line[12..20].yellow();
                let formatted = format!("{} {} {}", pkg_name, date, time); // The way the results are formatted 
                results.push(formatted.to_string());
            }   
        }

        if results.len() >= limit { // obeys the limit given by the user
            break;
        }   
    }

    Ok(results)
}

// Same as the above function but for the '-s' flag
fn specific_package_logs(limit: usize, offset: usize, package_name: &str) -> Result<Vec<String>>{
    let content = File::open("/var/log/pacman.log").context("something went wrong while reading the file")?;
    let rev_lines = RevLines::new(content);
    let mut specific_pkg = Vec::new();

    for line_result in rev_lines {
        if let Ok(line) = line_result{
            if (line.contains(*&package_name) && line.contains(" [ALPM] upgraded ")) || 
            (line.contains(" [ALPM] installed ") && line.contains(*&package_name)) || 
            (line.contains(*&package_name) && line.contains(" [ALPM] reinstalled ")) // long ass if statement to filter the specific package logs
            { 
                let pkg_name = &line[offset..].red();
                let date = &line[1..11].green();
                let time = &line[12..20].yellow();
                let formatted = format!("{} {} {}", pkg_name, date, time);
                specific_pkg.push(formatted.to_string());
            }
        }

        if specific_pkg.len() >= limit {
            break;
        }
    }

    Ok(specific_pkg)
}


pub fn package_updates(limit: usize) -> Result<Vec<String>> {
    get_pacman_logs(" [ALPM] upgraded ", limit, 43) /*filter word, limit and 'offset' which is just how much you need to
                                                                    slice the string*/ 
}

pub fn package_installs(limit: usize) -> Result<Vec<String>> {
    get_pacman_logs(" [ALPM] installed ", limit, 44)
}

pub fn specific_package(package_name: &str) -> Result<Vec<String>> {
    specific_package_logs(10, 34, package_name)
}


