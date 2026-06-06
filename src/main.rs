use std::f32::consts::{PI, TAU};
use three_d::*;
use three_d::context::HasContext;

#[cfg(target_arch = "wasm32")]
fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
    use wasm_bindgen::JsCast;
    web_sys::window()?
        .document()?
        .get_element_by_id("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .ok()
}

pub fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let window = Window::new(WindowSettings {
        title: "Orbital Elements".to_string(),
        #[cfg(target_arch = "wasm32")]
        canvas: get_canvas(),
        ..Default::default()
    })
    .expect("failed to create window");

    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 1.0, 10.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(75.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 5.0, 20.0);

    let axis_length: f32 = 4.0;
    let earth_radius: f32 = 2.0;
    let earth_tilt = Mat4::from_angle_x(degrees(23.5));

    // --- Earth wireframe (lat/lon lines) ---
    // GL_LINES geometry: each consecutive pair of vertices is one line segment.
    // No tube meshes, no end cap artifacts, no depth prepass needed.
    let mut earth_wireframe = Gm::new(
        GlobeLines::new(&context, earth_radius, 32, 32, 64, 64),
        ColorMaterial {
            color: Srgba::new(0, 255, 65, 255),
            render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                depth_test: DepthTest::Less,
                blend: Blend::Disabled,
                cull: Cull::None,
            },
            ..Default::default()
        },
    );

    // --- Axes: cylinder + cone for each, tilted with Earth ---
    // Original labels use orbital-mechanics convention:
    //   "Z" (blue) -> world +Y, "Y" (red) -> world +X, "X" (green) -> world +Z
    let blue = Srgba::new(0, 0, 255, 255);
    let red = Srgba::new(255, 0, 0, 255);
    let green = Srgba::new(0, 255, 0, 255);

    let mut axis_meshes: Vec<Gm<Mesh, ColorMaterial>> = Vec::new();
    let mut push_axis = |dir: Vec3, color: Srgba| {
        let rotation = rotation_x_to_dir(dir);
        let mut cyl = Gm::new(
            Mesh::new(&context, &CpuMesh::cylinder(8)),
            ColorMaterial {
                color,
                ..Default::default()
            },
        );
        cyl.set_transformation(
            earth_tilt * rotation * Mat4::from_nonuniform_scale(axis_length, 0.03, 0.03),
        );

        let mut cone = Gm::new(
            Mesh::new(&context, &CpuMesh::cone(8)),
            ColorMaterial {
                color,
                ..Default::default()
            },
        );
        cone.set_transformation(
            earth_tilt
                * rotation
                * Mat4::from_translation(vec3(axis_length - 0.15, 0.0, 0.0))
                * Mat4::from_nonuniform_scale(0.3, 0.1, 0.1),
        );

        axis_meshes.push(cyl);
        axis_meshes.push(cone);
    };
    push_axis(vec3(0.0, 1.0, 0.0), blue);
    push_axis(vec3(1.0, 0.0, 0.0), red);
    push_axis(vec3(0.0, 0.0, 1.0), green);
    drop(push_axis);

    // --- Pointer arrow (orange) at (3.8, 2.5, -1), tilted 160°/30° ---
    let orange = Srgba::new(255, 165, 0, 255);
    let pointer_group = Mat4::from_translation(vec3(3.8, 2.5, -1.0))
        * Mat4::from_angle_x(degrees(160.0))
        * Mat4::from_angle_z(degrees(30.0));
    let x_to_y = Mat4::from_angle_z(degrees(90.0));

    let mut pointer_line = Gm::new(
        Mesh::new(&context, &CpuMesh::cylinder(8)),
        ColorMaterial {
            color: orange,
            ..Default::default()
        },
    );
    pointer_line.set_transformation(
        pointer_group
            * x_to_y
            * Mat4::from_translation(vec3(-0.9, 0.0, 0.0))
            * Mat4::from_nonuniform_scale(1.8, 0.03, 0.03),
    );

    let mut pointer_cone = Gm::new(
        Mesh::new(&context, &CpuMesh::cone(8)),
        ColorMaterial {
            color: orange,
            ..Default::default()
        },
    );
    pointer_cone.set_transformation(
        pointer_group
            * x_to_y
            * Mat4::from_translation(vec3(0.85, 0.0, 0.0))
            * Mat4::from_nonuniform_scale(0.3, 0.1, 0.1),
    );

    // Arrow shaft runs from local x=-0.9 (tail) to x=1.15 (cone tip); anchor
    // the label slightly past the tail end so it sits opposite the arrowhead.
    #[cfg(target_arch = "wasm32")]
    let arg_perigee_label_pos = mul_point(pointer_group * x_to_y, vec3(-1.1, 0.0, 0.0));

    // --- Equatorial plane (blue, transparent), tilted 115° on X ---
    let transparent_states = RenderStates {
        cull: Cull::None,
        blend: Blend::TRANSPARENCY,
        write_mask: WriteMask::COLOR,
        depth_test: DepthTest::Less,
    };
    let mut eq_plane = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: Srgba::new(66, 135, 245, 76),
            render_states: transparent_states,
            is_transparent: true,
            ..Default::default()
        },
    );
    eq_plane.set_transformation(Mat4::from_angle_x(degrees(115.0)) * Mat4::from_scale(3.5));

    // --- Orbital plane (red, transparent), tilted 160°/45° ---
    let orbital_rot = Mat4::from_angle_x(degrees(160.0)) * Mat4::from_angle_z(degrees(45.0));
    let mut orbital_plane = Gm::new(
        Mesh::new(&context, &CpuMesh::square()),
        ColorMaterial {
            color: Srgba::new(245, 66, 66, 76),
            render_states: transparent_states,
            is_transparent: true,
            ..Default::default()
        },
    );
    orbital_plane.set_transformation(orbital_rot * Mat4::from_scale(4.0));

    // --- Orbit ellipse: dashed white circle in orbital plane ---
    let orbit = make_dashed_circle(
        &context,
        3.5,
        orbital_rot,
        Srgba::new(255, 255, 255, 255),
        0.025,
        28,
        0.625,
    );

    // --- Line of nodes (yellow) ---
    let line_of_nodes = make_tube_strip(
        &context,
        &[vec3(-3.5, 0.0, 0.0), vec3(3.5, 0.0, 0.0)],
        Srgba::new(255, 255, 0, 255),
        0.02,
    );

    // --- RAAN arc (orange, quadratic Bezier) ---
    // Start at the ascending-node yellow marker, end at the green X-arrow tip.
    // The cone tip sits at `axis_length + 0.15` along each axis (see axis cone
    // scale + translate above), then earth_tilt rotates it into world coords.
    let arrow_tip_along_axis = axis_length + 0.15;
    let raan_start = vec3(3.5, 0.0, 0.0);
    let raan_end = mul_point(earth_tilt, vec3(0.0, 0.0, arrow_tip_along_axis));
    let raan_control = vec3(2.5, -3.0, 2.5);
    let raan_pts: Vec<Vec3> = (0..=32)
        .map(|i| bezier_quad(raan_start, raan_control, raan_end, i as f32 / 32.0))
        .collect();
    let raan_arc = make_tube_strip(&context, &raan_pts, orange, 0.02);

    // --- Inclination arc (purple, quadratic Bezier) ---
    let eq_pt = vec3(
        3.5,
        3.5 * (115.0_f32.to_radians()).cos(),
        3.5 * (115.0_f32.to_radians()).sin(),
    );
    let temp_orbit_pt = vec3(
        3.5 * (45.0_f32.to_radians()).cos(),
        3.5 * (45.0_f32.to_radians()).sin(),
        0.0,
    );
    let cos_x = (160.0_f32.to_radians()).cos();
    let sin_x = (160.0_f32.to_radians()).sin();
    let orbit_pt = vec3(
        temp_orbit_pt.x,
        temp_orbit_pt.y * cos_x - temp_orbit_pt.z * sin_x,
        temp_orbit_pt.y * sin_x + temp_orbit_pt.z * cos_x,
    );
    let inc_control = vec3(
        4.5,
        (eq_pt.y + orbit_pt.y) / 2.0,
        (eq_pt.z + orbit_pt.z) / 2.0,
    );
    let inc_pts: Vec<Vec3> = (0..=32)
        .map(|i| bezier_quad(eq_pt, inc_control, orbit_pt, i as f32 / 32.0))
        .collect();
    let inc_arc = make_tube_strip(&context, &inc_pts, Srgba::new(128, 0, 128, 255), 0.02);

    // --- Node markers ---
    let yellow = Srgba::new(255, 255, 0, 255);
    let white = Srgba::new(255, 255, 255, 255);

    let mut descending_node = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        ColorMaterial {
            color: yellow,
            ..Default::default()
        },
    );
    descending_node
        .set_transformation(Mat4::from_translation(vec3(-3.5, 0.0, 0.0)) * Mat4::from_scale(0.1));

    let mut ascending_node = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        ColorMaterial {
            color: yellow,
            ..Default::default()
        },
    );
    ascending_node
        .set_transformation(Mat4::from_translation(vec3(3.5, 0.0, 0.0)) * Mat4::from_scale(0.1));

    let mut perigee_marker = Gm::new(
        Mesh::new(&context, &CpuMesh::sphere(16)),
        ColorMaterial {
            color: white,
            ..Default::default()
        },
    );
    perigee_marker.set_transformation(
        Mat4::from_translation(vec3(2.75, 2.0, -0.75)) * Mat4::from_scale(0.1),
    );

    // --- Cache the JS batch-update function (one boundary crossing per frame instead of 20+) ---
    #[cfg(target_arch = "wasm32")]
    let update_labels_fn: js_sys::Function = {
        use wasm_bindgen::JsCast;
        let win = web_sys::window().expect("no window");
        js_sys::Reflect::get(&win, &wasm_bindgen::JsValue::from_str("__updateLabels"))
            .expect("__updateLabels not on window")
            .dyn_into::<js_sys::Function>()
            .expect("__updateLabels is not a function")
    };

    // All label world positions are fixed (none move with Earth rotation), so precompute once.
    #[cfg(target_arch = "wasm32")]
    let label_world_positions: [Vec3; 10] = [
        mul_point(earth_tilt, vec3(0.0, axis_length + 0.5, 0.0)),  // lbl-z
        mul_point(earth_tilt, vec3(axis_length + 0.5, 0.0, 0.0)),  // lbl-y
        mul_point(earth_tilt, vec3(0.0, 0.0, axis_length + 0.5)),  // lbl-x
        vec3(3.0, -2.0, 2.0),                                       // lbl-inclination
        vec3(2.6, 2.6, -0.5),                                       // lbl-perigee
        arg_perigee_label_pos,                                       // lbl-arg-perigee
        vec3(raan_start.x / 2.0, -2.0, (axis_length - 0.3) / 3.0), // lbl-raan
        vec3(-3.5, 0.5, 0.0),                                       // lbl-desc-node
        vec3(4.5, 0.5, 0.0),                                        // lbl-asc-node
        vec3(0.0, -0.5, 0.0),                                       // lbl-nodes
    ];

    // --- Render loop ---
    let mut earth_y_rotation: f32 = 0.0;
    let mut drag_velocity: (f32, f32) = (0.0, 0.0);
    let mut is_dragging = false;

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);

        // Pre-scan events to track drag state and capture velocity for inertia.
        // OrbitControl marks MouseMotion as handled but not press/release, so
        // we read them here before handle_events consumes them.
        for event in frame_input.events.iter() {
            match event {
                Event::MousePress { button: MouseButton::Left, .. } => {
                    is_dragging = true;
                    drag_velocity = (0.0, 0.0);
                }
                Event::MouseRelease { button: MouseButton::Left, .. } => {
                    is_dragging = false;
                }
                Event::MouseMotion {
                    button: Some(MouseButton::Left),
                    delta,
                    handled,
                    ..
                } if !*handled => {
                    drag_velocity = *delta;
                }
                _ => {}
            }
        }

        control.handle_events(&mut camera, &mut frame_input.events);

        // Inertia: decay rate of 3.0/s matches Three.js dampingFactor=0.05 at 60fps.
        if !is_dragging {
            let decay = (-3.0 * frame_input.elapsed_time as f32 / 1000.0).exp();
            drag_velocity.0 *= decay;
            drag_velocity.1 *= decay;
            if drag_velocity.0.abs() > 0.01 || drag_velocity.1.abs() > 0.01 {
                camera.rotate_around_with_fixed_up(
                    control.target,
                    0.01 * drag_velocity.0,
                    0.01 * drag_velocity.1,
                );
            }
        }

        earth_y_rotation += (frame_input.elapsed_time as f32) * 6.0e-5;
        let earth_transform = earth_tilt * Mat4::from_angle_y(Rad(earth_y_rotation));
        earth_wireframe.set_transformation(earth_transform);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some((w, h)) = get_css_size() {
                let view_proj = camera.projection() * camera.view();
                let mut coords = [0f32; 20];
                for (i, &world) in label_world_positions.iter().enumerate() {
                    let (x, y) = project_to_screen(world, view_proj, w, h);
                    coords[i * 2] = x;
                    coords[i * 2 + 1] = y;
                }
                let arr = unsafe { js_sys::Float32Array::view(&coords) };
                let _ = update_labels_fn.call1(&wasm_bindgen::JsValue::NULL, &arr);
            }
        }

        let mut objects: Vec<&dyn Object> = Vec::new();
        objects.push(&earth_wireframe);
        for a in &axis_meshes {
            objects.push(a);
        }
        objects.push(&pointer_line);
        objects.push(&pointer_cone);
        objects.push(&eq_plane);
        objects.push(&orbital_plane);
        objects.push(&orbit);
        objects.push(&line_of_nodes);
        objects.push(&raan_arc);
        objects.push(&inc_arc);
        objects.push(&descending_node);
        objects.push(&ascending_node);
        objects.push(&perigee_marker);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render(&camera, objects, &[]);

        FrameOutput::default()
    });
}

