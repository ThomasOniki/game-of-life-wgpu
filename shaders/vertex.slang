[shader("vertex")]
float4 main(uint32_t vertex_index: SV_VertexID)
{

    const float2 positions[] = {
        { -1, -1 },
        { -1, 3 },
        { 3, -1 },
    };

    return { positions[vertex_index], 0, 1 };
}

