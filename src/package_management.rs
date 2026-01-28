use anyhow::{Context, Result};
use std::fs::File;
use rev_lines::RevLines;
use colored::Colorize;

fn get_pacman_logs(keyword: &str, limit: usize, offset: usize) -> Result<Vec<String>> {
    let contents = File::open("/var/log/pacman.log").context("Could not open /var/log/pacman.log")?;
    let rev_lines = RevLines::new(contents);

    let mut results = Vec::new();

    for line in rev_lines.flatten() {
        if line.contains(keyword) {
            if line.len() > offset{
                let formatted = format!("{} {} {}", &line[offset..].red(), &line[1..11].green(), &line[12..20].yellow());
                results.push(formatted.to_string());
            }   
        }

        if results.len() >= limit {
            break;
        }
    }

    Ok(results)
}

fn specific_package_logs(limit: usize, offset: usize, package_name: &str) -> Result<Vec<String>>{
    let content = File::open("/var/log/pacman.log").context("something went wrong while reading the file")?;
    let rev_lines = RevLines::new(content);
    let mut spec = Vec::new();

    for line_result in rev_lines {
        if let Ok(line) = line_result{
            if (line.contains(*&package_name) && line.contains(" [ALPM] upgraded ")) || (line.contains(" [ALPM] installed ") && line.contains(*&package_name)) || (line.contains(*&package_name) && line.contains(" [ALPM] reinstalled ")){ 
                let formatted = format!("{} {} {}", &line[offset..].red(), &line[1..11].green(), &line[12..20].yellow());
                spec.push(formatted.to_string());
            }
        }

        if spec.len() >= limit {
            break;
        }
    }

    Ok(spec)
}


pub fn package_updates(limit: usize) -> Result<Vec<String>> {
    get_pacman_logs(" [ALPM] upgraded ", limit, 43)
}

pub fn package_installs(limit: usize) -> Result<Vec<String>> {
    get_pacman_logs(" [ALPM] installed ", limit, 44)
}

pub fn specific_package(package_name: &str) -> Result<Vec<String>> {
    specific_package_logs(10, 34, package_name)
}

