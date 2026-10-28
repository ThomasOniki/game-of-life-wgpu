@binding(0) @group(0) var<storage, read> ssbo_0 : array<u32>;

@id(1) override cols_0 : u32 = u32(0);

@id(0) override rows_0 : u32 = u32(0);

struct pixelOutput_0
{
    @location(0) output_0 : vec4<f32>,
};

@fragment
fn main(@builtin(position) pos_0 : vec4<f32>) -> pixelOutput_0
{
    var image_end_0 : u32 = (cols_0 / u32(32) + u32(2)) * (rows_0 + u32(2));
    var _S1 : vec2<u32> = vec2<u32>((pos_0.xy - vec2<f32>(f32(ssbo_0[image_end_0 + u32(2)]), f32(ssbo_0[image_end_0 + u32(3)]))) / vec2<f32>(vec2<u32>(ssbo_0[image_end_0], ssbo_0[image_end_0 + u32(1)])) * vec2<f32>(vec2<u32>(cols_0, rows_0)));
    var index_0 : u32 = _S1.x + u32(32) + (_S1.y + u32(1)) * (cols_0 + u32(64));
    var _S2 : pixelOutput_0 = pixelOutput_0( vec4<f32>(vec3<f32>(vec3<u32>((u32(1) - ((((ssbo_0[index_0 / u32(32)] >> ((index_0 % u32(32))))) & (u32(1))))))), 1.0f) );
    return _S2;
}

