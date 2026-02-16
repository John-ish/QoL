use std::fs::{self, File};
use std::path::{Path, PathBuf};
use anyhow::{Context, Ok, Result};
use std::result::Result::Ok as otherOK;
use walkdir::WalkDir;
use inquire::{Select, Text};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};


pub fn list_templates() -> anyhow::Result<Vec<String>> {
    let path = get_template_path()?;

    if !path.exists() {
        return Ok(Vec::<String>::new());
    }

    let mut templates = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if entry.file_type()?.is_dir() && !name.starts_with('.') {
            templates.push(name);
        }
    }

    templates.sort();
    Ok(templates)
}

// Gets the paths of the templates available in the config folder 
fn get_template_path() -> Result<PathBuf>{
    let home = std::env::var("HOME").context("Could not find HOME environment")?;
    let mut path = PathBuf::from(home);
    path.push(".config");
    path.push("Qol");
    path.push("Templates");
    Ok(path)
}

// Checks if the file is binary or not
fn is_binary(path: &Path) -> bool {
    let mut file = match File::open(path) {
        otherOK(f) => f,
        Err(_) => return true,
    };

    let mut buffer = [0; 1024]; // Creates a buffer to check if there is a binary
    let n = file.read(&mut buffer).unwrap_or(0);
    buffer[..n].contains(&0)
}

fn generate_refined(template_name: &str) -> Result<()> {
    let template_base = get_template_path()?.join(template_name);
    let target_base = std::env::current_dir()?;
    let re = Regex::new(r"\{\{(.*?)\}\}")?; // looks for '{{value_name}}' in the directory name or file name/content

    let mut tags = Vec::new();
    let mut seen = HashSet::new();

    for entry in WalkDir::new(&template_base).min_depth(1){
        let entry = entry?;
        let path_str = entry.path().to_string_lossy();

        for cap in re.captures_iter(&path_str){
            let label = if cap[1].is_empty() { "Value".into() } else { cap[1].to_string() };
            if !seen.contains(&label) {
                seen.insert(label.clone());
                tags.push(label);
            }
            
            if entry.file_type().is_file() && !is_binary(entry.path()){
                let file = File::open(entry.path())?;
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line.unwrap_or_default();
                    for cap in re.captures_iter(&line) {
                        let label = if cap[1].is_empty() { "Value".into() } else { cap[1].to_string() };
                        if !seen.contains(&label) {
                            seen.insert(label.clone());
                            tags.push(label);
                        }
                    }
                }
            }
        }
    }

    let mut answers = HashMap::new();

    for tag in tags {
        let val = Text::new(&format!("{}:", tag)).prompt()?;
        answers.insert(tag, val);
    }

    let mut entries: Vec<_> = WalkDir::new(&template_base).min_depth(1).into_iter().filter_map(|e| e.ok()).collect();

    entries.sort_by(|a, b| {a.depth().cmp(&b.depth()).then(a.path().cmp(b.path()))});

    for entry in entries {
        let rel_path = entry.path().strip_prefix(&template_base)?;

        let mut rel_path_str = rel_path.to_string_lossy().into_owned();
        for (tag, val) in &answers {
            let target = format!("{{{{{}}}}}", if tag == "Value" { "" } else { tag });
            rel_path_str = rel_path_str.replace(&target, val);
        }

        let dest_path = target_base.join(rel_path_str);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() { fs::create_dir_all(parent)?;}

            if !is_binary(entry.path()) {
                let mut content = fs::read_to_string(entry.path())?;
                for (tag, val) in &answers {
                    let target = format!("{{{{{}}}}}", if tag == "Value" { "" } else { tag });
                    content = content.replace(&target, val);
                }
                fs::write(&dest_path, content)?;

            } else {
                fs::copy(entry.path(), &dest_path)?;
            }
        }
    }

    Ok(())
}

pub fn interactive_select_template() -> Result<()> {
    let templates = list_templates()?;

    if templates.is_empty() {
        anyhow::bail!("No Templates found")
    }

    let selection = Select::new("Choose a template:", templates)
        .with_help_message("↑/↓ to navigate, start typing to filter")
        .prompt()?;

    generate_refined(&selection)

}


pub fn initialize_templates(template_name: &str, project_name: &str) -> anyhow::Result<()> {

    let template_base = get_template_path()?.join(template_name);
    let project_base = PathBuf::from(project_name);

    if !template_base.exists(){
        anyhow::bail!("Template '{}' does not exist in the config", template_name);
    }

    println!("Using template '{}'...", template_name);

    for entry in WalkDir::new(&template_base).min_depth(1) {
        let entry = entry?;
        let src_path = entry.path();

        let relative_path = src_path.strip_prefix(&template_base)?;
        let destination = project_base.join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&destination)?;
        } else {
            copy_and_replace(src_path, &destination, project_name)?;
        }
    }

    println!("Project created successfully");

    Ok(())

}

fn copy_and_replace(src: &Path, dest: &Path, project_name: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(src)?;

    let new_content = content.replace("{{NAME}}", project_name);

    fs::write(dest, new_content)?;

    Ok(())
}
