static const float2 vertices[] = {
    { -0.5, -0.5 },
    { 0.5, -0.5 },
    { 0, 0.5 },
};

[shader("vertex")]
float4 vertex_shader(uint32_t vertex_index: SV_VertexID)
{
    return { vertices[vertex_index], 0, 1 };
}

[shader("fragment")]
float4 fragment_shader()
{
    return { 1, 0, 0, 1 };
}