// ---------------------------------------------------------------------
// Globe wireframe: GL_LINES geometry
// ---------------------------------------------------------------------

/// Lat/lon grid drawn with GL_LINES primitives. Each pair of consecutive
/// vertices in the buffer is one line segment (GL_LINES topology).
struct GlobeLines {
    positions: VertexBuffer<Vec3>,
    vertex_count: u32,
    context: Context,
    transformation: Mat4,
    aabb: AxisAlignedBoundingBox,
}

impl GlobeLines {
    fn new(
        context: &Context,
        earth_radius: f32,
        lat_count: u32,
        lon_count: u32,
        lat_segs: u32,
        mer_segs: u32,
    ) -> Self {
        let pts = build_globe_line_vertices(earth_radius, lat_count, lon_count, lat_segs, mer_segs);
        let aabb = AxisAlignedBoundingBox::new_with_positions(&pts);
        let vertex_count = pts.len() as u32;
        Self {
            positions: VertexBuffer::new_with_data(context, &pts),
            vertex_count,
            context: context.clone(),
            transformation: Mat4::identity(),
            aabb,
        }
    }

    fn set_transformation(&mut self, t: Mat4) {
        self.transformation = t;
    }
}

impl Geometry for GlobeLines {
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("modelMatrix", self.transformation);
        program.use_vertex_attribute("position", &self.positions);
        let count = self.vertex_count;
        let context = self.context.clone();
        program.draw_with(render_states, viewer.viewport(), move || unsafe {
            context.draw_arrays(three_d::context::LINES, 0, count as i32);
        });
    }

    fn vertex_shader_source(&self) -> String {
        "uniform mat4 viewProjection;
uniform mat4 modelMatrix;
in vec3 position;
out vec3 pos;
out vec4 col;
flat out int instance_id;
void main() {
    vec4 worldPos = modelMatrix * vec4(position, 1.0);
    pos = worldPos.xyz;
    col = vec4(1.0);
    instance_id = gl_InstanceID;
    gl_Position = viewProjection * worldPos;
}"
        .to_string()
    }

    fn id(&self) -> GeometryId {
        GeometryId(1)
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        if let Err(e) = render_with_material(&self.context, viewer, self, material, lights) {
            panic!("{}", e);
        }
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        if let Err(e) =
            render_with_effect(&self.context, viewer, self, material, lights, color_texture, depth_texture)
        {
            panic!("{}", e);
        }
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb.transformed(self.transformation)
    }
}

