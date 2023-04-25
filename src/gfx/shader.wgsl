// Vertex shader
struct CameraUniform {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(6) model_matrix_0: vec4<f32>,
    @location(7) model_matrix_1: vec4<f32>,
    @location(8) model_matrix_2: vec4<f32>,
    @location(9) model_matrix_3: vec4<f32>,
    @location(10) normal_matrix_0: vec3<f32>,
    @location(11) normal_matrix_1: vec3<f32>,
    @location(12) normal_matrix_2: vec3<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_tangent: vec3<f32>,
    @location(4) world_bitangent: vec3<f32>,
    @location(5) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    let normal_matrix = mat3x3<f32>(
        instance.normal_matrix_0,
        instance.normal_matrix_1,
        instance.normal_matrix_2,
    );
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    var world_pos : vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0); 
    out.world_pos = world_pos.xyz;
    out.clip_position = camera.view_proj * world_pos;
    out.color = model.color;
    out.tex_coords = model.tex_coords;
    // TODO compute all this in tangent space to perform computation in tangent space.
    out.world_normal = normalize(normal_matrix * model.normal); // normal in object space
    out.world_tangent = normalize(normal_matrix * model.tangent); // normal in object space
    out.world_bitangent = normalize(normal_matrix * model.bitangent); // normal in object space

    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;
@group(0) @binding(2)
var t_normal: texture_2d<f32>;
@group(0) @binding(3)
var s_normal: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let albedo = textureSample(t_diffuse, s_diffuse, in.tex_coords) * in.color;
    let tangent_normal_map = textureSample(t_normal, s_normal, in.tex_coords).rgb * 2.0 - 1.0;
    
    // This generate normal matrix if no tangent / bitangent
    /*let p_dx = dpdx(in.world_pos);
	let p_dy = dpdy(in.world_pos);
	// derivations of the texture coordinate
	let t_dx = dpdx(in.tex_coords);
	let t_dy = dpdy(in.tex_coords);
	// tangent vector and binormal vector
    let world_matrix = mat3x3<f32>(
        normalize(t_dy.y * p_dx - t_dx.y * p_dy),
        normalize(t_dx.x * p_dy - t_dy.x * p_dx),
        normalize(in.world_normal)
    );*/

    // TODO use quat instead
    let world_matrix = mat3x3<f32>(
        normalize(in.world_tangent),
        normalize(in.world_bitangent),
        normalize(in.world_normal),
    );
	let world_normal = normalize(world_matrix * tangent_normal_map);

    let gi_color = vec3<f32>(0.1, 0.1, 0.1);
    let light_color = vec3<f32>(1.0, 1.0, 1.0);

    let light_pos = vec3<f32>(0.0, 10.0, 10.0);
    let light_dir = vec3<f32>(0.0, 0.0, 1.0);//normalize(light_pos - in.world_pos);
    let view_dir = normalize(camera.view_pos.xyz - in.world_pos);
    //let half_dir = normalize(view_dir + light_dir);
    
    let cos_theta = max(0.0, dot(world_normal, light_dir));
    let color = (cos_theta * light_color + gi_color) * albedo.xyz;

    return vec4<f32>(color, albedo.a);
}