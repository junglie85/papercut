// Vertex

struct VertexOutput {
    @location(0) v_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) a_position: vec2<f32>,
    @location(1) a_normal: vec2<f32>,
    // @location(2) a_prim_id: u32,
    // @builtin(instance_index) instance_idx: u32
) -> VertexOutput {
    // var prim: Primitive = u_primitives.primitives[a_prim_id + instance_idx];

    // var invert_y = vec2<f32>(1.0, -1.0);

    // var rotation = mat2x2<f32>(
    //     vec2<f32>(cos(prim.angle), -sin(prim.angle)),
    //     vec2<f32>(sin(prim.angle), cos(prim.angle))
    // );

    // var local_pos = (a_position * prim.scale + a_normal * prim.width) * rotation;
    // var world_pos = local_pos - globals.scroll_offset + prim.translate;
    // var transformed_pos = world_pos * globals.zoom / (0.5 * globals.resolution) * invert_y;

    // var z = f32(prim.z_index) / 4096.0;
    var position = vec4<f32>(a_position, 0.0, 1.0);
    var color = vec4<f32>(1.0, 0.0, 0.0, 1.0);

    return VertexOutput(color, position);
}

// Fragment

struct Output {
    @location(0) out_color: vec4<f32>,
};

@fragment
fn fs_main(@location(0) v_color: vec4<f32>) -> Output {
    return Output(v_color);
}
