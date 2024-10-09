// src/main.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[MAIN]Xyn>=====S===t===u===d===i===o===s======[R|$>

use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::env;

const PROJECT_DIRECTORIES: &[&str] = &[
    "src/omnixtracker",
    "src/constants",
    "src/utils",
    "tests",
    "Xdocs",
    "Xtls",
    "config",
];

const AUTHOR_NAME: &str = "Lord Xyn";
const AUTHOR_EMAIL: &str = "LordXyn@proton.me";
const GITHUB_URL: &str = "https://github.com/arcmoonstudios";
const LICENSE_YEAR: &str = "2024";
const LICENSE_HOLDER: &str = "Carl Liu";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Lord Xyn's Pro Initializer...");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Incorrect number of arguments.");
        println!("Usage: xynpro <project_name> or xynpro --version");
        return Ok(());
    }

    match args[1].as_str() {
        "--version" => {
            println!("xynpro version 0.1.3");
            return Ok(());
        }
        project_name => {
            let current_dir = env::current_dir()?;
            let project_path = current_dir.join(project_name);
            println!("Project path: {:?}", project_path);

            create_project_structure(&project_path, project_name)?;

            println!("{} project initialized successfully at {:?}!", project_name, project_path);
        }
    }

    Ok(())
}

fn create_project_structure(project_path: &Path, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating project structure at {:?}", project_path);
    fs::create_dir_all(project_path)?;

    for dir in PROJECT_DIRECTORIES {
        let full_path = project_path.join(dir);
        println!("Creating directory: {:?}", full_path);
        fs::create_dir_all(&full_path)?;
    }

    create_xtls_files(project_path)?;
    create_xdocs_files(project_path)?;
    create_tests_files(project_path)?;
    create_utils_files(project_path)?;
    create_omnixtracker_files(project_path)?;
    create_file(project_path, ".env", &generate_env_content())?;
    create_file(project_path, "LICENSE", &generate_license_content())?;
    create_file(project_path, "src/lib.rs", &generate_lib_rs_content())?;
    create_file(project_path, "build.rs", &generate_build_rs_content())?;
    create_file(project_path, "src/main.rs", &generate_main_rs_content())?;
    create_file(project_path, ".gitignore", &generate_gitignore_content())?;
    create_file(project_path, "README.md", &generate_readme_content(project_name))?;
    create_file(project_path, "src/constants/mod.rs", &generate_constants_content())?;
    create_file(project_path, "Cargo.toml", &generate_cargo_toml_content(project_name))?;
    create_file(project_path, "config/prometheus.yml", &generate_prometheus_yml_content(project_name))?;

    Ok(())
}

