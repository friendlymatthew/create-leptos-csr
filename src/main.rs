use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use dialoguer::{Confirm, Input};
use url::Url;
use colored::*;

fn main() {
    println!(
        "{}",
        r#"
                      _             _            _                                     _
   ___ _ __ ___  __ _| |_ ___      | | ___ _ __ | |_ ___  ___        ___ ___ _ __     | |___      __
  / __| '__/ _ \/ _` | __/ _ \_____| |/ _ \ '_ \| __/ _ \/ __|_____ / __/ __| '__|____| __\ \ /\ / /
 | (__| | |  __/ (_| | ||  __/_____| |  __/ |_) | || (_) \__ \_____| (__\__ \ | |_____| |_ \ V  V /
  \___|_|  \___|\__,_|\__\___|     |_|\___| .__/ \__\___/|___/      \___|___/_|        \__| \_/\_/
                                          |_|
"#.blue());

    let project_name: String = Input::new()
        .with_prompt("Enter your project name")
        .interact_text()
        .unwrap();

    let use_vercel = Confirm::new()
        .with_prompt("Will you be deploying with vercel?")
        .interact()
        .unwrap();

    setup_project(&project_name, use_vercel)
}

fn modify_file<F: FnOnce(String) -> String>(file_path: &str, modifier: F) {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let modified = modifier(content);
    let mut file = fs::File::create(file_path).expect("Failed to open file for editing");
    file.write_all(modified.as_bytes()).expect("Failed to write to file");
}

fn setup_project(project_name: &str, use_vercel: bool) {
    let repo_url = Url::parse("https://github.com/friendlymatthew/create-leptos-csr.git").unwrap();

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
        content.replace("name = \"tailwind\"", &format!("name = \"{}\"", project_name))
    });

    // 2. modify index.html
    let index_html_path = format!("{}/index.html", project_name);
    modify_file(&index_html_path, |content| {
        content.replace("<title>tailwind</title>", &format!("<title>{}</title>", project_name))
    });

    // 3. modify app.rs
    let app_rs_path = format!("{}/src/app.rs", project_name);
    modify_file(&app_rs_path, |content| {
        content.replace("<Stylesheet id=\"leptos\" href=\"/pkg/tailwind.css\"/>",
                        &format!("<Stylesheet id=\"leptos\" href=\"/pkg/{}.css\"/>", project_name))
    });

    // 4. conditionally remove vercel.json
    if !use_vercel {
        let vercel_path = format!("{}/vercel.json", project_name);
        let index_html_path = format!("{}/index.html", project_name);

        modify_file(&index_html_path, |content| {
            let target_pattern = r#"\s*<link\s+data-trunk\s+rel="copy-file"\s+href="vercel\.json"\s*/>\s*"#;
            let re = regex::Regex::new(target_pattern).unwrap();
            let replaced_content = re.replace_all(&content, "\n\t").to_string();
            replaced_content
        });

        if Path::new(&vercel_path).exists() {
            fs::remove_file(vercel_path).expect("Failed to remove vercel path");
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use tempfile::TempDir;
    use super::*;

    #[test]
    fn test_modify_file_no_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("bogus_wef.txt");

        let result = std::panic::catch_unwind(|| {
            modify_file(&file_path.to_string_lossy(), |content| {
                content.replace("random", "test")
            });
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_modify_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("wef.txt");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"name = \"leptos-csr-tailwind\"").unwrap();

        modify_file(&file_path.to_string_lossy(), |content| {
            content.replace("name = \"leptos-csr-tailwind\"", "name = \"test\"")
        });

        let modified_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(modified_content, "name = \"test\"");
    }

}