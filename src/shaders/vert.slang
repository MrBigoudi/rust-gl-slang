struct vs_input {
    float3 position : POSITION;
    float3 color : COLOR;
}

struct vs_output {
    float4 position : SV_Position;
    float3 color : COLOR;

    __init(float4 pos, float3 col){
        position = pos;
        color = col;
    }

    __init(vs_input input){
        position = float4(input.position, 1.f);
        color = input.color;
    }
}

[shader("vertex")]
vs_output main(vs_input input){
    return vs_output(input);
}