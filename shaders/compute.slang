#include "constants.h"

StructuredBuffer<uint32_t> in_ssbo;
RWStructuredBuffer<uint32_t> out_ssbo;

static uint32_t neighbor(uint32_t index, int32_t x_off, int32_t y_off)
{
    if (x_off < 0 && (index % COLS == 0))
    {
        return 0;
    }
    if (y_off < 0 && (index / COLS == 0))
    {
        return 0;
    }
    if (x_off > 0 && (index % COLS == (COLS - 1)))
    {
        return 0;
    }
    if (y_off > 0 && (index / COLS == (ROWS - 1)))
    {
        return 0;
    }

    index += x_off;
    index += y_off * COLS;

    return (in_ssbo[index / 32] >> (index % 32)) & 1;
}

[shader("compute")]
void compute_shader(uint32_t id: SV_DispatchThreadID)
{
    uint32_t new_cells = 0;
    for (uint32_t i = 0; i < 32; ++i)
    {
        uint32_t index = id * 32 + i;
        uint32_t value = neighbor(index, 0, 0);
        const uint32_t neighbors = neighbor(index, -1, -1) + neighbor(index, -1, 0) + neighbor(index, -1, 1) +
                                   neighbor(index, 0, -1) + neighbor(index, 0, 1) + neighbor(index, 1, -1) +
                                   neighbor(index, 1, 0) + neighbor(index, 1, 1);

        if (value == 1 && (neighbors < 2 || neighbors > 3))
        {
            value = 0;
        }
        if (value == 0 && neighbors == 3)
        {
            value = 1;
        }
        new_cells |= value << i;
    }

    out_ssbo[id] = new_cells;
}
