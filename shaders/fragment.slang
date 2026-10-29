[vk_constant_id(0)]
const uint32_t rows = 0;
[vk_constant_id(1)]
const uint32_t cols = 0;

StructuredBuffer<uint32_t> ssbo;

[shader("fragment")]
float4 main(float4 pos: SV_Position)
{
    const uint32_t image_end = (cols / 32 + 2) * (rows + 2);
    const uint32_t2 size = { ssbo[image_end + 0], ssbo[image_end + 1] };
    const float2 offset = { ssbo[image_end + 2], ssbo[image_end + 3] };

    const float2 norm_pos = (pos.xy - offset) / size;
    const uint32_t2 index2d = uint32_t2(norm_pos * uint32_t2(cols, rows));
    const uint32_t index = (index2d.x + 32) + (index2d.y + 1) * (cols + 64);

    const uint32_t col = (ssbo[index / 32] >> index % 32) & 1;

    return { float3(1 - col), 1 };
}
