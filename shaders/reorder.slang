[vk_constant_id(0)]
const uint32_t cols = 0;

StructuredBuffer<uint32_t> in_ssbo;
RWStructuredBuffer<uint32_t> out_ssbo;

[shader("compute")]
void main(uint32_t2 id: SV_DispatchThreadID)
{
    const uint32_t flat_id = id.x + id.y * cols / 32;

    uint32_t new_cells = 0;
    for (uint32_t i = 0; i < 32; i += 8)
    {
        for (uint32_t j = 0; j < 8; ++j)
        {
            const uint32_t bit = (in_ssbo[flat_id] >> i + j) & 1;
            new_cells |= bit << i + (7 - j);
        }
    }

    const uint32_t padded_id = (id.x + 1) + (id.y + 1) * (cols / 32 + 2);

    out_ssbo[padded_id] = new_cells;
}
