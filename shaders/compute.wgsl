@binding(0) @group(0) var<storage, read> in_ssbo_0 : array<u32>;

@binding(1) @group(0) var<storage, read_write> out_ssbo_0 : array<u32>;

@id(1) override cols_0 : u32 = u32(0);

@id(0) override rows_0 : u32 = u32(0);

fn neighbor_0( index_0 : u32,  x_off_0 : i32,  y_off_0 : i32) -> u32
{
    var _S1 : bool;
    if(x_off_0 < i32(0))
    {
        var _S2 : u32 = index_0 % cols_0;
        _S1 = _S2 == u32(0);
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    if(y_off_0 < i32(0))
    {
        var _S3 : u32 = index_0 / cols_0;
        _S1 = _S3 == u32(0);
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    if(x_off_0 > i32(0))
    {
        var _S4 : u32 = index_0 % cols_0;
        _S1 = _S4 == (cols_0 - u32(1));
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    if(y_off_0 > i32(0))
    {
        var _S5 : u32 = index_0 / cols_0;
        _S1 = _S5 == (rows_0 - u32(1));
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    var _S6 : u32 = index_0 + u32(x_off_0) + u32(y_off_0) * cols_0;
    return (((in_ssbo_0[_S6 / u32(32)] >> ((_S6 % u32(32))))) & (u32(1)));
}

@compute
@workgroup_size(1, 1, 1)
fn compute_shader(@builtin(global_invocation_id) id_0 : vec3<u32>)
{
    var id_1 : u32 = id_0.x;
    var i_0 : u32 = u32(0);
    var new_cells_0 : u32 = u32(0);
    for(;;)
    {
        if(i_0 < u32(32))
        {
        }
        else
        {
            break;
        }
        var index_1 : u32 = id_1 * u32(32) + i_0;
        var value_0 : u32 = neighbor_0(index_1, i32(0), i32(0));
        var _S7 : u32 = neighbor_0(index_1, i32(-1), i32(-1));
        var _S8 : u32 = neighbor_0(index_1, i32(-1), i32(0));
        var _S9 : u32 = _S7 + _S8;
        var _S10 : u32 = neighbor_0(index_1, i32(-1), i32(1));
        var _S11 : u32 = _S9 + _S10;
        var _S12 : u32 = neighbor_0(index_1, i32(0), i32(-1));
        var _S13 : u32 = _S11 + _S12;
        var _S14 : u32 = neighbor_0(index_1, i32(0), i32(1));
        var _S15 : u32 = _S13 + _S14;
        var _S16 : u32 = neighbor_0(index_1, i32(1), i32(-1));
        var _S17 : u32 = _S15 + _S16;
        var _S18 : u32 = neighbor_0(index_1, i32(1), i32(0));
        var _S19 : u32 = _S17 + _S18;
        var _S20 : u32 = neighbor_0(index_1, i32(1), i32(1));
        var neighbors_0 : u32 = _S19 + _S20;
        var _S21 : bool;
        if(value_0 == u32(1))
        {
            if(neighbors_0 < u32(2))
            {
                _S21 = true;
            }
            else
            {
                _S21 = neighbors_0 > u32(3);
            }
        }
        else
        {
            _S21 = false;
        }
        var value_1 : u32;
        if(_S21)
        {
            value_1 = u32(0);
        }
        else
        {
            value_1 = value_0;
        }
        var _S22 : bool;
        if(value_1 == u32(0))
        {
            _S22 = neighbors_0 == u32(3);
        }
        else
        {
            _S22 = false;
        }
        var value_2 : u32;
        if(_S22)
        {
            value_2 = u32(1);
        }
        else
        {
            value_2 = value_1;
        }
        var new_cells_1 : u32 = (new_cells_0 | (((value_2 << (i_0)))));
        i_0 = i_0 + u32(1);
        new_cells_0 = new_cells_1;
    }
    out_ssbo_0[id_1] = new_cells_0;
    return;
}

