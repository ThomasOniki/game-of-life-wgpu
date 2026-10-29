@binding(0) @group(0) var<storage, read> in_ssbo_0 : array<u32>;

@binding(1) @group(0) var<storage, read_write> out_ssbo_0 : array<u32>;

@id(1) override u32_cols_0 : u32 = u32(0);

@id(0) override rows_0 : u32 = u32(0);

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id_0 : vec3<u32>)
{
    var id_1 : vec2<u32> = id_0.xy;
    var flat_id_0 : u32 = id_1.x + id_1.y * u32_cols_0;
    var row_0 : u32 = flat_id_0 / u32_cols_0;
    var bottommost_0 : bool = row_0 == u32(0);
    var topmost_0 : bool = row_0 == (rows_0 - u32(1));
    var col_0 : u32 = flat_id_0 % u32_cols_0;
    var leftmost_0 : bool = col_0 == u32(0);
    var _S1 : bool;
    if(topmost_0)
    {
        _S1 = true;
    }
    else
    {
        _S1 = leftmost_0;
    }
    var topleft_0 : u32;
    if(_S1)
    {
        topleft_0 = u32(0);
    }
    else
    {
        topleft_0 = in_ssbo_0[flat_id_0 + u32_cols_0 - u32(1)];
    }
    var left_0 : u32;
    if(leftmost_0)
    {
        left_0 = u32(0);
    }
    else
    {
        left_0 = in_ssbo_0[flat_id_0 - u32(1)];
    }
    if(bottommost_0)
    {
        _S1 = true;
    }
    else
    {
        _S1 = leftmost_0;
    }
    var bottomleft_0 : u32;
    if(_S1)
    {
        bottomleft_0 = u32(0);
    }
    else
    {
        bottomleft_0 = in_ssbo_0[flat_id_0 - u32_cols_0 - u32(1)];
    }
    var top_0 : u32;
    if(topmost_0)
    {
        top_0 = u32(0);
    }
    else
    {
        top_0 = in_ssbo_0[flat_id_0 + u32_cols_0];
    }
    var middle_0 : u32 = in_ssbo_0[flat_id_0];
    var bottom_0 : u32;
    if(bottommost_0)
    {
        bottom_0 = u32(0);
    }
    else
    {
        bottom_0 = in_ssbo_0[flat_id_0 - u32_cols_0];
    }
    var new_cells_0 : u32;
    var _S2 : u32 = ((((topleft_0 >> (u32(31)))) & (u32(1)))) + ((((left_0 >> (u32(31)))) & (u32(1)))) + ((((bottomleft_0 >> (u32(31)))) & (u32(1))));
    var old_0 : u32 = ((top_0 & (u32(1)))) + ((middle_0 & (u32(1)))) + ((bottom_0 & (u32(1))));
    var older_0 : u32 = _S2;
    var i_0 : u32 = u32(0);
    var middle_1 : u32 = middle_0;
    var new_cells_1 : u32 = u32(0);
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
        var top_1 : u32 = (top_0 >> (u32(1)));
        var middle_2 : u32 = (middle_1 >> (u32(1)));
        var bottom_1 : u32 = (bottom_0 >> (u32(1)));
        var sum_0 : u32 = ((top_1 & (u32(1)))) + ((middle_2 & (u32(1)))) + ((bottom_1 & (u32(1))));
        var neighbors_0 : u32 = sum_0 + old_0 + older_0 - middle_bit_0;
        if(neighbors_0 == u32(3))
        {
            new_cells_1 = (new_cells_1 | (u32((i32(1) << (i_0)))));
        }
        else
        {
            if(neighbors_0 == u32(2))
            {
                new_cells_0 = (new_cells_1 | (((middle_bit_0 << (i_0)))));
            }
            else
            {
                new_cells_0 = new_cells_1;
            }
            new_cells_1 = new_cells_0;
        }
        var i_1 : u32 = i_0 + u32(1);
        older_0 = old_0;
        old_0 = sum_0;
        i_0 = i_1;
        middle_1 = middle_2;
        top_0 = top_1;
        bottom_0 = bottom_1;
    }
    var middle_bit_1 : u32 = (middle_1 & (u32(1)));
    var rightmost_0 : bool = col_0 == (u32_cols_0 - u32(1));
    if(topmost_0)
    {
        _S1 = true;
    }
    else
    {
        _S1 = rightmost_0;
    }
    if(_S1)
    {
        new_cells_0 = u32(0);
    }
    else
    {
        new_cells_0 = in_ssbo_0[flat_id_0 + u32_cols_0 + u32(1)];
    }
    var right_0 : u32;
    if(rightmost_0)
    {
        right_0 = u32(0);
    }
    else
    {
        right_0 = in_ssbo_0[flat_id_0 + u32(1)];
    }
    if(bottommost_0)
    {
        _S1 = true;
    }
    else
    {
        _S1 = rightmost_0;
    }
    var bottomright_0 : u32;
    if(_S1)
    {
        bottomright_0 = u32(0);
    }
    else
    {
        bottomright_0 = in_ssbo_0[flat_id_0 - u32_cols_0 + u32(1)];
    }
    var neighbors_1 : u32 = ((((new_cells_0 & (u32(1)))) >> (u32(0)))) + ((((right_0 & (u32(1)))) >> (u32(0)))) + ((((bottomright_0 & (u32(1)))) >> (u32(0)))) + old_0 + older_0 - middle_bit_1;
    if(neighbors_1 == u32(3))
    {
        new_cells_1 = (new_cells_1 | (u32(2147483648)));
    }
    else
    {
        if(neighbors_1 == u32(2))
        {
            new_cells_1 = (new_cells_1 | (((middle_bit_1 << (u32(31))))));
        }
    }
    out_ssbo_0[flat_id_0] = new_cells_1;
    return;
}

