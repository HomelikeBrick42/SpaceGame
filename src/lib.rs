use az::CastFrom;
use encase::ShaderType;
use fixed::{types::extra::U16, FixedI128};
use motor::{GpuMotor, Motor};

pub mod game;
pub mod motor;
pub mod vector3;

pub type Number = FixedI128<U16>;

pub struct Camera {
    pub transform: Motor,
}

#[derive(ShaderType)]
pub struct GpuCamera {
    pub aspect: f32,
}

pub struct Mesh {
    pub color: cgmath::Vector3<f32>,
    pub start_vertex_index: u32,
    pub triangle_count: u32,
    pub transform: Motor,
}

#[derive(ShaderType)]
pub struct GpuMesh {
    pub color: cgmath::Vector3<f32>,
    pub transform: GpuMotor,
}

#[derive(ShaderType)]
pub struct GpuMeshes<'a> {
    #[size(runtime)]
    pub meshes: &'a [GpuMesh],
}

#[derive(ShaderType)]
pub struct Vertex {
    pub position: cgmath::Vector3<f32>,
    pub normal: cgmath::Vector3<f32>,
}

#[derive(ShaderType)]
pub struct GpuVertices<'a> {
    #[size(runtime)]
    pub vertices: &'a [Vertex],
}

fn sin_cos(mut x: Number) -> (Number, Number) {
    x %= Number::TAU;
    let (sin, cos) = f64::cast_from(x).sin_cos();
    (Number::from_num(sin), Number::from_num(cos))
}
