/* SHADER.frag
 *   by Lut99
 *
 * Created:
 *   01 May 2022, 11:50:19
 * Last edited:
 *   01 May 2022, 11:53:37
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Simple fragment shader for the triangle pipeline, which takes the
 *   hardcoded vertices and returns a different colour per-vertex (which,
 *   interpolated, becomes a nice rainbow thingy).
**/

#version 450


/***** LAYOUT *****/
// The output colour of the fragment shader
layout(location = 0) out vec4 outColour;

// The colour from the vertex shader passed to us
layout(location = 0) in vec3 fragColour;





/***** ENTRYPOINT *****/
void main() {
    outColour = vec4(fragColour, 1.0);
}
