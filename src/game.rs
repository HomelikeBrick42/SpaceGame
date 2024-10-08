use crate::{
    motor::Transform, vector3::Vector3, Camera, GpuCamera, GpuMesh, GpuMeshes, GpuVertices, Mesh,
    Number, Vertex,
};
use encase::{ShaderSize, ShaderType, StorageBuffer, UniformBuffer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use winit::window::Window;

pub struct Game {
    window: Arc<Window>,
    surface_config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    meshes: HashMap<String, Mesh>,
    vertices: Vec<Vertex>,
    mesh_buffer: wgpu::Buffer,
    vertices_buffer: wgpu::Buffer,
    mesh_bind_group_layout: wgpu::BindGroupLayout,
    mesh_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct MeshConfig {
    name: String,
    model: String,
    title: String,
    texture_color_map: String,
    discription: String,
}

impl Game {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8Unorm,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: GpuCamera::SHADER_SIZE.get(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(GpuCamera::SHADER_SIZE),
                    },
                    count: None,
                }],
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let mesh_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mesh Buffer"),
            size: GpuMeshes::min_size().get(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let vertices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertices Buffer"),
            size: GpuVertices::min_size().get(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let mesh_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Mesh Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: Some(GpuMeshes::min_size()),
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: Some(GpuVertices::min_size()),
                        },
                        count: None,
                    },
                ],
            });
        let mesh_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Mesh Bind Group"),
            layout: &mesh_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: mesh_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: vertices_buffer.as_entire_binding(),
                },
            ],
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let shader = device.create_shader_module(wgpu::include_wgsl!("./shader.wgsl"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout,
                    &mesh_bind_group_layout,
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        let mut app = Self {
            window,
            surface_config,
            surface,
            device,
            queue,
            camera: Camera {
                transform: Transform::IDENTITY,
            },
            camera_buffer,
            camera_bind_group,
            meshes: HashMap::new(),
            vertices: Vec::new(),
            mesh_buffer,
            vertices_buffer,
            mesh_bind_group_layout,
            mesh_bind_group,
            texture_bind_group_layout,
            render_pipeline,
        };
        app.load_game(Path::new("GameData"));
        
        let fuel_tank = app.meshes.get_mut("mk1-fl-100").unwrap();
        fuel_tank.transform = fuel_tank.transform.apply(Transform::translation(Vector3::Y * Number::from_num(-2)));
        app
    }

    fn load_game(&mut self, path: &Path) {
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                self.load_game(&path)
            }
        } else if path
            .extension()
            .is_some_and(|path| path.to_str() == Some("json"))
        {
            let part: MeshConfig =
                serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            self.load_mesh(
                &part.name,
                &path.parent().unwrap().join(part.model),
                &path.parent().unwrap().join(part.texture_color_map),
                Transform::translation(Vector3::Z * Number::from_num(5) + Vector3::Y * Number::from_num(2)),
            );
        }
    }

    fn load_mesh(
        &mut self,
        name: &str,
        path: &Path,
        texture_color_map_path: &Path,
        transform: Transform,
    ) -> usize {
        let index = self.meshes.len();
        let start_vertex_index = self.vertices.len() as _;

        let object = obj::Obj::load(path).unwrap();

        assert_eq!(object.data.objects.len(), 1);
        assert_eq!(object.data.objects[0].groups.len(), 1);
        let vertices = object.data.objects[0].groups[0]
            .polys
            .iter()
            .flat_map(|poly| &poly.0)
            .map(|index| Vertex {
                position: object.data.position[index.0].into(),
                normal: object.data.normal[index.2.unwrap()].into(),
                texture_coords: object.data.texture[index.1.unwrap()].into(),
            })
            .collect::<Vec<_>>();

        let image =
            image::load_from_memory(&std::fs::read(texture_color_map_path).unwrap()).unwrap();
        let rgba_image = image.flipv().to_rgba8();

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Mesh Texture"),
            size: wgpu::Extent3d {
                width: rgba_image.width(),
                height: rgba_image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        self.queue.write_texture(
            texture.as_image_copy(),
            &rgba_image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * rgba_image.width()),
                rows_per_image: None,
            },
            texture.size(),
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let textures = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Mesh Texture Bind Group"),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
            ],
        });

        if self
            .meshes
            .insert(
                name.into(),
                Mesh {
                    start_vertex_index,
                    triangle_count: vertices.len() as _,
                    textures,
                    transform,
                },
            )
            .is_some()
        {
            panic!("{name} part defined multiple times");
        }

        self.vertices.extend(vertices);

        index
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(wgpu::SurfaceError::Outdated) => {
                let size = self.window.inner_size();
                self.resize(size.width, size.height);
                return;
            }
            Err(wgpu::SurfaceError::Timeout) => return,
            Err(e @ (wgpu::SurfaceError::Lost | wgpu::SurfaceError::OutOfMemory)) => panic!("{e}"),
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Upload Camera
        {
            let Camera { transform: _ } = self.camera;
            let size = output.texture.size();

            UniformBuffer::new(
                self.queue
                    .write_buffer_with(&self.camera_buffer, 0, GpuCamera::SHADER_SIZE)
                    .unwrap()
                    .as_mut(),
            )
            .write(&GpuCamera {
                aspect: size.width as f32 / size.height as f32,
            })
            .unwrap();
        }

        // Upload meshes and vertices
        {
            let mut recreate_bind_group = false;

            // Upload meshes
            {
                let inverse_camera = self.camera.transform.inverse();
                let meshes = GpuMeshes {
                    meshes: &self
                        .meshes
                        .values()
                        .map(
                            |&Mesh {
                                 start_vertex_index: _,
                                 triangle_count: _,
                                 textures: _,
                                 transform,
                             }| GpuMesh {
                                transform: transform.apply(inverse_camera).into(),
                            },
                        )
                        .collect::<Vec<_>>(),
                };

                let size = meshes.size();
                if size.get() > self.mesh_buffer.size() {
                    self.mesh_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Mesh Buffer"),
                        size: size.get(),
                        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                        mapped_at_creation: false,
                    });
                    recreate_bind_group = true;
                }
                StorageBuffer::new(
                    self.queue
                        .write_buffer_with(&self.mesh_buffer, 0, size)
                        .unwrap()
                        .as_mut(),
                )
                .write(&meshes)
                .unwrap();
            }

            // Upload vertices
            {
                let vertices = GpuVertices {
                    vertices: &self.vertices,
                };

                let size = vertices.size();
                if size.get() > self.vertices_buffer.size() {
                    self.vertices_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Vertices Buffer"),
                        size: size.get(),
                        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
                        mapped_at_creation: false,
                    });
                    recreate_bind_group = true;
                }
                StorageBuffer::new(
                    self.queue
                        .write_buffer_with(&self.vertices_buffer, 0, size)
                        .unwrap()
                        .as_mut(),
                )
                .write(&vertices)
                .unwrap();
            }

            if recreate_bind_group {
                self.mesh_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Mesh Bind Group"),
                    layout: &self.mesh_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.mesh_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: self.vertices_buffer.as_entire_binding(),
                        },
                    ],
                });
            }
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.mesh_bind_group, &[]);
            for (i, mesh) in self.meshes.values().enumerate() {
                render_pass.set_bind_group(2, &mesh.textures, &[]);
                render_pass.draw(
                    mesh.start_vertex_index..mesh.start_vertex_index + mesh.triangle_count,
                    i as u32..i as u32 + 1,
                );
            }
        }
        self.queue.submit(Some(encoder.finish()));

        self.window.pre_present_notify();
        output.present();
    }

    pub fn update(&mut self, _time: std::time::Duration, dt: std::time::Duration) {
        let ts = Number::from_num(dt.as_secs_f64());

        // let position = Vector3::new(
        //     Number::from_num((time.as_secs_f64() * 2.0).sin() * 4.0),
        //     Number::from_num((time.as_secs_f64() * 3.1).cos() * 4.0),
        //     Number::ZERO,
        // );
        // self.camera.transform = Motor::translation(position);

        let cockpit = self.meshes.get_mut("mk1-cockpit").unwrap();
        cockpit.transform = cockpit
            .transform
            .pre_apply(Transform::rotation_xy(ts * Number::from_num(0)))
            .pre_apply(Transform::rotation_xz(ts * Number::from_num(1)))
            .pre_apply(Transform::rotation_yz(ts * Number::from_num(0)))
            .normalized();

    }
}
