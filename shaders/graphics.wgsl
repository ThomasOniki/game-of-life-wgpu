@binding(0) @group(0) var<storage, read> ssbo_0 : array<u32>;

struct vertexOutput_0
{
    @builtin(position) output_0 : vec4<f32>,
};

@vertex
fn vertex_shader(@builtin(vertex_index) vertex_index_0 : u32) -> vertexOutput_0
{
    var positions_0 : array<vec2<f32>, i32(3)> = array<vec2<f32>, i32(3)>( vec2<f32>(-1.0f, -1.0f), vec2<f32>(-1.0f, 3.0f), vec2<f32>(3.0f, -1.0f) );
    var _S1 : vertexOutput_0 = vertexOutput_0( vec4<f32>(positions_0[vertex_index_0], 0.0f, 1.0f) );
    return _S1;
}

struct pixelOutput_0
{
    @location(0) output_1 : vec4<f32>,
};

@fragment
fn fragment_shader(@builtin(position) pos_0 : vec4<f32>) -> pixelOutput_0
{
    var height_0 : u32 = ssbo_0[i32(65)];
    var _S2 : f32 = f32(ssbo_0[i32(64)]);
    var _S3 : bool;
    if((pos_0.x) > _S2)
    {
        _S3 = true;
    }
    else
    {
        _S3 = (pos_0.y) > f32(height_0);
    }
    if(_S3)
    {
        var _S4 : pixelOutput_0 = pixelOutput_0( vec4<f32>(0.0f, 0.0f, 0.0f, 1.0f) );
        return _S4;
    }
    var _S5 : vec2<u32> = vec2<u32>(pos_0.xy / vec2<f32>(_S2, f32(height_0)) * vec2<f32>(vec2<u32>(u32(64), u32(32))));
    var index_0 : u32 = _S5.x + _S5.y * u32(64);
    var _S6 : pixelOutput_0 = pixelOutput_0( vec4<f32>(vec3<f32>(vec3<u32>((u32(1) - ((((ssbo_0[index_0 / u32(32)] >> ((index_0 % u32(32))))) & (u32(1))))))), 1.0f) );
    return _S6;
}

