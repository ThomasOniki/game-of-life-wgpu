@binding(0) @group(0) var<storage, read> in_ssbo_0 : array<u32>;

@binding(1) @group(0) var<storage, read_write> out_ssbo_0 : array<u32>;

fn neighbor_0( index_0 : u32,  x_off_0 : u32,  y_off_0 : u32) -> u32
{
    var _S1 : bool;
    if(x_off_0 < u32(0))
    {
        _S1 = (index_0 % u32(64)) == u32(0);
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    if(y_off_0 < u32(0))
    {
        _S1 = (index_0 / u32(64)) == u32(0);
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    if(x_off_0 > u32(0))
    {
        _S1 = (index_0 % u32(64)) == u32(31);
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    if(y_off_0 > u32(0))
    {
        _S1 = (index_0 / u32(64)) == u32(63);
    }
    else
    {
        _S1 = false;
    }
    if(_S1)
    {
        return u32(0);
    }
    var _S2 : u32 = index_0 + x_off_0 + y_off_0 * u32(64);
    return (((in_ssbo_0[_S2 / u32(32)] >> ((_S2 % u32(32))))) & (u32(1)));
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
        var value_0 : u32 = neighbor_0(index_1, u32(0), u32(0));
        var neighbors_0 : u32 = neighbor_0(index_1, u32(4294967295), u32(4294967295)) + neighbor_0(index_1, u32(4294967295), u32(0)) + neighbor_0(index_1, u32(4294967295), u32(1)) + neighbor_0(index_1, u32(0), u32(4294967295)) + neighbor_0(index_1, u32(0), u32(1)) + neighbor_0(index_1, u32(1), u32(4294967295)) + neighbor_0(index_1, u32(1), u32(0)) + neighbor_0(index_1, u32(1), u32(1));
        var _S3 : bool;
        if(value_0 == u32(1))
        {
            if(neighbors_0 < u32(2))
            {
                _S3 = true;
            }
            else
            {
                _S3 = neighbors_0 > u32(3);
            }
        }
        else
        {
            _S3 = false;
        }
        var value_1 : u32;
        if(_S3)
        {
            value_1 = u32(0);
        }
        else
        {
            value_1 = value_0;
        }
        var _S4 : bool;
        if(value_1 == u32(0))
        {
            _S4 = neighbors_0 == u32(3);
        }
        else
        {
            _S4 = false;
        }
        var value_2 : u32;
        if(_S4)
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