/// Builds a flat Vec of vertex pairs for GL_LINES: each consecutive pair is one
/// segment. Latitude parallels are circles at fixed latitude; meridians run
/// pole-to-pole.
fn build_globe_line_vertices(
    earth_radius: f32,
    lat_count: u32,
    lon_count: u32,
    lat_segs: u32,
    mer_segs: u32,
) -> Vec<Vec3> {
    let mut pts = Vec::new();

    // Latitude parallels (skip the poles where radius collapses to zero)
    for i in 1..lat_count {
        let phi = (i as f32 / lat_count as f32) * PI;
        let y = earth_radius * phi.cos();
        let r = earth_radius * phi.sin();
        for j in 0..lat_segs {
            let u0 = (j as f32 / lat_segs as f32) * TAU;
            let u1 = ((j + 1) as f32 / lat_segs as f32) * TAU;
            pts.push(vec3(r * u0.cos(), y, r * u0.sin()));
            pts.push(vec3(r * u1.cos(), y, r * u1.sin()));
        }
    }

    // Longitude meridians (pole to pole)
    for j in 0..lon_count {
        let theta = (j as f32 / lon_count as f32) * TAU;
        let (st, ct) = theta.sin_cos();
        for i in 0..mer_segs {
            let phi0 = (i as f32 / mer_segs as f32) * PI;
            let phi1 = ((i + 1) as f32 / mer_segs as f32) * PI;
            let (sp0, cp0) = phi0.sin_cos();
            let (sp1, cp1) = phi1.sin_cos();
            pts.push(vec3(earth_radius * sp0 * ct, earth_radius * cp0, earth_radius * sp0 * st));
            pts.push(vec3(earth_radius * sp1 * ct, earth_radius * cp1, earth_radius * sp1 * st));
        }
    }

    pts
}

