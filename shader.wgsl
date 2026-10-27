const vertices_0 : array<vec2<f32>, i32(3)> = array<vec2<f32>, i32(3)>( vec2<f32>(-0.5f, -0.5f), vec2<f32>(0.5f, -0.5f), vec2<f32>(0.0f, 0.5f) );
struct vertexOutput_0
{
    @builtin(position) output_0 : vec4<f32>,
};

@vertex
fn vertex_shader(@builtin(vertex_index) vertex_index_0 : u32) -> vertexOutput_0
{
    var _S1 : vertexOutput_0 = vertexOutput_0( vec4<f32>(vertices_0[vertex_index_0], 0.0f, 1.0f) );
    return _S1;
}

struct pixelOutput_0
{
    @location(0) output_1 : vec4<f32>,
};

@fragment
fn fragment_shader() -> pixelOutput_0
{
    var _S2 : pixelOutput_0 = pixelOutput_0( vec4<f32>(1.0f, 0.0f, 0.0f, 1.0f) );
    return _S2;
}

