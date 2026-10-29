[vk_constant_id(0)]
const uint32_t rows = 0;
[vk_constant_id(1)]
const uint32_t u32_cols = 0;

StructuredBuffer<uint32_t> in_ssbo;
RWStructuredBuffer<uint32_t> out_ssbo;

[shader("compute")]
void main(uint32_t2 id: SV_DispatchThreadID)
{
    const uint32_t flat_id = id.x + id.y * u32_cols;

    const uint32_t row = flat_id / u32_cols;
    const bool bottommost = row == 0;
    const bool topmost = row == rows - 1;

    const uint32_t col = flat_id % u32_cols;

    const bool leftmost = col == 0;
    const uint32_t topleft = topmost || leftmost ? 0 : in_ssbo[flat_id + u32_cols - 1];
    const uint32_t left = leftmost ? 0 : in_ssbo[flat_id - 1];
    const uint32_t bottomleft = bottommost || leftmost ? 0 : in_ssbo[flat_id - u32_cols - 1];

    uint32_t top = topmost ? 0 : in_ssbo[flat_id + u32_cols];
    uint32_t middle = in_ssbo[flat_id];
    uint32_t bottom = bottommost ? 0 : in_ssbo[flat_id - u32_cols];

    uint32_t new_cells = 0;

    uint32_t older = ((topleft >> 31) & 1) + ((left >> 31) & 1) + ((bottomleft >> 31) & 1);
    uint32_t old = (top & 1) + (middle & 1) + (bottom & 1);

    for (uint32_t i = 0; i < 31; ++i)
    {
        const uint32_t middle_bit = middle & 1;

        top >>= 1;
        middle >>= 1;
        bottom >>= 1;

        const uint32_t sum = (top & 1) + (middle & 1) + (bottom & 1);

        const uint32_t neighbors = sum + old + older - middle_bit;
        older = old;
        old = sum;

        if (neighbors == 3)
        {
            new_cells |= 1 << i;
        }
        else if (neighbors == 2)
        {
            new_cells |= middle_bit << i;
        }
    }

    const uint32_t middle_bit = middle & 1;

    const bool rightmost = col == u32_cols - 1;
    const uint32_t topright = topmost || rightmost ? 0 : in_ssbo[flat_id + u32_cols + 1];
    const uint32_t right = rightmost ? 0 : in_ssbo[flat_id + 1];
    const uint32_t bottomright = bottommost || rightmost ? 0 : in_ssbo[flat_id - u32_cols + 1];

    const uint32_t sum = ((topright & 1) >> 0) + ((right & 1) >> 0) + ((bottomright & 1) >> 0);

    const uint32_t neighbors = sum + old + older - middle_bit;
    
    if (neighbors == 3)
    {
        new_cells |= 1 << 31;
    }
    else if (neighbors == 2)
    {
        new_cells |= middle_bit << 31;
    }

    out_ssbo[flat_id] = new_cells;
}
