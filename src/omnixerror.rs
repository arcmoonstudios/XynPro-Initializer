// src/omnixerror.rs ~=#######D]======A===r===c====M===o===o===n=====<Lord[OMNIXERROR]Xyn>=====S===t===u===d===i===o===s======[R|$>

use thiserror::Error;

/// Represents errors that can occur during the project initialization process.
#[derive(Error, Debug)]
pub enum InitError {
    #[error("Failed to create the project directory: {0}")]
    ProjectDirCreationError(String),

    #[error("Invalid number of arguments provided: expected at least 2, got {0}")]
    ArgumentCountError(usize),

    #[error("Error while creating file '{file_name}': {source}")]
    FileCreationError { file_name: String, source: std::io::Error },

    #[error("Error while generating content for '{file_name}': {source}")]
    ContentGenerationError { file_name: String, source: Box<dyn std::error::Error> },

    #[error("Unknown error occurred: {0}")]
    UnknownError(String),
}

/// A utility function to handle errors in the initializer gracefully.
pub fn handle_init_error(err: InitError) {
    match err {
        InitError::ProjectDirCreationError(ref msg) => {
            eprintln!("Error: {}", msg);
        }
        InitError::ArgumentCountError(count) => {
            eprintln!("Error: Incorrect number of arguments provided. Expected at least 2, but got {}.", count);
        }
        InitError::FileCreationError { file_name, source } => {
            eprintln!("Error: Failed to create file '{}': {}", file_name, source);
        }
        InitError::ContentGenerationError { file_name, source } => {
            eprintln!("Error: Failed to generate content for file '{}': {}", file_name, source);
        }
        InitError::UnknownError(msg) => {
            eprintln!("Error: {}", msg);
        }
    }
}
