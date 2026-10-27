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

#define ROWS 8
#define COLS 8

StructuredBuffer<uint32_t> ssbo;

[shader("fragment")]
float4 fragment_shader(float4 pos: SV_Position)
{
    uint32_t width = ssbo[(ROWS * COLS / 32)];
    uint32_t height = ssbo[(ROWS * COLS / 32) + 1];

    float2 norm_pos = pos.xy / float2(width, height);
    uint32_t2 index2d = uint32_t2(norm_pos * uint32_t2(COLS, ROWS));
    uint32_t index = index2d.x + index2d.y * COLS;

    uint32_t packed = ssbo[index / 32];
    uint32_t bit_index = index % 32;

    uint32_t col = (packed >> bit_index) & 1;

    return { float3(col), 1 };
}
