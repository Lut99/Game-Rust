//  BUILD.rs
//    by Lut99
// 
//  Created:
//    30 Apr 2022, 17:52:26
//  Last edited:
//    20 Aug 2022, 14:30:07
//  Auto updated?
//    Yes
// 
//  Description:
//!   Build script for the game-gfx crate.
// 

use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;
use std::process::Command;


/***** HELPER FUNCTIONS *****/
/// Checks if glslc is available in the PATH.
/// 
/// Will panic if it isn't.
fn check_glslc() {
    // Check glslc is in the path by running a test command
    let mut cmd = Command::new("glslc");
    cmd.arg("--version");
    let output = match cmd.output() {
        Ok(output) => output,
        Err(err)   => { panic!("Could not run command '{:?}' to test for glslc presence: {}", cmd, err); }
    };
    if !output.status.success() { panic!("glslc not found in path; cannot compile shaders\n\nStdout:\n{}\n\nStderr:\n{}\n\n", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr)); }
}

/// Expands a list of arguments into command arguments.
macro_rules! expand_args {
    ($cmd:ident, $arg:expr) => {
        $cmd.arg($arg);
    };

    ($cmd:ident, $arg:expr, $($args:expr),+) => {
        expand_args!($cmd, $arg);
        expand_args!($cmd, $($args),+);
    };
}

/// Runs glslc with the given commands.
/// 
/// Will panic if it fails.
macro_rules! glslc {
    ($($args:expr),+) => {
        // Check glslc is in the path by running a test command
        let mut cmd = Command::new("glslc");
        expand_args!(cmd, $($args),+);
        let output = match cmd.output() {
            Ok(output) => output,
            Err(err)   => { panic!("Could not run command '{:?}' to compile shader: {}", cmd, err); }
        };
        if !output.status.success() {
            panic!("glslc returned non-zero exit status.\n\nStdout:\n{}\n\nStderr:\n{}\n\n", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        }
    };
}





/// Entrypoint to the build script
fn main() {
    // Check glslc is in the path
    check_glslc();

    // Otherwise, build the triangle shaders
    println!("Compiling Triangle pipeline shaders...");
    let src_path: PathBuf = PathBuf::from("./src");
    let mut todo: Vec<(PathBuf, ReadDir)> = vec![ (src_path.clone(), fs::read_dir(&src_path).unwrap_or_else(|err| panic!("Could not read src folder '{}': {}", src_path.display(), err))) ];
    while !todo.is_empty() {
        // Pop the todo one
        let (path, list): (PathBuf, ReadDir) = todo.pop().unwrap();

        // Iterate through the entries
        for (i, entry) in list.enumerate() {
            // Unwrap it
            let entry: DirEntry = entry.unwrap_or_else(|err| panic!("Failed to unwrap entry {} in '{}': {}", i, path.display(), err));
            let entry_path: PathBuf = entry.path();

            // Match on file or no
            if entry_path.is_file() {
                // Check the extension
                if let Some(file_stem) = entry_path.file_stem() {
                    let file_stem: String = file_stem.to_string_lossy().to_string();
                    if let Some(extension) = entry_path.extension() {
                        let extension: String = extension.to_string_lossy().to_string();

                        // It has be called 'shader' and not end in '.spv' or '.rs'
                        if file_stem == String::from("shader") && &extension[extension.len() - 3..] != "spv" && &extension[extension.len() - 2..] != "rs" {
                            // Create the SPIR-V directory if it does not exist yet
                            let spirv_dir: PathBuf = entry_path.parent().unwrap().join("spir-v");
                            if !spirv_dir.exists() { fs::create_dir(&spirv_dir).unwrap_or_else(|err| panic!("Failed to create SPIR-V output directory '{}': {}", spirv_dir.display(), err)); }

                            // Compile the thing
                            let out: PathBuf = spirv_dir.join(&format!("{}.spv", entry_path.file_name().unwrap().to_string_lossy().to_string()));
                            println!("Compiling '{}' to '{}'...", entry_path.display(), out.display());
                            glslc!("-o", out, entry_path);
                        }
                    }
                }
            } else if entry_path.is_dir() {
                // Recurse
                todo.push((entry_path.clone(), fs::read_dir(&entry_path).unwrap_or_else(|err| panic!("Could not read directory '{}': {}", entry_path.display(), err))))
            }
        }
    }

    // glslc!("-o", "./src/triangle/shaders/spir-v/vertex.spv", "./src/triangle/shaders/shader.vert");
    // glslc!("-o", "./src/triangle/shaders/spir-v/fragment.spv", "./src/triangle/shaders/shader.frag");
    // glslc!("-o", "./src/square/shaders/spir-v/vertex.spv", "./src/square/shaders/shader.vert");
    // glslc!("-o", "./src/square/shaders/spir-v/fragment.spv", "./src/square/shaders/shader.frag");
}
