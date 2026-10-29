struct vertexOutput_0
{
    @builtin(position) output_0 : vec4<f32>,
};

@vertex
fn main(@builtin(vertex_index) vertex_index_0 : u32) -> vertexOutput_0
{
    var positions_0 : array<vec2<f32>, i32(3)> = array<vec2<f32>, i32(3)>( vec2<f32>(-1.0f, -1.0f), vec2<f32>(-1.0f, 3.0f), vec2<f32>(3.0f, -1.0f) );
    var _S1 : vertexOutput_0 = vertexOutput_0( vec4<f32>(positions_0[vertex_index_0], 0.0f, 1.0f) );
    return _S1;
}

