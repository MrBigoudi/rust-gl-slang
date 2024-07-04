#version 460
layout(row_major) uniform;
layout(row_major) buffer;

#line 1 0
layout(location = 0)
out vec4 entryPointParam_main_0;


#line 1
layout(location = 0)
in vec3 input_color_0;




void main()
{

#line 7
    entryPointParam_main_0 = vec4(input_color_0, 1.0);

#line 7
    return;
}

