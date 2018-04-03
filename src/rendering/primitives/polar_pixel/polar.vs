/*
Standard Polar Vertex Shader
 */

#version 410 core
in vec2 radial;
in vec2 angle;
in vec4 color;

uniform float radial_shift;
uniform float rotation_angle;
uniform bool tunnel_mode;
uniform float length_circle;
uniform float length_total;

out vec2 radial_vertex;
out vec2 angle_vertex;
out vec4 color_vertex;

float render_radial(in float polar);

void main()
{
  if (tunnel_mode)
  {
    float radial_x = render_radial(radial.x);
    float radial_y = render_radial(radial.y);
    radial_vertex = vec2(max(radial_x, 0.0f), max(radial_y, 0.0f));
  }
  else {
    radial_vertex = vec2(max(radial.x - radial_shift, 0.0f), max(radial.y - radial_shift, 0.0f));
  }
  angle_vertex = vec2(angle.x - rotation_angle, angle.y - rotation_angle);
  color_vertex = color;

  gl_Position = vec4(0.0f, 0.0f, 0.0f, 1.0f);
}

float render_radial(in float radial_in){
  float length_tunnel = length_total - length_circle;
  if (radial_in < length_circle){
    return radial_in / (length_circle * (length_tunnel + 1));
  }
  else{
    float radial_tunnel = max(length_tunnel - (radial_in - length_circle), -0.9);
    return 1.0 - (radial_tunnel / (radial_tunnel + 1));
  }
}