fn generate_jwt_secret() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn create_file(project_path: &Path, file_name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = project_path.join(file_name);
    println!("Writing {} at {:?}", file_name, file_path);
    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_omnixtracker_files(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let omnixtracker_path = project_path.join("src/omnixtracker");
    create_file(&omnixtracker_path, "omnixerror.rs", &generate_omnixerror_content())?;
    create_file(&omnixtracker_path, "omnixmetry.rs", &generate_omnixmetry_content())?;
    create_file(&omnixtracker_path, "mod.rs", &generate_omnixtracker_mod_content())?;
    println!("Created omnixtracker directory at {:?}", omnixtracker_path);
    Ok(())
}

fn create_utils_files(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let utils_path = project_path.join("src/utils");
    create_file(&utils_path, "lxsl.rs", &generate_lxsl_content())?;
    create_file(&utils_path, "mod.rs", &generate_utils_mod_content())?;
    println!("Created utils directory at {:?}", utils_path);
    Ok(())
}

fn create_tests_files(project_path: &Path, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tests_path = project_path.join("tests");
    create_file(&tests_path, "constants_tests.rs", &generate_constants_tests_content(project_name))?;
    create_file(&tests_path, "omnixerror_tests.rs", &generate_omnixerror_tests_content(project_name))?;
    create_file(&tests_path, "omnixmetry_tests.rs", &generate_omnixmetry_tests_content(project_name))?;
    create_file(&tests_path, "utils_lxsl_tests.rs", &generate_utils_lxsl_tests_content(project_name))?;
    println!("Created tests directory at {:?}", tests_path);
    Ok(())
}

fn create_xdocs_files(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let xdocs_path = project_path.join("Xdocs");
    fs::create_dir_all(&xdocs_path)?;
    println!("Created Xdocs directory at {:?}", xdocs_path);
    Ok(())
}

fn create_xtls_files(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let xtls_path = project_path.join("Xtls");
    fs::create_dir_all(&xtls_path)?;
    create_file(&xtls_path, "XynPro_Instructions-HowTo.txt", &generate_xtls_xynpro_content())?;
    println!("Created Xtls/XynPro_Instructions-HowTo.txt");
    create_file(&xtls_path, "XynTools-HowTo.txt", &generate_xtls_xyntools_content())?;
    println!("Created Xtls/XynTools-HowTo.txt");
    Ok(())
}

fn generate_cargo_toml_content(project_name: &str) -> String {
    format!(
        r##"[package]
name = "{}"
version = "0.1.4"
edition = "2021"
authors = ["{} <{}>"]
repository = "{}/{}"
build = "build.rs"

[build-dependencies]
anyhow = "1.0.89"
walkdir = "2.5.0"

[dependencies]
anyhow = "1.0.89"
chrono = "0.4"
colored = "2.0"
dotenv = "0.15.0"
git2 = "0.15"
lazy_static = "1.4"
metrics = "0.23.0"
metrics-exporter-prometheus = "0.15"
parking_lot = "0.12.3"
thiserror = "1.0.64"
tokio = {{ version = "1.40", features = ["full"] }}
tracing = "0.1.40"
tracing-subscriber = {{ version = "0.3.18", features = ["env-filter", "std"] }}

[profile.dev]
debug = true
lto = false
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

[profile.release]
codegen-units = 1
debug = false
lto = true
opt-level = "z"

[workspace]
resolver = "2"
"##,
        project_name, AUTHOR_NAME, AUTHOR_EMAIL, GITHUB_URL, project_name
    )
}

fn generate_prometheus_yml_content(project_name: &str) -> String {
    format!(r#"# config/prometheus.yml ~=#######D]======A===r===c====M===o===o===n=====<Lord[PROMETHEUS]Xyn>=====S===t===u===d===i===o===s======[R|$>

global:
  scrape_interval: 5s
  evaluation_interval: 5s

scrape_configs:
  - job_name: '{}'
    static_configs:
      - targets: ['localhost:9001']
"#, project_name)
}

fn generate_main_rs_content() -> String {
    r##"// src/main.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[MAIN]Xyn>=====S===t===u===d===i===o===s======[R|$>

use test6::constants::{CIRCUIT_BREAKER_THRESHOLD, CIRCUIT_BREAKER_DURATION, BASE_DELAY, MAX_DELAY, DEFAULT_TIMEOUT};
use test6::omnixtracker::{OmniXMetry, setup_global_subscriber, OmniXErrorManager, OmniXErrorManagerConfig};
use anyhow::Result;
use dotenv::dotenv;
use std::env::args; 
use tracing::info;
use std::env; 


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    info!("Welcome to Lord Xyn's Domain! Initializing systems...");

    // Check if placeholders in .env are replaced
    if env::var("GPG_PASSPHRASE").unwrap_or_default() == "your_gpg_passphrase_placeholder" {
        return Err(anyhow::anyhow!("Please replace the GPG_PASSPHRASE placeholder in the .env file with your actual GPG passphrase."));
    }
    if env::var("GIT_REMOTE_URL").unwrap_or_default() == "https://github.com/your/repo.git" {
        return Err(anyhow::anyhow!("Please replace the GIT_REMOTE_URL placeholder in the .env file with the actual URL of your GitHub repository."));
    }
    if env::var("JWT_SECRET").unwrap_or_default() == "your_JWT_passphrase_placeholder" {
        return Err(anyhow::anyhow!("Please replace the JWT_SECRET placeholder in the .env file with your actual JWT secret."));
    }

    // Initialize OmniXMetry for logging and metrics
    let omnixmetry = OmniXMetry::init()?;
    setup_global_subscriber(omnixmetry.clone())?;
    info!("OmniXMetry initialized successfully.");

    // Initialize OmniXErrorManager for error handling
    let error_manager_config = OmniXErrorManagerConfig {
        max_retries: env::var("MAX_RETRIES").unwrap_or_else(|_| "3".to_string()).parse().unwrap_or(3),
        circuit_breaker_threshold: *CIRCUIT_BREAKER_THRESHOLD,
        circuit_breaker_duration: *CIRCUIT_BREAKER_DURATION,
        base_delay: *BASE_DELAY,
        max_delay: *MAX_DELAY,
        timeout: *DEFAULT_TIMEOUT,
    };
    let omnix_error_manager = OmniXErrorManager::new(error_manager_config.clone());
    info!("OmniXErrorManager initialized successfully.");

    // Use the omnix_error_manager to ensure it's not unused
    let _ = &omnix_error_manager;

    // Process command-line arguments
    let args: Vec<String> = args().collect(); // Collect args into a vector

    // Check for the minimum number of arguments
    if args.len() < 2 {
        return Err(anyhow::anyhow!("Usage: {} --version", args[0]));
    }

    // Process command-line arguments
    match args[1].as_str() {
        "--version" => {
            println!("xynpro version 0.1.0");
            return Ok(());
        }
        _ => {
            // Future commands can be added here
            info!("No valid command provided. Exiting.");
        }
    }

    Ok(())
}
"##.to_string()
}

fn generate_constants_content() -> String {
    r#"// src/constants/mod.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[CONSTANTS]Xyn>=====S===t===u===d===i===o===s======[R|$>

use lazy_static::lazy_static;
use std::time::Duration;
use tracing::Level;
use std::env;

pub const ARCMOON_SIGNATURE: &str = "~=#######D]======A===r===c====M===o===o===n=====<Lord[{}]Xyn>=====S===t===u===d===i===o===s======[R|$>";
pub const PROJECT_DIRECTORIES: &[&str] = &[
    "src/omnixtracker",
    "src/constants",
    "src/utils",
    "tests",
    "Xdocs",
    "Xtls",
];

pub const AUTHOR_NAME: &str = "Lord Xyn";
pub const AUTHOR_EMAIL: &str = "LordXyn@proton.me";
pub const GITHUB_URL: &str = "https://github.com/arcmoonstudios";

pub const LICENSE_YEAR: &str = "2024";
pub const LICENSE_HOLDER: &str = "Carl Liu";

pub const PASSWORD_SALT_LENGTH: usize = 32;
pub const PASSWORD_HASH_ITERATIONS: u32 = 100_000;
pub const JWT_EXPIRATION: i64 = 3600;
pub const RATE_LIMIT_WINDOW: u64 = 60;
pub const RATE_LIMIT_MAX_REQUESTS: u32 = 100;
pub const ENABLE_EXPERIMENTAL_FEATURES: bool = false;
pub const USE_LEGACY_AUTH: bool = false;

// Constants related to Git
pub const GIT_REMOTE: &str = "origin"; // Default remote name
pub const GIT_BRANCH: &str = "main"; // Default branch name
pub const GIT_COMMIT_MESSAGE: &str = "Automated update via xyngit"; // Default commit message
pub const MAX_RETRIES: usize = 3; // Default max retries for Git push
pub const RETRY_DELAY: Duration = Duration::from_secs(2); // Default delay between retries

lazy_static! {
    pub static ref PROMETHEUS_LISTENER: String =
        env::var("PROMETHEUS_LISTENER").unwrap_or_else(|_| "0.0.0.0:9001".to_string());
    pub static ref PROMETHEUS_TEST_LISTENER: String =
        env::var("PROMETHEUS_TEST_LISTENER").unwrap_or_else(|_| "127.0.0.1:0".to_string());
    pub static ref INITIAL_LOG_LEVEL: Level = env::var("INITIAL_LOG_LEVEL")
        .map(|v| v.parse().unwrap_or(Level::INFO))
        .unwrap_or(Level::INFO);
    pub static ref LOG_FILE_PATH: String =
        env::var("LOG_FILE_PATH").unwrap_or_else(|_| "xynpro.log".to_string());
    pub static ref CIRCUIT_BREAKER_THRESHOLD: usize = env::var("CIRCUIT_BREAKER_THRESHOLD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    pub static ref CIRCUIT_BREAKER_DURATION: Duration = Duration::from_secs(
        env::var("CIRCUIT_BREAKER_DURATION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60)
    );
    pub static ref BASE_DELAY: Duration = Duration::from_millis(
        env::var("BASE_DELAY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100)
    );
    pub static ref MAX_DELAY: Duration = Duration::from_secs(
        env::var("MAX_DELAY")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10)
    );
    pub static ref DEFAULT_TIMEOUT: Duration = Duration::from_secs(
        env::var("DEFAULT_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30)
    );
}

pub fn get_max_retries() -> usize {
    env::var("MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(MAX_RETRIES)
}
"#.to_string()
}

fn generate_lib_rs_content() -> String {
    r#"// src/lib.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[LIB]Xyn>=====S===t===u===d===i===o===s======[R|$>

pub mod omnixtracker;
pub mod constants;
pub mod utils;

pub use crate::omnixtracker::{OmniXMetry, OmniXErrorManager, OmniXErrorManagerConfig, setup_global_subscriber};
pub use crate::constants::*;
pub use crate::utils::{update_git, LordXynSignatureLine};
"#.to_string()
}

fn generate_env_content() -> String {
    let jwt_secret = generate_jwt_secret();
    format!(
        r#"# ~=#######D]======A===r===c====M===o===o===n=====<Lord[ENV]Xyn>=====S===t===u===d===i===o===s======[R|$>
JWT_SECRET={}
PROMETHEUS_LISTENER=0.0.0.0:9001
INITIAL_LOG_LEVEL=INFO
GIT_REMOTE=origin
GIT_BRANCH=main
GIT_COMMIT_MESSAGE="Automated update via xyngit"
XYNGIT_COMMAND=xyngit
PASSWORD_SALT_LENGTH=32
PASSWORD_HASH_ITERATIONS=100000
JWT_EXPIRATION=3600
RATE_LIMIT_WINDOW=60
RATE_LIMIT_MAX_REQUESTS=100
ENABLE_EXPERIMENTAL_FEATURES=false
USE_LEGACY_AUTH=false
REPO_PATH=/path/to/your/repo
GIT_REMOTE_URL=https://github.com/your/repo.git
GPG_PASSPHRASE=your_gpg_passphrase_placeholder
"#,
        jwt_secret
    )
}

fn generate_build_rs_content() -> String {
    r##"// build.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[BUILD]Xyn>=====S===t===u===d===i===o===s======[R|$>

use std::path::{Path, Component};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use walkdir::WalkDir;
use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    println!("cargo:rerun-if-env-changed=CONFIG_PATH");

    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_string());

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("config.rs");
    let mut f = File::create(&dest_path).expect("Could not create config.rs");
    writeln!(f, "pub const CONFIG_PATH: &str = \"{}\";", config_path)
        .expect("Could not write to config.rs");

    add_custom_headers(&env::current_dir()?)?;

    Ok(())
}

fn add_custom_headers(project_path: &Path) -> Result<()> {
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().map(|s| s.starts_with('.')).unwrap_or(false))
    {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if let Some(header) = generate_custom_header(project_path, path, extension) {
                add_header_to_file(path, &header)?; // Add header to file (enforces on line 1)
            }
        }
    }
    Ok(())
}

fn generate_custom_header(project_root: &Path, path: &Path, extension: &str) -> Option<String> {
    // Ignore certain extensions
    if extension == "toml" {
        return None;
    }

    // Get the relative path
    let relative_path = path.strip_prefix(project_root).unwrap_or(path);
    let components: Vec<_> = relative_path.components().collect();

    // Generate the module name
    let module_name = if components.is_empty() {
        "UNKNOWN".to_string()
    } else if components.len() == 1 {
        // Files in the root directory
        let file_name = components[0].as_os_str().to_str().unwrap_or("UNKNOWN");
        if file_name == "build.rs" {
            "BUILD".to_string()
        } else {
            file_name.split('.').next().unwrap_or("UNKNOWN").to_uppercase()
        }
    } else if components.len() == 2 && components[0].as_os_str() == "src" {
        // Files directly under src/
        components[1].as_os_str().to_str().unwrap_or("UNKNOWN").split('.').next().unwrap_or("UNKNOWN").to_uppercase()
    } else {
        // Files in subdirectories
        components.iter().rev().nth(1)
            .and_then(|c| match c {
                Component::Normal(name) => name.to_str(),
                _ => None,
            })
            .unwrap_or("UNKNOWN")
            .to_uppercase()
    };

    let module_name = module_name.replace('_', "-");

    // Determine the comment syntax based on file extension
    let comment_syntax = match extension {
        "rs" | "js" | "ts" | "jsx" | "tsx" | "css" | "scss" => "//",
        "py" | "sh" | "bash" | "md" => "#",
        "html" | "xml" => "<!--",
        "sql" | "txt" => "--",
        _ => return None,
    };

    let display_path = relative_path.display().to_string();

    // Return the custom header string
    Some(format!(
        "{} {} ~=#######D]======A===r===c====M===o===o===n=====<Lord[{}]Xyn>=====S===t===u===d===i===o===s======[R|$>\n",
        comment_syntax,
        display_path,
        module_name
    ))
}

fn add_header_to_file(path: &Path, new_header: &str) -> Result<()> {
    let mut content = String::new();
    {
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut new_lines = Vec::new();
    let mut header_added = false;

    // Enforce header is added on line 1
    if !lines.is_empty() {
        if is_header(lines[0]) {
            // If the first line already contains a valid header, we don't add a new one.
            new_lines.push(lines[0]);
            header_added = true;
        } else {
            // Add the new header to the first line
            new_lines.push(new_header.trim_end());
            header_added = true;
        }
    }

    // Append the rest of the content (excluding existing signatures in the first 10 lines)
    for (i, &line) in lines.iter().enumerate() {
        if !is_header(line) || i >= 10 {
            new_lines.push(line);
        }
    }

    if !header_added {
        // If no header has been added, insert it at the beginning of the file
        new_lines.insert(0, new_header.trim_end());
    }

    // Write the new content to the file
    let new_content = new_lines.join("\n");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(new_content.as_bytes())?;

    Ok(())
}

fn is_header(line: &str) -> bool {
    // Check for specific header components
    (line.contains("//src") || line.contains("// src") || line.contains("// build") || line.contains("# README") || line.contains("// tests")) &&
    line.contains("~=#######D]======A===r===c====M===o===o===n=====<Lord[") &&
    line.contains("]Xyn>=====S===t===u===d===i===o===s======[R|$>")
}
"##.to_string()
}

fn generate_lib_rs_content() -> String {
    r#"// src/lib.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[LIB]Xyn>=====S===t===u===d===i===o===s======[R|$>

// Core modules
pub mod omnixtracker;
pub mod constants;
pub mod utils;

// Re-exports for convenient access
pub use crate::omnixtracker::{
    OmniXMetry,
    OmniXErrorManager,
    OmniXErrorManagerConfig,
    setup_global_subscriber,
    OmniXError,
};
pub use crate::constants::*;
pub use crate::utils::LordXynSignatureLine;

// Note: `update_git` is not re-exported here as it's not defined in the current utils module
"#.to_string()
}

fn generate_readme_content(project_name: &str) -> String {
    format!(
        r#"~=#######D]======A===r===c====M===o===o===n=====<Lord[README]Xyn>=====S===t===u===d===i===o===s======[R|$>

<p align="center">
  <img src="https://tinypic.host/images/2024/09/30/LordXyn.jpeg" alt="ArcMoon Studios Logo" width="200"/>
</p>

# 🚀 {} 🦀

{} is a Rust project created using the magical XynPro Initializer. Prepare to be amazed!

## ✨ Features

- 📁 Creates a standardized Rust project structure
- 🎨 Adds custom headers to Rust files (.rs)
- 📜 Generates a LICENSE file with MIT License
- 📚 Creates a README.md with project information and acknowledgements
- 🐙 Initializes a Git repository
- 📦 Sets up a Cargo.toml with predefined dependencies

## 🛠️ Usage

To unleash the power of this project, run the following command in your terminal:

```bash
cargo run

## 🗂️ Project Structure
Behold, the glorious structure of your project:

{}/
├── 📂 src/
│   ├── 📂 constants/
│   │   └── 📄 mod.rs
│   ├── 📂 omnixtracker/
│   │   ├── 📄 omnixerror.rs
│   │   ├── 📄 omnixmetry.rs 
│   │   └── 📄 mod.rs
│   ├── 📂 utils/
│   │   ├── 📄 lxsl.rs 
│   │   └── 📄 mod.rs
│   └── 📄 lib.rs
│   └── 📄 main.rs
├── 📂 tests/
├── 📄 .gitignore
├── 📄 Cargo.toml
├── 📄 LICENSE
└── 📄 README.md

📜 License
This project is licensed under the MIT License. See the LICENSE file for all the legal jazz.
🧙‍♂️ Author
Lord Xyn (LordXyn@proton.me)
Github: https://github.com/arcmoonstudios
🙏 Acknowledgements

💼 Insurance Sales - for motivating me to find a new trade.
🧠 Liner AI - for having the best AI resource on research papers and documents.
🤖 ChatGPT - for generating enough terrible code that it inspired me to learn programming.
👩‍💼 Luna - an "overly-attemptive secretary" personality AI coding assistant I created on Claude.ai.
👪 My Parents - for never giving up on me as a person, regardless of my struggle with commitment.
👶 My Children - for giving me the motivation to exist and persevere.
❤️ Valina - for her love and support, and dealing with my crazy self.
"#,
        project_name.to_uppercase(),
        project_name,
        project_name
    )
}

fn generate_license_content() -> String {
    format!(
        r#"MIT License

Copyright (c) {} {}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."#,
        LICENSE_YEAR, LICENSE_HOLDER
    )
}

fn generate_gitignore_content() -> String {
    r#"# ~=#######D]==A==r==c===M==o==o==n=<Lord[GITIGNORE]Xyn>=S==t==u==d==i==o==s==[R|$>

# Generated by Cargo
/target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information: https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# Backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# Editor directories and files
.idea/
.vscode/
*.swp
*.swo

# Operating System files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db
"#
    .to_string()
}

fn generate_omnixerror_content() -> String {
    r#"// src/omnixtracker/omnixerror.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[OMNIXTRACKER]Xyn>=====S===t===u===d===i===o===s======[R|$>

use crate::omnixtracker::omnixmetry::OmniXMetry;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use parking_lot::{Mutex, RwLock};
use tracing::{error, info, warn};
use git2::Error as GitError;
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum OmniXError {
    #[error("Operation failed during {operation}: {details}")]
    OperationFailed { operation: String, details: String },
    #[error("Retry limit exceeded after {retries} attempts: {last_error}")]
    RetryLimitExceeded { retries: usize, last_error: String },
    #[error("Circuit breaker activated after {count} errors in {duration:?}")]
    CircuitBreakerActivated { count: usize, duration: Duration },
    #[error("Operation timed out after {duration:?}")]
    OperationTimeout { duration: Duration },
    #[error("File system error: {0}")]
    FileSystemError(String),
    #[error("Environment variable error: {0}")]
    EnvVarError(String),
    #[error("Project creation error: {0}")]
    ProjectCreationError(String),
    #[error("Metrics initialization error: {0}")]
    MetricsInitError(String),
    #[error("Logging error: {0}")]
    LoggingError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl OmniXError {
    pub fn log(&self) {
        match self {
            OmniXError::OperationFailed { .. } | OmniXError::RetryLimitExceeded { .. } => {
                error!("{}", self)
            }
            OmniXError::CircuitBreakerActivated { .. } | OmniXError::OperationTimeout { .. } => {
                warn!("{}", self)
            }
            _ => {
                info!("{}", self)
            }
        }
    }
}

pub fn handle_build_error(error: Box<dyn std::error::Error>) -> OmniXError {
    match error.downcast::<std::io::Error>() {
        Ok(io_error) => OmniXError::FileSystemError(io_error.to_string()),
        Err(error) => OmniXError::OperationFailed {
            operation: "Build".to_string(),
            details: error.to_string(),
        },
    }
}

pub fn handle_main_error(error: Box<dyn std::error::Error>) -> OmniXError {
    match error.downcast::<std::io::Error>() {
        Ok(io_error) => OmniXError::FileSystemError(io_error.to_string()),
        Err(error) => match error.downcast::<std::env::VarError>() {
            Ok(var_error) => OmniXError::EnvVarError(var_error.to_string()),
            Err(error) => OmniXError::ProjectCreationError(error.to_string()),
        },
    }
}

pub fn handle_metrics_error(error: impl std::error::Error) -> OmniXError {
    OmniXError::MetricsInitError(error.to_string())
}

#[derive(Debug, Clone)]
pub struct OmniXErrorManagerConfig {
    pub max_retries: usize,
    pub circuit_breaker_threshold: usize,
    pub circuit_breaker_duration: Duration,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub timeout: Duration,
}

impl Default for OmniXErrorManagerConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            circuit_breaker_threshold: 10,
            circuit_breaker_duration: Duration::from_secs(60),
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            timeout: Duration::from_secs(30),
        }
    }
}

impl From<GitError> for OmniXError {
    fn from(err: GitError) -> Self {
        OmniXError::OperationFailed {
            operation: "Git operation failed".to_string(),
            details: err.to_string(),
        }
    }
}

impl From<anyhow::Error> for OmniXError {
    fn from(err: anyhow::Error) -> Self {
        OmniXError::OperationFailed {
            operation: "Unknown operation".to_string(),
            details: err.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed,
    Open(Instant),
    HalfOpen,
}

impl fmt::Display for CircuitState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "Closed"),
            CircuitState::Open(_) => write!(f, "Open"),
            CircuitState::HalfOpen => write!(f, "Half-Open"),
        }
    }
}

pub struct OmniXErrorManager {
    error_count: AtomicUsize,
    config: RwLock<OmniXErrorManagerConfig>,
    circuit_state: Mutex<CircuitState>,
    last_error_time: Mutex<Instant>,
    half_open_trial_count: AtomicUsize,
}

impl OmniXErrorManager {
    pub fn new(config: OmniXErrorManagerConfig) -> Self {
        Self {
            error_count: AtomicUsize::new(0),
            config: RwLock::new(config),
            circuit_state: Mutex::new(CircuitState::Closed),
            last_error_time: Mutex::new(Instant::now()),
            half_open_trial_count: AtomicUsize::new(0),
        }
    }

    pub async fn handle_error<T, F, Fut>(
        &self,
        operation: F,
        metrics: &OmniXMetry,
    ) -> Result<T, OmniXError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, OmniXError>>,
    {
        let config = self.config.read();

        if !self.check_circuit_state() {
            metrics.increment_counter("error_manager.circuit_opened".to_string(), 1);
            return Err(OmniXError::CircuitBreakerActivated {
                count: self.error_count.load(Ordering::Relaxed),
                duration: config.circuit_breaker_duration,
            });
        }

        for retries in 0..config.max_retries {
            let start_time = Instant::now();
            match tokio::time::timeout(config.timeout, operation()).await {
                Ok(Ok(result)) => {
                    self.error_count.store(0, Ordering::Relaxed);
                    self.close_circuit();
                    metrics.increment_counter("error_manager.successes".to_string(), 1);
                    metrics.update_gauge("error_manager.operation_latency".to_string(), start_time.elapsed().as_secs_f64());
                    info!("Operation succeeded on attempt {}", retries + 1);
                    return Ok(result);
                }
                Ok(Err(e)) => {
                    e.log();
                    self.error_count.fetch_add(1, Ordering::Relaxed);
                    *self.last_error_time.lock() = Instant::now();

                    metrics.increment_counter("error_manager.failures".to_string(), 1);
                    metrics.increment_counter(format!("error_manager.failures.{}", e), 1);

                    if self.error_count.load(Ordering::Relaxed) >= config.circuit_breaker_threshold {
                        self.open_circuit();
                        metrics.increment_counter("error_manager.circuit_tripped".to_string(), 1);
                        error!(
                            "Circuit breaker tripped after {} consecutive failures",
                            self.error_count.load(Ordering::Relaxed)
                        );
                        return Err(OmniXError::CircuitBreakerActivated {
                            count: self.error_count.load(Ordering::Relaxed),
                            duration: config.circuit_breaker_duration,
                        });
                    }

                    if retries == config.max_retries - 1 {
                        metrics.increment_counter("error_manager.max_retries_exceeded".to_string(), 1);
                        metrics.update_gauge("error_manager.operation_latency".to_string(), start_time.elapsed().as_secs_f64());
                        return Err(OmniXError::RetryLimitExceeded {
                            retries: retries + 1,
                            last_error: e.to_string(),
                        });
                    }

                    let delay = config
                        .base_delay
                        .mul_f32(2_f32.powi(retries as i32))
                        .min(config.max_delay);
                    tokio::time::sleep(delay).await;
                }
                Err(_) => {
                    metrics.increment_counter("error_manager.operation_timeout".to_string(), 1);
                    metrics.update_gauge("error_manager.operation_latency".to_string(), config.timeout.as_secs_f64());
                    return Err(OmniXError::OperationTimeout {
                        duration: config.timeout,
                    });
                }
            }
        }

        Err(OmniXError::RetryLimitExceeded {
            retries: config.max_retries,
            last_error: "Maximum retries reached".to_string(),
        })
    }

    fn check_circuit_state(&self) -> bool {
        let mut circuit_state = self.circuit_state.lock();
        match *circuit_state {
            CircuitState::Closed => true,
            CircuitState::Open(opened_at) => {
                let config = self.config.read();
                if opened_at.elapsed() >= config.circuit_breaker_duration {
                    *circuit_state = CircuitState::HalfOpen;
                    self.half_open_trial_count.store(0, Ordering::Relaxed);
                    warn!("Circuit breaker transitioning to HalfOpen state");
                    true
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => {
                if self.half_open_trial_count.fetch_add(1, Ordering::Relaxed) < 1 {
                    info!("Circuit breaker is Half-Open; allowing trial operation");
                    true
                } else {
                    warn!("Circuit breaker is Half-Open; trial limit reached");
                    false
                }
            }
        }
    }

    fn open_circuit(&self) {
        let mut circuit_state = self.circuit_state.lock();
        *circuit_state = CircuitState::Open(Instant::now());
        self.half_open_trial_count.store(0, Ordering::Relaxed);
        warn!("Circuit breaker opened");
    }

    fn close_circuit(&self) {
        let mut circuit_state = self.circuit_state.lock();
        *circuit_state = CircuitState::Closed;
        self.half_open_trial_count.store(0, Ordering::Relaxed);
        info!("Circuit breaker closed");
    }

    pub fn update_config(&self, new_config: OmniXErrorManagerConfig) {
        let mut config = self.config.write();
        *config = new_config;
        info!("OmniXErrorManager configuration updated");
    }
}

impl fmt::Debug for OmniXErrorManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OmniXErrorManager")
            .field("error_count", &self.error_count.load(Ordering::Relaxed))
            .field("config", &*self.config.read())
            .field("circuit_state", &self.circuit_state.lock())
            .field("last_error_time", &*self.last_error_time.lock())
            .field("half_open_trial_count", &self.half_open_trial_count.load(Ordering::Relaxed))
            .finish()
    }
}
"#.to_string()
}

fn generate_omnixmetry_content() -> String {
    r#"// src/omnixtracker/omnixmetry.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[OMNIXTRACKER]Xyn>=====S===t===u===d===i===o===s======[R|$>

use crate::constants::{PROMETHEUS_LISTENER, PROMETHEUS_TEST_LISTENER, INITIAL_LOG_LEVEL, LOG_FILE_PATH};
use tracing_subscriber::{Layer, Registry, EnvFilter};
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::{Event, Level, Metadata, Subscriber};
use anyhow::{Context, Result as AnyhowResult};
use tracing_subscriber::prelude::*;
use std::fmt::Write as FmtWrite;
use once_cell::sync::OnceCell;
use std::net::TcpListener;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufWriter};
use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use chrono::Local;
use regex::Regex;
use colored::*;

static PROMETHEUS_RECORDER: OnceCell<()> = OnceCell::new();

#[derive(Clone)]
pub struct OmniXMetry {
    log_file: Arc<RwLock<Option<BufWriter<File>>>>,
    log_level: Arc<RwLock<Level>>,
}

impl OmniXMetry {
    pub fn init() -> AnyhowResult<Self> {
        // Initialize Prometheus recorder only once
        PROMETHEUS_RECORDER.get_or_try_init(|| {
            let listener_result = if cfg!(test) {
                TcpListener::bind(&*PROMETHEUS_TEST_LISTENER)
                    .context("Failed to bind to test port")
            } else {
                TcpListener::bind(&*PROMETHEUS_LISTENER)
                    .context("Failed to bind to configured Prometheus listener")
            };
        
            let listener = listener_result?;
            println!("Prometheus listening on: {}", listener.local_addr()?);
        
            PrometheusBuilder::new()
                .with_http_listener(listener.local_addr()?)
                .install_recorder()
                .context("Failed to set global Prometheus recorder")?;
        
            Ok::<(), anyhow::Error>(())
        }).context("Failed to initialize Prometheus recorder")?;

        // Open the log file
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*LOG_FILE_PATH)
            .context("Failed to open log file")?;

        let buffered_file = BufWriter::new(log_file);

        Ok(Self {
            log_level: Arc::new(RwLock::new(*INITIAL_LOG_LEVEL)),
            log_file: Arc::new(RwLock::new(Some(buffered_file))),
        })
    }

    pub fn set_log_level(&self, level: Level) {
        let mut log_level = self.log_level.write();
        *log_level = level;
    }

    pub fn get_log_level(&self) -> Level {
        *self.log_level.read()
    }

    pub fn is_log_file_initialized(&self) -> bool {
        self.log_file.read().is_some()
    }

    pub fn increment_counter(&self, key_name: String, value: u64) {
        let counter = metrics::counter!(key_name.clone(), "value" => value.to_string());
        counter.increment(value);
    }
    
    pub fn update_gauge(&self, key_name: String, value: f64) {
        let gauge = metrics::gauge!(key_name.clone(), "value" => value.to_string());
        gauge.set(value);
    }
    
    pub fn record_histogram(&self, key_name: String, value: f64) {
        let histogram = metrics::histogram!(key_name.clone(), "value" => value.to_string());
        histogram.record(value);
    }    

    pub fn rotate_log_file(&self) -> AnyhowResult<()> {
        let mut log_file_lock = self.log_file.write();
        if let Some(mut file) = log_file_lock.take() {
            file.flush().context("Failed to flush log file before rotation")?;
            drop(file);
    
            let xdocs_path = Path::new("Xdocs");
            std::fs::create_dir_all(&xdocs_path)
                .context("Failed to create Xdocs directory")?;
    
            let old_log_path = Path::new(&*LOG_FILE_PATH);
            if old_log_path.exists() {
                let new_log_path = self.generate_new_log_path(xdocs_path)?;
                std::fs::rename(&old_log_path, &new_log_path)
                    .context("Failed to rename log file")?;
            } else {
                println!("Warning: Log file doesn't exist. Creating a new one.");
            }
        }
    
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&*LOG_FILE_PATH)
            .context("Failed to open new log file")?;
        
        let buffered_file = BufWriter::new(new_file);
        *log_file_lock = Some(buffered_file);
    
        Ok(())
    }

    fn generate_new_log_path(&self, xdocs_path: &Path) -> AnyhowResult<PathBuf> {
        let date_time = Local::now().format("%m-%d-%Y_%H-%M");
        let base_name = format!("{}", date_time);
        
        let regex = Regex::new(&format!(r"^{}_(\d{{3}})\.log$", regex::escape(&base_name)))
            .context("Failed to create regex for log file naming")?;

        let mut highest_number = 0;
        for entry in std::fs::read_dir(xdocs_path)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            if let Some(captures) = regex.captures(&file_name) {
                if let Some(number_match) = captures.get(1) {
                    if let Ok(number) = number_match.as_str().parse::<u32>() {
                        highest_number = highest_number.max(number);
                    }
                }
            }
        }

        let new_number = highest_number + 1;
        let new_file_name = format!("{}_{:03}.log", base_name, new_number);
        Ok(xdocs_path.join(new_file_name))
    }

    pub fn write_log(&self, log_entry: &str) -> std::io::Result<()> {
        if let Some(ref mut file) = *self.log_file.write() {
            writeln!(file, "{}", log_entry)?;
            file.flush()?;
        }
        Ok(())
    }
}

impl<S> Layer<S> for OmniXMetry
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        if *event.metadata().level() <= self.get_log_level() {
            let level_str = match *event.metadata().level() {
                Level::ERROR => "ERROR".red(),
                Level::WARN => "WARN ".yellow(),
                Level::INFO => "INFO ".green(),
                Level::DEBUG => "DEBUG".blue(),
                Level::TRACE => "TRACE".magenta(),
            };

            let mut fields = String::new();
            {
                let mut visitor = FieldVisitor { output: &mut fields };
                event.record(&mut visitor);
            }

            let log_entry = format!(
                "{} [{}] {}: {}",
                Local::now().format("%B, %d %Y @ %I:%M %p"),
                level_str,
                event.metadata().target(),
                fields
            );

            println!("{}", log_entry);

            if let Err(e) = self.write_log(&log_entry) {
                eprintln!("Failed to write to log file: {}", e);
            }
        }
    }

    fn enabled(
        &self,
        metadata: &Metadata<'_>,
        _: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        *metadata.level() <= self.get_log_level()
    }
}

struct FieldVisitor<'a> {
    output: &'a mut String,
}

impl<'a> tracing::field::Visit for FieldVisitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if !self.output.is_empty() {
            self.output.push_str(", ");
        }
        let _ = write!(self.output, "{} = {:?}", field.name(), value);
    }
}

pub fn setup_global_subscriber(omnixmetry: OmniXMetry) -> AnyhowResult<()> {
    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;
    let subscriber = Registry::default().with(env_filter).with(omnixmetry);
    tracing::subscriber::set_global_default(subscriber).context("Failed to set global subscriber")?;
    Ok(())
}
"#.to_string()
}

fn generate_lxsl_content() -> String {
    r##"// src/utils/lxsl.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[UTILS]Xyn>=====S===t===u===d===i===o===s======[R|$>

use crate::constants::ARCMOON_SIGNATURE;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;

pub struct LordXynSignatureLine;

impl LordXynSignatureLine {
    pub fn generate_signature_line(file_path: &str) -> String {
        let normalized_path = Path::new(file_path)
            .to_str()
            .unwrap_or(file_path)
            .replace('\\', "/");

        let path_parts: Vec<&str> = normalized_path.split('/').collect();

        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let comment_prefix = Self::get_comment_prefix(extension);

        if comment_prefix.is_empty() {
            return String::new();
        }

        let signature_path = Self::build_signature_path(&path_parts);
        let xyn_signature = Self::build_xyn_signature(&path_parts);

        format!(
            "{} {} {}",
            comment_prefix,
            signature_path,
            ARCMOON_SIGNATURE.replace("{}", &xyn_signature)
        )
    }

    pub fn enforce_signature_at_line_1(file_path: &str) -> io::Result<()> {
        let path = Path::new(file_path);

        if Self::should_skip_file(file_path) {
            return Ok(());
        }

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let comment_prefix = Self::get_comment_prefix(extension);

        if comment_prefix.is_empty() {
            return Ok(());
        }

        let file = OpenOptions::new().read(true).open(&path)?;
        let lines: Vec<String> = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;

        let cleaned_lines: Vec<String> = lines
            .into_iter()
            .enumerate()
            .filter_map(|(idx, line)| {
                if idx < 10 && (Self::is_invalid_xyn_signature(&line) || Self::is_xyn_signature(&line)) {
                    None
                } else {
                    Some(line)
                }
            })
            .collect();

        let signature = Self::generate_signature_line(file_path);
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

        if !signature.is_empty() {
            writeln!(file, "{}", signature)?;
        }

        for line in cleaned_lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    pub fn build_signature_path(path_parts: &[&str]) -> String {
        if path_parts.len() > 1 {
            path_parts
                .iter()
                .take(path_parts.len() - 1)
                .map(|comp| format!("{}/", comp))
                .collect()
        } else {
            String::new()
        }
    }

    pub fn build_xyn_signature(path_parts: &[&str]) -> String {
        path_parts
            .last()
            .map(|last_part| {
                Path::new(last_part)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("UNKNOWN")
                    .to_uppercase()
                    .replace('_', "-")
            })
            .unwrap_or_else(|| "UNKNOWN".to_string())
    }

    pub fn get_comment_prefix(extension: &str) -> &str {
        match extension {
            "rs" | "js" | "ts" | "cpp" | "c" | "java" => "//",
            "py" | "sh" | "rb" | "pl" => "#",
            "html" | "xml" => "<!--",
            "css" | "scss" => "/*",
            "sql" | "txt" => "--",
            _ => "",
        }
    }

    pub fn is_invalid_xyn_signature(line: &str) -> bool {
        let has_valid_path = line.contains("// src") || line.contains("// build") || line.contains("// tests");
        let has_signature_format = line.contains("~=#######D]") && line.contains("<Lord[") && line.contains("]Xyn>");
        has_valid_path && !has_signature_format
    }

    pub fn is_xyn_signature(line: &str) -> bool {
        let has_valid_path = line.contains("// src") || line.contains("// build") || line.contains("// tests");
        let has_signature_format = line.contains("~=#######D]") && line.contains("<Lord[") && line.contains("]Xyn>");
        has_valid_path && has_signature_format
    }

    pub fn should_skip_file(file_path: &str) -> bool {
        let skip_extensions = [
            "lock", "log", "png", "jpg", "jpeg", "gif", "pyc", "toml", "exe", "dll", "so", "bin",
        ];

        Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| skip_extensions.contains(&ext))
    }
}
"##.to_string()
}

fn generate_utils_mod_content() -> String {
    r#"// src/utils/mod.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[UTILS]Xyn>=====S===t===u===d===i===o===s======[R|$>

pub mod lxsl;

pub use lxsl::LordXynSignatureLine;
"#.to_string()
}

fn generate_constants_tests_content(project_name: &str) -> String {
    format!(r#"// tests/constants_tests.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[TESTS]Xyn>=====S===t===u===d===i===o===s======[R|$>

#[cfg(test)]
mod tests {
    use test6::constants::*;
    use std::env;

    #[test]
    fn test_project_directories() {
        let expected_directories = vec![
            "src/omnixtracker",
            "src/constants",
            "src/utils",
            "tests",
            "Xdocs",
            "Xtls",
        ];
        assert_eq!(PROJECT_DIRECTORIES.to_vec(), expected_directories);
    }

    #[test]
    fn test_password_salt_length() {
        assert_eq!(PASSWORD_SALT_LENGTH, 32);
    }

    #[test]
    fn test_password_hash_iterations() {
        assert_eq!(PASSWORD_HASH_ITERATIONS, 100_000);
    }

    #[test]
    fn test_jwt_expiration() {
        assert_eq!(JWT_EXPIRATION, 3600);
    }

    #[test]
    fn test_rate_limit_window() {
        assert_eq!(RATE_LIMIT_WINDOW, 60);
    }

    #[test]
    fn test_rate_limit_max_requests() {
        assert_eq!(RATE_LIMIT_MAX_REQUESTS, 100);
    }

    #[test]
    fn test_enable_experimental_features() {
        assert_eq!(ENABLE_EXPERIMENTAL_FEATURES, false);
    }

    #[test]
    fn test_use_legacy_auth() {
        assert_eq!(USE_LEGACY_AUTH, false);
    }

    #[test]
    fn test_prometheus_listener_default() {
        env::remove_var("PROMETHEUS_LISTENER");
        assert_eq!(&*PROMETHEUS_LISTENER, "0.0.0.0:9001");
    }

    #[test]
    fn test_initial_log_level_default() {
        env::remove_var("INITIAL_LOG_LEVEL");
        assert_eq!(*INITIAL_LOG_LEVEL, tracing::Level::INFO);
    }

    #[test]
    fn test_log_file_path_default() {
        env::remove_var("LOG_FILE_PATH");
        assert_eq!(&*LOG_FILE_PATH, "xynpro.log");
    }

    #[test]
    fn test_git_remote_default() {
        env::remove_var("GIT_REMOTE");
        assert_eq!(&*GIT_REMOTE, "origin");
    }

    #[test]
    fn test_git_branch_default() {
        env::remove_var("GIT_BRANCH");
        assert_eq!(&*GIT_BRANCH, "main");
    }

    #[test]
    fn test_git_commit_message_default() {
        env::remove_var("GIT_COMMIT_MESSAGE");
        assert_eq!(&*GIT_COMMIT_MESSAGE, "Automated update via xyngit");
    }

    #[test]
    fn test_circuit_breaker_threshold_default() {
        env::remove_var("CIRCUIT_BREAKER_THRESHOLD");
        assert_eq!(*CIRCUIT_BREAKER_THRESHOLD, 10);
    }

    #[test]
    fn test_circuit_breaker_duration_default() {
        env::remove_var("CIRCUIT_BREAKER_DURATION");
        assert_eq!(*CIRCUIT_BREAKER_DURATION, std::time::Duration::from_secs(60));
    }

    #[test]
    fn test_base_delay_default() {
        env::remove_var("BASE_DELAY");
        assert_eq!(*BASE_DELAY, std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_max_delay_default() {
        env::remove_var("MAX_DELAY");
        assert_eq!(*MAX_DELAY, std::time::Duration::from_secs(10));
    }

    #[test]
    fn test_default_timeout_default() {
        env::remove_var("DEFAULT_TIMEOUT");
        assert_eq!(*DEFAULT_TIMEOUT, std::time::Duration::from_secs(30));
    }

    #[test]
    fn test_max_retries_default() {
        env::remove_var("MAX_RETRIES");
        assert_eq!(get_max_retries(), 3);
    }
}
"#, project_name)
}

fn generate_omnixerror_tests_content(project_name: &str) -> String {
    format!(r#"// tests/omnixerror_tests.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[TESTS]Xyn>=====S===t===u===d===i===o===s======[R|$>

#[cfg(test)]
mod tests {
    use test6::omnixtracker::OmniXError;
    use std::time::Duration;

    #[test]
    fn test_operation_failed_error() {
        let error = OmniXError::OperationFailed { 
            operation: "test".to_string(),
            details: "Some error occurred".to_string() 
        };
        assert_eq!(format!("{}", error), "Operation failed during test: Some error occurred");
    }

    #[test]
    fn test_retry_limit_exceeded_error() {
        let error = OmniXError::RetryLimitExceeded {
            retries: 5,
            last_error: "Last attempt failed".to_string(),
        };
        assert_eq!(format!("{}", error), "Retry limit exceeded after 5 attempts: Last attempt failed");
    }

    #[test]
    fn test_circuit_breaker_activated_error() {
        let error = OmniXError::CircuitBreakerActivated {
            count: 10,
            duration: Duration::from_secs(60),
        };
        assert_eq!(format!("{}", error), "Circuit breaker activated after 10 errors in 60s");
    }

    #[test]
    fn test_operation_timeout_error() {
        let error = OmniXError::OperationTimeout {
            duration: Duration::from_secs(30),
        };
        assert_eq!(format!("{}", error), "Operation timed out after 30s");
    }

    #[test]
    fn test_file_system_error() {
        let error = OmniXError::FileSystemError("Disk not found".to_string());
        assert_eq!(format!("{}", error), "File system error: Disk not found");
    }

    #[test]
    fn test_env_var_error() {
        let error = OmniXError::EnvVarError("Missing environment variable".to_string());
        assert_eq!(format!("{}", error), "Environment variable error: Missing environment variable");
    }

    #[test]
    fn test_project_creation_error() {
        let error = OmniXError::ProjectCreationError("Unable to create project".to_string());
        assert_eq!(format!("{}", error), "Project creation error: Unable to create project");
    }

    #[test]
    fn test_metrics_init_error() {
        let error = OmniXError::MetricsInitError("Failed to initialize metrics".to_string());
        assert_eq!(format!("{}", error), "Metrics initialization error: Failed to initialize metrics");
    }

    #[test]
    fn test_logging_error() {
        let error = OmniXError::LoggingError("Log file not found".to_string());
        assert_eq!(format!("{}", error), "Logging error: Log file not found");
    }

    #[test]
    fn test_database_error() {
        let error = OmniXError::DatabaseError("Connection failed".to_string());
        assert_eq!(format!("{}", error), "Database error: Connection failed");
    }

    #[test]
    fn test_network_error() {
        let error = OmniXError::NetworkError("Network unreachable".to_string());
        assert_eq!(format!("{}", error), "Network error: Network unreachable");
    }

    #[test]
    fn test_authentication_error() {
        let error = OmniXError::AuthenticationError("Invalid credentials".to_string());
        assert_eq!(format!("{}", error), "Authentication error: Invalid credentials");
    }

    #[test]
    fn test_authorization_error() {
        let error = OmniXError::AuthorizationError("Access denied".to_string());
        assert_eq!(format!("{}", error), "Authorization error: Access denied");
    }

    #[test]
    fn test_validation_error() {
        let error = OmniXError::ValidationError("Input is invalid".to_string());
        assert_eq!(format!("{}", error), "Validation error: Input is invalid");
    }
}
"#, project_name)
}

fn generate_omnixmetry_tests_content(project_name: &str) -> String {
    format!(r#"// tests/omnixmetry_tests.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[TESTS]Xyn>=====S===t===u===d===i===o===s======[R|$>

use test6::omnixtracker::OmniXMetry;
use test6::constants::{INITIAL_LOG_LEVEL, LOG_FILE_PATH};
use std::env;
use std::path::Path;
use tracing::Level;
use std::sync::Once;

static INIT: Once = Once::new();

fn initialize() {
    INIT.call_once(|| {
        env::remove_var("PROMETHEUS_LISTENER");
        env::remove_var("LOG_FILE_PATH");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        initialize();
        match OmniXMetry::init() {
            Ok(omnixmetry) => {
                assert_eq!(omnixmetry.get_log_level(), *INITIAL_LOG_LEVEL);
                assert!(omnixmetry.is_log_file_initialized());
            },
            Err(e) => {
                // Check if the error is due to Prometheus already being initialized
                if e.to_string().contains("metrics system was already initialized") {
                    println!("Prometheus recorder already initialized. Skipping this test.");
                } else {
                    panic!("Unexpected error: {}", e);
                }
            }
        }
    }

    #[test]
    fn test_log_level() {
        initialize();
        if let Ok(omnixmetry) = OmniXMetry::init() {
            assert_eq!(omnixmetry.get_log_level(), *INITIAL_LOG_LEVEL);
            
            omnixmetry.set_log_level(Level::DEBUG);
            assert_eq!(omnixmetry.get_log_level(), Level::DEBUG);
        } else {
            println!("Failed to initialize OmniXMetry. Skipping this test.");
        }
    }

// In tests/omnixmetry_tests.rs

#[test]
fn test_rotate_log_file() {
    initialize();
    if let Ok(omnixmetry) = OmniXMetry::init() {
        // Write something to the log file
        omnixmetry.write_log("Test log entry").unwrap();

        omnixmetry.rotate_log_file().unwrap();

        let xdocs_path = Path::new("Xdocs");
        assert!(xdocs_path.exists(), "Xdocs directory should exist");

        let rotated_files: Vec<_> = std::fs::read_dir(xdocs_path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        assert!(!rotated_files.is_empty(), "At least one rotated log file should exist in Xdocs/");
        assert!(omnixmetry.is_log_file_initialized(), "Log file should be initialized after rotation.");

        // Clean up
        std::fs::remove_dir_all(xdocs_path).unwrap();
        std::fs::remove_file(&*LOG_FILE_PATH).unwrap();
    } else {
        println!("Failed to initialize OmniXMetry. Skipping this test.");
    }
}
}
"#, project_name, project_name)
}

fn generate_utils_lxsl_tests_content(project_name: &str) -> String {
    format!(r#"// tests/utils_lxsl_tests.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[TESTS]Xyn>=====S===t===u===d===i===o===s======[R|$>

#[cfg(test)]
mod tests {
    use test6::utils::lxsl::LordXynSignatureLine;
    use std::io::{BufRead, Write};

    #[test]
    fn test_generate_signature_line() {
        let file_path = "src/utils/lxsl.rs"; // Valid Rust file for header generation
        let signature_line = LordXynSignatureLine::generate_signature_line(file_path);
    
        // Ensure that signature is properly generated for valid Rust file
        assert!(signature_line.starts_with("//"), "Signature should start with a comment prefix");
        assert!(signature_line.contains("src/"), "Signature should contain the file path");
        assert!(signature_line.contains("<Lord[LXSL]Xyn>"), "Signature should contain the correct LordXyn identifier");
        assert!(!signature_line.is_empty(), "Generated signature line should not be empty");
    }

    #[test]
    fn test_enforce_signature_at_line_1() {
        // Create a temporary file with a .rs extension
        let temp_file = tempfile::Builder::new()
            .suffix(".rs")
            .tempfile()
            .unwrap();
    
        let file_path = temp_file.path().to_str().unwrap().to_string();
    
        {
            // Write initial content to the temporary file
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .open(&file_path)
                .unwrap();
            writeln!(file, "Old line 1").unwrap();
            writeln!(file, "Old line 2").unwrap();
            file.flush().unwrap(); // Ensure all data is flushed to disk
        } // The file handle is dropped here
    
        // Enforce the signature at line 1
        LordXynSignatureLine::enforce_signature_at_line_1(&file_path).unwrap();
    
        // Read the updated file content
        let lines: Vec<String> = std::io::BufReader::new(std::fs::File::open(&file_path).unwrap())
            .lines()
            .collect::<Result<_, _>>()
            .unwrap();
    
        // Assertions to verify the test
        assert!(!lines.is_empty(), "File should have content after enforcing signature");
        assert!(lines[0].starts_with("//"), "First line should contain a comment prefix");
        assert!(lines[0].contains("<Lord[") && lines[0].contains("]Xyn>"), "First line should contain the LordXyn signature");
        assert_eq!(lines[1].trim(), "Old line 1", "Second line should be the original first line");
        assert_eq!(lines[2].trim(), "Old line 2", "Third line should be the original second line");
    }
    

    #[test]
    fn test_build_signature_path() {
        let path_parts = ["src", "utils", "lxsl.rs"];
        let signature_path = LordXynSignatureLine::build_signature_path(&path_parts);
        assert_eq!(signature_path, "src/utils/", "Signature path should be correctly built from path parts");
    }

    #[test]
    fn test_build_xyn_signature() {
        let path_parts = ["src", "utils", "lxsl.rs"];
        let xyn_signature = LordXynSignatureLine::build_xyn_signature(&path_parts);
        assert_eq!(xyn_signature, "LXSL", "Xyn signature should be correctly generated from the file name");
    }

    #[test]
    fn test_get_comment_prefix() {
        assert_eq!(LordXynSignatureLine::get_comment_prefix("rs"), "//", "Comment prefix for Rust files should be '//'");
        assert_eq!(LordXynSignatureLine::get_comment_prefix("py"), "#", "Comment prefix for Python files should be '#'");
        assert_eq!(LordXynSignatureLine::get_comment_prefix("html"), "<!--", "Comment prefix for HTML files should be '<!--'");
        assert_eq!(LordXynSignatureLine::get_comment_prefix("unknown"), "", "Comment prefix for unknown files should be empty");
    }

    #[test]
    fn test_is_invalid_xyn_signature() {
        let invalid_line = "// src/utils/lxsl.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[UTILS]====S===t===u===d===i===o===s======[R|$>";
        assert!(LordXynSignatureLine::is_invalid_xyn_signature(invalid_line), "Line should be identified as an invalid Xyn signature");
    }

    #[test]
    fn test_is_xyn_signature() {
        let valid_signature = "// src/utils/lxsl.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[LXSL]Xyn>=====S===t===u===d===i===o===s======[R|$>";
        assert!(LordXynSignatureLine::is_xyn_signature(valid_signature), "Line should be identified as a valid Xyn signature");
    }

    #[test]
    fn test_should_skip_file() {
        assert!(LordXynSignatureLine::should_skip_file("file.lock"), "File with .lock extension should be skipped");
        assert!(!LordXynSignatureLine::should_skip_file("file.rs"), "File with .rs extension should not be skipped");
    }
}
"#, project_name)
}

fn generate_xtls_xynpro_content() -> String {
    r#"-- Xtls/XynPro_Instructions-HowTo.txt ~=#######D]======A===r===c====M===o===o===n=====<Lord[XTLS]Xyn>=====S===t===u===d===i===o===s======[R|$>

XynPro Update Instructions

Update your src/main.rs file with this new code.
Rebuild your project:

cargo build --release

Copy the new binary to your ~/.cargo/bin directory:

cp ./target/release/xynpro ~/.cargo/bin/xynpro

Make sure the binary is executable:

chmod +x ~/.cargo/bin/xynpro

Test the new functionality:
a. Check the version:

Copyxynpro --version

This should output: "xynpro version 0.1.0"
b. Create a new project:

xynpro my_new_project

This should create a new project structure in a directory called "my_new_project"
If you encounter any issues, check the error messages and ensure all necessary dependencies are installed.
If everything works as expected, you can now use this updated version of xynpro to create projects and check its version.

Remember, if you need to make any further changes, you'll need to repeat steps 2-4 to rebuild and update the executable.
"#.to_string()
}

fn generate_xtls_xyntools_content() -> String {
    r#"-- Xtls/XynTools-HowTo.txt ~=#######D]======A===r===c====M===o===o===n=====<Lord[XTLS]Xyn>=====S===t===u===d===i===o===s======[R|$>

XynPro Command Guide

1. Lord Xyn Signature Line (LXSL)
   Command: lxsl
   Description: Adds Lord Xyn's signature to all appropriate files in the current directory and its subdirectories.
   Usage: xynpro lxsl
   Example: 
     $ cd /path/to/your/project
     $ xynpro lxsl
   
   This command will:
   - Scan all files in the current directory and subdirectories
   - Skip files that shouldn't have signatures (e.g., binaries, images)
   - Clean existing signatures from files
   - Add new signatures to appropriate files

2. Xyn Git Update (XynGit)
   Command: xyngit
   Description: Automates the process of staging, committing, and pushing changes to the git repository.
   Usage: xynpro xyngit
   Example:
     $ cd /path/to/your/git/repo
     $ xynpro xyngit

   This command will:
   - Stage all changes in the current repository
   - Create a commit with the message "Automated update via xyngit"
   - Push the changes to the remote repository (default branch: main)

3. XynScript Management
   Description: Tools for managing your custom scripts (including pyscan and others).

   a. Backup Current Script
      Command: cat ~/.bin/<script_name> > ~/Projects/$(basename "$PWD")/xdocs/ow<script_name>
      Example: cat ~/.bin/pyscan > ~/Projects/$(basename "$PWD")/xdocs/owpyscan
      Description: Creates a tantalizing backup of your script in the xdocs/ directory of your current project.

   b. Reset Script
      Commands:
        rm ~/.bin/<script_name>
        nano ~/.bin/<script_name>
      Example:
        rm ~/.bin/pyscan
        nano ~/.bin/pyscan
      Description: Removes the current script, letting you start fresh with a clean slate. Opens nano for you to sculpt your new script to perfection.

   c. Make Script Executable
      Command: chmod +x ~/.bin/<script_name>
      Example: chmod +x ~/.bin/pyscan
      Description: Grants your script the power to execute, making it ready for action at your command.

4. PyScan Management
   Description: Tools for managing the pyscan utility.

   a. Backup Current PyScan
      Command: cat ~/.bin/pyscan > ~/Projects/$(basename "$PWD")/xdocs/owPyScan.txt
      Description: Overwrites the current pyscan in xdocs/ as owPyScan.txt, creating a tantalizing backup.

   b. Reset PyScan
      Commands:
        rm ~/.bin/pyscan
        nano ~/.bin/pyscan
      Description: Removes the current pyscan, letting you start fresh with a clean slate. Opens nano for you to sculpt your new pyscan to perfection.

   c. Make PyScan Executable
      Command: chmod +x ~/.bin/pyscan
      Description: Grants pyscan the power to execute, making it ready for action at your command.

5. Source Code Concatenation
   Command: 
   
find src/ tests/ -type f -name '*.rs' -print0 | xargs -0 awk 'FNR==1 && NR!=1 {print "\n\n"} {print}' > Xdocs/XynProCurrentState.txt
   
   Description: Combines all source files in the src/ and tests/ directories and their subdirectories into a single file.
   Usage: Run this command from the root directory of your project.
   Example:
     $ cd /path/to/your/project
     $ find src/ tests/ -type f -print0 | xargs -0 cat > Xdocs/XynProCurrentState.txt

   This command will:
   - Find all files in the src/ and tests/ directories and their subdirectories
   - Concatenate the contents of all these files
   - Output the result to Xdocs/XynProCurrentState.txt in your current working directory

Note: LXSL, XynGit, and Source Code Concatenation commands should be run from the root directory of your project or git repository. Script management commands can be run from any directory, but be mindful of your current location when backing up.

Remember, with great power comes great responsibility. Use these commands wisely, and may your code always bear the mark of Lord Xyn!

Pro Tip: Regular use of pyscan may lead to increased productivity and an irresistible urge to optimize everything. Use with caution - you might find yourself addicted to its efficiency!
"#.to_string()
}

