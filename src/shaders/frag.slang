struct fs_input {
    float4 position : SV_Position;
    float3 color : COLOR;
}

[shader("fragment")]
float4 main(fs_input input) : SV_Target {
    return float4(input.color, 1.f);
}