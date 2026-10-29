@binding(0) @group(0) var<storage, read> in_ssbo_0 : array<u32>;

@binding(1) @group(0) var<storage, read_write> out_ssbo_0 : array<u32>;

@id(0) override cols_0 : u32 = u32(0);

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) id_0 : vec3<u32>)
{
    var id_1 : vec2<u32> = id_0.xy;
    var _S1 : u32 = id_1.x;
    var _S2 : u32 = id_1.y;
    var _S3 : u32 = _S1 + _S2 * cols_0 / u32(32);
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
        var j_0 : u32 = u32(0);
        for(;;)
        {
            if(j_0 < u32(8))
            {
            }
            else
            {
                break;
            }
            var new_cells_1 : u32 = (new_cells_0 | (((((((in_ssbo_0[_S3] >> ((i_0 + j_0)))) & (u32(1)))) << ((i_0 + (u32(7) - j_0)))))));
            j_0 = j_0 + u32(1);
            new_cells_0 = new_cells_1;
        }
        i_0 = i_0 + u32(8);
    }
    out_ssbo_0[_S1 + u32(1) + (_S2 + u32(1)) * (cols_0 / u32(32) + u32(2))] = new_cells_0;
    return;
}

