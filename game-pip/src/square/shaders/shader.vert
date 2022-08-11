/* SHADER.vert
 *   by Lut99
 *
 * Created:
 *   01 May 2022, 11:47:41
 * Last edited:
 *   11 Aug 2022, 15:57:50
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The simple vertex shader for the triangle pipeline, which just passes
 *   three hardcoded vertices.
**/

#version 450


/***** LAYOUT *****/
// The input vertex position
layout(location = 0) in vec2 pos;
// The input vertex colour
layout(location = 1) in vec3 colour;

// The vertex colour to pass to the fragment shader (since it doesn't know vertex indices)
layout(location = 0) out vec3 frag_colour;





/***** ENTRYPOINT *****/
void main() {
    // Simply pass the given position, except scaled to 4D
    gl_Position = vec4(pos, 0.0, 1.0);
    // pass the given colour
    frag_colour = colour;
}
