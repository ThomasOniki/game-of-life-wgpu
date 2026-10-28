@binding(0) @group(0) var<storage, read> in_ssbo_0 : array<u32>;

@binding(1) @group(0) var<storage, read_write> out_ssbo_0 : array<u32>;

@id(0) override cols_0 : u32 = u32(0);

fn neighbor_0( index_0 : u32,  x_off_0 : i32,  y_off_0 : i32) -> u32
{
    var _S1 : u32 = index_0 + u32(x_off_0) + u32(y_off_0) * cols_0;
    return (((in_ssbo_0[_S1 / u32(32)] >> ((_S1 % u32(32))))) & (u32(1)));
}

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id_0 : vec3<u32>)
{
    var id_1 : vec2<u32> = id_0.xy;
    var flat_id_0 : u32 = id_1.x + u32(1) + (id_1.y + u32(1)) * cols_0 / u32(32);
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
        var index_1 : u32 = flat_id_0 * u32(32) + i_0;
        var value_0 : u32 = neighbor_0(index_1, i32(0), i32(0));
        var neighbors_0 : u32 = neighbor_0(index_1, i32(-1), i32(-1)) + neighbor_0(index_1, i32(-1), i32(0)) + neighbor_0(index_1, i32(-1), i32(1)) + neighbor_0(index_1, i32(0), i32(-1)) + neighbor_0(index_1, i32(0), i32(1)) + neighbor_0(index_1, i32(1), i32(-1)) + neighbor_0(index_1, i32(1), i32(0)) + neighbor_0(index_1, i32(1), i32(1));
        var _S2 : bool;
        if(value_0 == u32(1))
        {
            if(neighbors_0 < u32(2))
            {
                _S2 = true;
            }
            else
            {
                _S2 = neighbors_0 > u32(3);
            }
        }
        else
        {
            _S2 = false;
        }
        var value_1 : u32;
        if(_S2)
        {
            value_1 = u32(0);
        }
        else
        {
            value_1 = value_0;
        }
        var _S3 : bool;
        if(value_1 == u32(0))
        {
            _S3 = neighbors_0 == u32(3);
        }
        else
        {
            _S3 = false;
        }
        var value_2 : u32;
        if(_S3)
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
    out_ssbo_0[flat_id_0] = new_cells_0;
    return;
}

