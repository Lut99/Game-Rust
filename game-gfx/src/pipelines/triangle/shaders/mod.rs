/* MOD.rs
 *   by Lut99
 *
 * Created:
 *   30 Apr 2022, 17:48:29
 * Last edited:
 *   30 Apr 2022, 18:00:29
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Imports the shader modules as raw bytes in the code.
**/

// Use external macros
#[macro_use] extern crate glossy;

/// The embedded vertex shader
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/pipelines/triangle/shaders/spir-v/vertex.spv"]
struct VertexShader;

/// The embedded fragment shader
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/src/pipelines/triangle/shaders/spir-v/fragment.spv"]
struct FragmentShader;
