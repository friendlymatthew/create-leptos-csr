use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use dialoguer::{Confirm, Input};
use url::Url;

fn main() {
    let project_name: String = Input::new()
        .with_prompt("Enter your project name")
        .interact_text()
        .unwrap();

    let use_vercel = Confirm::new()
        .with_prompt("Will you be deploying with vercel?")
        .interact()
        .unwrap();

    let use_test = Confirm::new()
        .with_prompt("Do you want to scaffold tests?")
        .interact()
        .unwrap();

    setup_project(&project_name, use_vercel, use_test)
}

fn modify_file<F: FnOnce(String) -> String>(file_path: &str, modifier: F) {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let modified = modifier(content);
    let mut file = fs::File::create(file_path).expect("Failed to open file for editing");
    file.write_all(modified.as_bytes()).expect("Failed to write to file");
}

fn setup_project(project_name: &str, use_vercel: bool, use_test: bool) {
    let repo_url = Url::parse("https://github.com/friendlymatthew/leptos-csr-tailwind.git").unwrap();

    let status = Command::new("git")
        .arg("clone")
        .arg(repo_url.as_str())
        .arg(project_name)
        .status()
        .expect("Failed to clone boilerplate repository");

    if !status.success() {
        eprintln!("Error: failed to clone the repository.");
        return;
    }

    // 1. modify Cargo.toml
    let cargo_path = format!("{}/Cargo.toml", project_name);
    modify_file(&cargo_path, |content| {
        content.replace("name = \"leptos-csr-tailwind\"", &format!("name = \"{}\"", project_name))
    });

    // 2. modify app.rs
    let app_rs_path = format!("{}/src/app.rs", project_name);
    modify_file(&app_rs_path, |content| {
        content.replace("<Stylesheet id=\"leptos\" href=\"/pkg/leptos-csr-tailwind.css\"/>",
                        &format!("<Stylesheet id=\"leptos\" href=\"/pkg/{}.css\"/>", project_name))
    });

    // 3. conditionally remove vercel.json
    if !use_vercel {
        let vercel_path = format!("{}/vercel.json", project_name);

        if Path::new(&vercel_path).exists() {
            fs::remove_file(vercel_path).expect("Failed to remove vercel path");
        }
    }
}