// ---------------------------------------------------------------------
// Geometry helpers
// ---------------------------------------------------------------------

fn rotation_x_to_dir(dir: Vec3) -> Mat4 {
    let from = vec3(1.0, 0.0, 0.0);
    let dot = from.dot(dir);
    if dot > 0.9999 {
        return Mat4::identity();
    }
    if dot < -0.9999 {
        return Mat4::from_angle_z(degrees(180.0));
    }
    let axis = from.cross(dir).normalize();
    Mat4::from_axis_angle(axis, Rad(dot.acos()))
}

fn bezier_quad(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
    let m = 1.0 - t;
    p0 * (m * m) + p1 * (2.0 * m * t) + p2 * (t * t)
}

fn mul_point(m: Mat4, p: Vec3) -> Vec3 {
    let v = m * p.extend(1.0);
    vec3(v.x, v.y, v.z)
}

fn tube_segment_transform(start: Vec3, end: Vec3, radius: f32) -> Option<Mat4> {
    let dir = end - start;
    let length = dir.magnitude();
    if length < 1e-6 {
        return None;
    }
    let unit = dir / length;
    let rotation = rotation_x_to_dir(unit);
    // Extend one radius on each end so end caps are buried inside adjacent segments,
    // hiding the junction artifacts.
    Some(
        Mat4::from_translation(start - unit * radius)
            * rotation
            * Mat4::from_nonuniform_scale(length + 2.0 * radius, radius, radius),
    )
}

