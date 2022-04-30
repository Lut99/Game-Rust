/* BUILD.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:52:26
 * Last edited:
 *   30 Apr 2022, 18:20:49
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Build script for the game-gfx crate.
**/

use std::io::ErrorKind;
use std::fs;
use std::path::Path;
use std::process::Command;


/***** HELPER FUNCTIONS *****/
/// Makes sure the target directory exists.
/// 
/// # Generic types
/// - `P`: The Path-like type of the path to create.
/// 
/// # Arguments
/// - `path`: The path of the directory to create.
/// 
/// # Errors
/// This function panics if it could not create the directory (except when it already exists).
fn create_dir<P: AsRef<Path>>(path: P) {
    // Create the triangle build stuff
    if let Err(err) = fs::create_dir(path) {
        if err.kind() != ErrorKind::AlreadyExists { panic!("Could not create triangle spir-v directory: {}", err); }
    }
}

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
    create_dir("./src/pipelines/triangle/shaders/spir-v");

    // Check glslc is in the path
    check_glslc();

    // Otherwise, build the triangle shaders
    glslc!("-o", "./src/pipelines/triangle/shaders/spir-v/vertex.spv", "./src/pipelines/triangle/shaders/shader.vert");
    glslc!("-o", "./src/pipelines/triangle/shaders/spir-v/fragment.spv", "./src/pipelines/triangle/shaders/shader.frag");
}
