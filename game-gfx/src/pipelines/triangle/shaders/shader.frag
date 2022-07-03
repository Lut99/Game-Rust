/* SHADER.frag
 *   by Lut99
 *
 * Created:
 *   01 May 2022, 11:50:19
 * Last edited:
 *   03 Jul 2022, 14:47:20
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
// The colour from the vertex shader passed to us
layout(location = 0) in vec3 frag_colour;
// The output colour of the fragment shader
layout(location = 0) out vec4 out_colour;





/***** ENTRYPOINT *****/
void main() {
    // Simply pass, after some dimensional rescaling
    out_colour = vec4(frag_colour, 1.0);
}
