/* SHADER.vert
 *   by Lut99
 *
 * Created:
 *   01 May 2022, 11:47:41
 * Last edited:
 *   01 May 2022, 11:52:21
 * Auto updated?
 *   Yes
 *
 * Description:
 *   The simple vertex shader for the triangle pipeline, which just passes
 *   three hardcoded vertices.
**/

#version 450


/***** CONSTANTS *****/
// The hardcoded positions (in 2D) of the test triangle
vec2 positions[3] = vec2[](
    vec2( 0.0, -0.5),
    vec2( 0.5,  0.5),
    vec2(-0.5,  0.5)
);

// The per-vertex colours
vec3 colours[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);





/***** LAYOUT *****/
// The vertex colour to pass to the fragment shader (since it doesn't know vertex indices)
layout(location = 0) out vec3 fragColour;





/***** ENTRYPOINT *****/
void main() {
    // Simply pass the appropriate vertex from the triangle (there will be three vertices, so index is 0-2)
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    // Pass the same colour index
    fragColour  = colours[gl_VertexIndex];
}
