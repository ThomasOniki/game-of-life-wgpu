struct vertexOutput_0
{
    @builtin(position) output_0 : vec4<f32>,
};

@vertex
fn vertex_shader() -> vertexOutput_0
{
    var _S1 : vertexOutput_0 = vertexOutput_0( vec4<f32>(0.0f, 0.0f, 0.0f, 0.0f) );
    return _S1;
}

struct pixelOutput_0
{
    @location(0) output_1 : vec4<f32>,
};

@fragment
fn fragment_shader() -> pixelOutput_0
{
    var _S2 : pixelOutput_0 = pixelOutput_0( vec4<f32>(1.0f, 0.0f, 0.0f, 0.0f) );
    return _S2;
}

