@binding(0) @group(0) var<storage, read> in_ssbo_0 : array<u32>;

@binding(1) @group(0) var<storage, read_write> out_ssbo_0 : array<u32>;

@id(0) override u32_cols_0 : u32 = u32(0);

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id_0 : vec3<u32>)
{
    var id_1 : vec2<u32> = id_0.xy;
    var flat_id_0 : u32 = id_1.x + u32(1) + (id_1.y + u32(1)) * u32_cols_0;
    var _S1 : u32 = flat_id_0 / u32_cols_0;
    var _S2 : u32 = flat_id_0 % u32_cols_0;
    var _S3 : u32 = flat_id_0 + u32_cols_0;
    var _S4 : u32 = flat_id_0 - u32_cols_0;
    var top_0 : u32 = in_ssbo_0[_S3];
    var middle_0 : u32 = in_ssbo_0[flat_id_0];
    var bottom_0 : u32 = in_ssbo_0[_S4];
    var _S5 : u32 = ((((in_ssbo_0[_S3 - u32(1)] >> (u32(31)))) & (u32(1)))) + ((((in_ssbo_0[flat_id_0 - u32(1)] >> (u32(31)))) & (u32(1)))) + ((((in_ssbo_0[_S4 - u32(1)] >> (u32(31)))) & (u32(1))));
    var old_0 : u32 = ((top_0 & (u32(1)))) + ((middle_0 & (u32(1)))) + ((bottom_0 & (u32(1))));
    var older_0 : u32 = _S5;
    var i_0 : u32 = u32(0);
    var middle_1 : u32 = middle_0;
    var top_1 : u32 = top_0;
    var bottom_1 : u32 = bottom_0;
    var new_cells_0 : u32 = u32(0);
    for(;;)
    {
        if(i_0 < u32(31))
        {
        }
        else
        {
            break;
        }
        var middle_bit_0 : u32 = (middle_1 & (u32(1)));
        var top_2 : u32 = (top_1 >> (u32(1)));
        var middle_2 : u32 = (middle_1 >> (u32(1)));
        var bottom_2 : u32 = (bottom_1 >> (u32(1)));
        var sum_0 : u32 = ((top_2 & (u32(1)))) + ((middle_2 & (u32(1)))) + ((bottom_2 & (u32(1))));
        var neighbors_0 : u32 = sum_0 + old_0 + older_0 - middle_bit_0;
        if(neighbors_0 == u32(3))
        {
            new_cells_0 = (new_cells_0 | (u32((i32(1) << (i_0)))));
        }
        else
        {
            var new_cells_1 : u32;
            if(neighbors_0 == u32(2))
            {
                new_cells_1 = (new_cells_0 | (((middle_bit_0 << (i_0)))));
            }
            else
            {
                new_cells_1 = new_cells_0;
            }
            new_cells_0 = new_cells_1;
        }
        var i_1 : u32 = i_0 + u32(1);
        older_0 = old_0;
        old_0 = sum_0;
        i_0 = i_1;
        middle_1 = middle_2;
        top_1 = top_2;
        bottom_1 = bottom_2;
    }
    var middle_bit_1 : u32 = (middle_1 & (u32(1)));
    var neighbors_1 : u32 = ((((in_ssbo_0[_S3 + u32(1)] & (u32(1)))) >> (u32(0)))) + ((((in_ssbo_0[flat_id_0 + u32(1)] & (u32(1)))) >> (u32(0)))) + ((((in_ssbo_0[_S4 + u32(1)] & (u32(1)))) >> (u32(0)))) + old_0 + older_0 - middle_bit_1;
    if(neighbors_1 == u32(3))
    {
        new_cells_0 = (new_cells_0 | (u32(2147483648)));
    }
    else
    {
        if(neighbors_1 == u32(2))
        {
            new_cells_0 = (new_cells_0 | (((middle_bit_1 << (u32(31))))));
        }
    }
    out_ssbo_0[flat_id_0] = new_cells_0;
    return;
}

