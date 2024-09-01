struct Camera {
    aspect: f32,
}

@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Mesh {
    color: vec3<f32>,
    transform: Transform,
}

struct Meshes {
    meshes: array<Mesh>,
}

@group(1)
@binding(0)
var<storage, read> meshes: Meshes;

struct Vertex {
    position: vec3<f32>,
    normal: vec3<f32>,
}

struct Vertices {
    vertices: array<Vertex>,
}

@group(1)
@binding(1)
var<storage, read> vertices: Vertices;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) instance_index: u32,
    @location(1) position: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let mesh = meshes.meshes[instance_index];
    let vertex = vertices.vertices[vertex_index];
    let position = point_to_vec3(transform_point(vec3_to_point(vertex.position), mesh.transform));
    let normal = point_to_normal(transform_point(normal_to_point(vertex.normal), mesh.transform));

    out.clip_position = vec4<f32>(
        position.x / camera.aspect,
        position.y,
        1.0,
        position.z,
    );
    out.instance_index = instance_index;
    out.position = position;
    out.normal = normal;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let mesh = meshes.meshes[in.instance_index];
    let light = dot(in.normal, -normalize(in.position)) * 0.5 + 0.5;
    return vec4<f32>(mesh.color * light, 1.0);
}

struct Point {
    e012: f32,
    e013: f32,
    e023: f32,
    e123: f32,
}

fn vec3_to_point(v: vec3<f32>) -> Point {
    var result: Point;
    result.e012 = v.z;
    result.e013 = -v.y;
    result.e023 = v.x;
    result.e123 = 1.0;
    return result;
}

fn normal_to_point(v: vec3<f32>) -> Point {
    var result: Point;
    result.e012 = v.z;
    result.e013 = -v.y;
    result.e023 = v.x;
    result.e123 = 0.0;
    return result;
}

fn point_to_vec3(p: Point) -> vec3<f32> {
    return vec3<f32>(
        p.e023 / p.e123,
        -p.e013 / p.e123,
        p.e012 / p.e123,
    );
}

fn point_to_normal(p: Point) -> vec3<f32> {
    return normalize(vec3<f32>(
        p.e023,
        -p.e013,
        p.e012,
    ));
}

struct Transform {
    s: f32,
    e12: f32,
    e13: f32,
    e23: f32,
    e01: f32,
    e02: f32,
    e03: f32,
    e0123: f32,
}

fn transform_point(point: Point, transform: Transform) -> Point {
    let a = transform.s;
    let b = transform.e12;
    let c = transform.e13;
    let d = transform.e23;
    let e = transform.e01;
    let f = transform.e02;
    let g = transform.e03;
    let h = transform.e0123;
    let i = point.e012;
    let j = point.e013;
    let k = point.e023;
    let l = point.e123;

    var result: Point;
    result.e012 = -2.0 * a * d * j + -2.0 * a * g * l + 1.0 * a * a * i + 2.0 * a * c * k + -1.0 * d * d * i + -2.0 * d * f * l + 2.0 * b * d * k + -2.0 * b * h * l + -2.0 * c * e * l + 1.0 * b * b * i + 2.0 * b * c * j + -1.0 * c * c * i;
    result.e013 = -2.0 * a * b * k + -1.0 * b * b * j + 2.0 * b * c * i + 2.0 * b * e * l + 1.0 * a * a * j + 2.0 * a * d * i + 2.0 * a * f * l + -2.0 * c * h * l + -2.0 * d * g * l + -1.0 * d * d * j + 2.0 * c * d * k + 1.0 * c * c * j;
    result.e023 = -2.0 * a * c * i + -2.0 * a * e * l + 1.0 * a * a * k + 2.0 * a * b * j + -1.0 * c * c * k + 2.0 * c * d * j + 2.0 * c * g * l + -2.0 * d * h * l + 2.0 * b * f * l + -1.0 * b * b * k + 2.0 * b * d * i + 1.0 * d * d * k;
    result.e123 = a * a * l + b * b * l + c * c * l + d * d * l;
    return result;
}

fn inverse_transform(transform: Transform) -> Transform {
    var result = transform;
    result.e12 = -transform.e12;
    result.e13 = -transform.e13;
    result.e23 = -transform.e23;
    result.e01 = -transform.e01;
    result.e02 = -transform.e02;
    result.e03 = -transform.e03;
    return result;
}