fn make_tube_strip(
    context: &Context,
    points: &[Vec3],
    color: Srgba,
    radius: f32,
) -> Gm<InstancedMesh, ColorMaterial> {
    let mut transformations: Vec<Mat4> = Vec::with_capacity(points.len().saturating_sub(1));
    for w in points.windows(2) {
        if let Some(t) = tube_segment_transform(w[0], w[1], radius) {
            transformations.push(t);
        }
    }
    Gm::new(
        InstancedMesh::new(
            context,
            &Instances {
                transformations,
                ..Default::default()
            },
            &CpuMesh::cylinder(8),
        ),
        ColorMaterial {
            color,
            ..Default::default()
        },
    )
}

/// Render a circle in the local XY plane (then transformed by `transform`) as a
/// dashed ring of tube segments.
fn make_dashed_circle(
    context: &Context,
    radius: f32,
    transform: Mat4,
    color: Srgba,
    tube_radius: f32,
    dash_count: u32,
    dash_fraction: f32,
) -> Gm<InstancedMesh, ColorMaterial> {
    let mut transformations: Vec<Mat4> = Vec::new();
    let cycle_angle = TAU / dash_count as f32;
    let dash_angle = cycle_angle * dash_fraction;
    let pts_per_dash = 4_u32;

    for d in 0..dash_count {
        let start_angle = d as f32 * cycle_angle;
        let mut points: Vec<Vec3> = Vec::with_capacity((pts_per_dash + 1) as usize);
        for i in 0..=pts_per_dash {
            let theta = start_angle + (i as f32 / pts_per_dash as f32) * dash_angle;
            let local = vec3(radius * theta.cos(), radius * theta.sin(), 0.0);
            points.push(mul_point(transform, local));
        }
        for w in points.windows(2) {
            if let Some(t) = tube_segment_transform(w[0], w[1], tube_radius) {
                transformations.push(t);
            }
        }
    }

    Gm::new(
        InstancedMesh::new(
            context,
            &Instances {
                transformations,
                ..Default::default()
            },
            &CpuMesh::cylinder(8),
        ),
        ColorMaterial {
            color,
            ..Default::default()
        },
    )
}

// ---------------------------------------------------------------------
// Browser overlay labels
// ---------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
fn get_css_size() -> Option<(f32, f32)> {
    let win = web_sys::window()?;
    let w = win.inner_width().ok()?.as_f64()? as f32;
    let h = win.inner_height().ok()?.as_f64()? as f32;
    Some((w, h))
}

#[cfg(target_arch = "wasm32")]
fn project_to_screen(world: Vec3, view_proj: Mat4, width: f32, height: f32) -> (f32, f32) {
    let clip = view_proj * world.extend(1.0);
    if clip.w <= 0.0 {
        return (-9999.0, -9999.0);
    }
    let sx = (clip.x / clip.w + 1.0) * 0.5 * width;
    let sy = (1.0 - clip.y / clip.w) * 0.5 * height;
    (sx, sy)
}
