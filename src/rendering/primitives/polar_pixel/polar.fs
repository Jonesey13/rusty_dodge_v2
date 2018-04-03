/*
Standard Polar Fragment Shader
 */

#version 410 core

in vec2 radial_geom;
in vec2 angle_geom;
in vec4 color_geom;
in vec2 emit_vertex;

uniform vec2 center;
uniform float aspect_ratio;

out vec4 color;

bool angleCompare(in float a, in vec2 range);

void main()
{
  vec2 fragCoord = emit_vertex;
  fragCoord.x = aspect_ratio * fragCoord.x;
  fragCoord -= center;
  float fragRadius = dot(fragCoord, fragCoord);
  vec2 radial_square = radial_geom * radial_geom;
  bool radialOverlap = fragRadius >= radial_square.x && fragRadius <= radial_square.y;

  bool angleOverlap = true;
  if(radialOverlap)
    {
      float angle = atan(fragCoord.y, fragCoord.x);
      angle = degrees(angle) / 360.0f;
      angleOverlap = angleCompare(angle, angle_geom);
    }
   if(angleOverlap && radialOverlap)
     color = color_geom;
   else
     color = vec4(0.0f, 0.0f, 0.0f, 0.0f);
}

bool angleCompare(in float ang, in vec2 range)
{
  ang -= floor(ang);

  if (range.y - range.x >= 0.9999)
  {
    return true;
  }
  
  range.y -= floor(range.x);
  range.x -= floor(range.y);

  bool isless = range.x <= range.y;
  if (isless)
    {
      return ang <=  range.y &&  ang >= range.x;
    }
  else
    {
      return ang >= range.x ||  ang <= range.y;
    }  
}
