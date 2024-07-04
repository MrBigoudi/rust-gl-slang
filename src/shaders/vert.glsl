#version 460
layout(row_major) uniform;
layout(row_major) buffer;

#line 6 0
struct vs_output_0
{
    vec4 position_0;
    vec3 color_0;
};


#line 1
struct vs_input_0
{
    vec3 position_1;
    vec3 color_1;
};


#line 15
vs_output_0 vs_output_x24init_0(vs_input_0 input_0)
{

#line 15
    vs_output_0 _S1;
    _S1.position_0 = vec4(input_0.position_1, 1.0);
    _S1.color_0 = input_0.color_1;

#line 15
    return _S1;
}


#line 15
layout(location = 0)
out vec3 entryPointParam_main_color_0;


#line 15
layout(location = 0)
in vec3 input_position_0;


#line 15
layout(location = 1)
in vec3 input_color_0;


#line 22
void main()
{

#line 22
    vs_input_0 _S2 = { input_position_0, input_color_0 };
    vs_output_0 _S3 = vs_output_x24init_0(_S2);

#line 23
    gl_Position = _S3.position_0;

#line 23
    entryPointParam_main_color_0 = _S3.color_0;

#line 23
    return;
}

