use std::f32::consts::{PI, TAU};
use three_d::*;

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
    .unwrap();

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
    // Matches three.js SphereGeometry(2, 32, 32) — 32 meridians, 32 parallels,
    // each subdivided 32 times so the curves look smooth from any angle.
    let wire_polylines = sphere_wireframe_polylines(earth_radius, 32, 32);
    let wire_transforms = polylines_to_tube_segments(&wire_polylines, 0.012);
    let mut earth_wireframe = Gm::new(
        InstancedMesh::new(
            &context,
            &Instances {
                transformations: wire_transforms,
                ..Default::default()
            },
            &CpuMesh::cylinder(16),
        ),
        ColorMaterial {
            color: Srgba::new(0, 255, 65, 255),
            // Don't write depth — every lat/lon tube crosses every other tube,
            // and depth-fighting between them is what causes the shimmery /
            // fuzzy look as the camera rotates. They still test against the
            // scene depth, so opaque geometry in front still occludes them.
            render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                depth_test: DepthTest::Less,
                blend: Blend::Disabled,
                cull: Cull::Back,
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
        let rot = rotation_x_to_dir(dir);
        let mut cyl = Gm::new(
            Mesh::new(&context, &CpuMesh::cylinder(8)),
            ColorMaterial {
                color,
                ..Default::default()
            },
        );
        cyl.set_transformation(
            earth_tilt * rot * Mat4::from_nonuniform_scale(axis_length, 0.03, 0.03),
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
                * rot
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

    // --- Render loop ---
    let mut earth_y_rot: f32 = 0.0;
    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        earth_y_rot += (frame_input.elapsed_time as f32) * 6.0e-5;
        let earth_transform = earth_tilt * Mat4::from_angle_y(Rad(earth_y_rot));
        earth_wireframe.set_transformation(earth_transform);

        #[cfg(target_arch = "wasm32")]
        {
            if let Some((w, h)) = get_css_size() {
                let view_proj = camera.projection() * camera.view();
                let update = |id: &str, pos: Vec3| {
                    update_label(id, pos, view_proj, w, h);
                };
                update("lbl-z", mul_point(earth_tilt, vec3(0.0, axis_length + 0.5, 0.0)));
                update("lbl-y", mul_point(earth_tilt, vec3(axis_length + 0.5, 0.0, 0.0)));
                update("lbl-x", mul_point(earth_tilt, vec3(0.0, 0.0, axis_length + 0.5)));
                update("lbl-inclination", vec3(3.0, -2.0, 2.0));
                update("lbl-perigee", vec3(2.6, 2.6, -0.5));
                update("lbl-arg-perigee", arg_perigee_label_pos);
                update(
                    "lbl-raan",
                    vec3(raan_start.x / 2.0, -2.0, (axis_length - 0.3) / 3.0),
                );
                update("lbl-desc-node", vec3(-3.5, 0.5, 0.0));
                update("lbl-asc-node", vec3(4.5, 0.5, 0.0));
                update("lbl-nodes", vec3(0.0, -0.5, 0.0));
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
    let rot = rotation_x_to_dir(dir / length);
    Some(
        Mat4::from_translation(start) * rot * Mat4::from_nonuniform_scale(length, radius, radius),
    )
}

fn polylines_to_tube_segments(polylines: &[Vec<Vec3>], radius: f32) -> Vec<Mat4> {
    let mut transforms = Vec::new();
    for line in polylines {
        for w in line.windows(2) {
            if let Some(t) = tube_segment_transform(w[0], w[1], radius) {
                transforms.push(t);
            }
        }
    }
    transforms
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

/// Generates wireframe-sphere polylines: a set of latitude parallels and
/// longitude meridians at the given radius.
fn sphere_wireframe_polylines(radius: f32, lat_count: u32, lon_count: u32) -> Vec<Vec<Vec3>> {
    let mut polylines: Vec<Vec<Vec3>> = Vec::new();
    let segs_per_parallel: u32 = 32;
    let segs_per_meridian: u32 = 32;

    // Latitude parallels (skip the poles where r == 0)
    for i in 1..lat_count {
        let phi = (i as f32 / lat_count as f32) * PI;
        let y = radius * phi.cos();
        let r = radius * phi.sin();
        let mut line: Vec<Vec3> = Vec::with_capacity((segs_per_parallel + 1) as usize);
        for j in 0..=segs_per_parallel {
            let theta = (j as f32 / segs_per_parallel as f32) * TAU;
            line.push(vec3(r * theta.cos(), y, r * theta.sin()));
        }
        polylines.push(line);
    }

    // Longitude meridians (pole to pole)
    for i in 0..lon_count {
        let theta = (i as f32 / lon_count as f32) * TAU;
        let mut line: Vec<Vec3> = Vec::with_capacity((segs_per_meridian + 1) as usize);
        for j in 0..=segs_per_meridian {
            let phi = (j as f32 / segs_per_meridian as f32) * PI;
            line.push(vec3(
                radius * phi.sin() * theta.cos(),
                radius * phi.cos(),
                radius * phi.sin() * theta.sin(),
            ));
        }
        polylines.push(line);
    }

    polylines
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
fn update_label(id: &str, world: Vec3, view_proj: Mat4, width: f32, height: f32) {
    use wasm_bindgen::JsCast;
    let clip = view_proj * world.extend(1.0);
    let html = match web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    {
        Some(el) => el,
        None => return,
    };
    if clip.w <= 0.0 {
        let _ = html
            .style()
            .set_property("transform", "translate(-9999px, -9999px)");
        return;
    }
    let nx = clip.x / clip.w;
    let ny = clip.y / clip.w;
    let sx = (nx + 1.0) * 0.5 * width;
    let sy = (1.0 - ny) * 0.5 * height;
    let _ = html.style().set_property(
        "transform",
        &format!("translate({:.1}px, {:.1}px) translate(-50%, -50%)", sx, sy),
    );
}
