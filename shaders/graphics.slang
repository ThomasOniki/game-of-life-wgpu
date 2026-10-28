[vk_constant_id(0)]
const uint32_t rows = 0;
[vk_constant_id(1)]
const uint32_t cols = 0;

[shader("vertex")]
float4 vertex_shader(uint32_t vertex_index: SV_VertexID)
{

    const float2 positions[] = {
        { -1, -1 },
        { -1, 3 },
        { 3, -1 },
    };

    return { positions[vertex_index], 0, 1 };
}

StructuredBuffer<uint32_t> ssbo;

[shader("fragment")]
float4 fragment_shader(float4 pos: SV_Position)
{
    const uint32_t2 size = { ssbo[(rows * cols / 32)], ssbo[(rows * cols / 32) + 1] };
    const float2 offset = { ssbo[(rows * cols / 32) + 2], ssbo[(rows * cols / 32) + 3] };

    const float2 norm_pos = (pos.xy - offset) / size;
    const uint32_t2 index2d = uint32_t2(norm_pos * uint32_t2(cols, rows));
    const uint32_t index = index2d.x + index2d.y * cols;

    const uint32_t col = (ssbo[index / 32] >> index) & 1;

    return { float3(1 - col), 1 };
}
