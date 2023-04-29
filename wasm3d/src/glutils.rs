use crate::constants;
use js_sys::WebAssembly;
use nalgebra::{Matrix4, Perspective3};
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

// Compiles and links a GPU program with 2 vertex and fragment shaders
pub fn link_program(
    gl: &WebGlRenderingContext,
    vert_source: &str,
    frag_source: &str,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

// Compiles a shader string as a specific gl shader type
// GL::VERTEX_SHADER or GL::FRGAMENT_SHADER
//
pub fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating shader"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Error getting shader info log")))
    }
}

pub fn load_glsl_shader(path: &str) -> Result<String, String> {
    Ok(path.to_owned())
}
// compose translation matrix of following form
// [1.0, 0.0, 0.0, 0.0]
// [0.0, 1.0, 0.0, 0.0]
// [0.0, 0.0, 1.0, 0.0]
// [tx,  ty,  tz,  1.0]
//
pub fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut mat = [0.0; 16];
    // diagonal
    mat[0] = 1.0;
    mat[5] = 1.0;
    mat[10] = 1.0;
    mat[15] = 1.0;

    // bottom row [tx, ty, tz, 1,0]
    mat[12] = tx;
    mat[13] = ty;
    mat[14] = tz;

    mat
}

// compose scaling matrix of following form
// [sx , 0.0, 0.0, 0.0]
// [0.0, sy , 0.0, 0.0]
// [0.0, 0.0, sz , 0.0]
// [0.0, 0.0, 0.0, 1.0]
//
pub fn scaling_matrix(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
    let mut mat = [0.0; 16];
    mat[0] = sx;
    mat[5] = sy;
    mat[10] = sz;
    mat[15] = 1.0;
    mat
}

pub fn mat_mult4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut mat = [0.; 16];

    mat[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    mat[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    mat[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    mat[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    mat[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    mat[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    mat[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    mat[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    mat[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    mat[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    mat[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    mat[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    mat[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    mat[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    mat[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    mat[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    mat
}

pub fn get_grid_positions(n_cells: usize) -> (Vec<f32>, Vec<u16>) {
    let n_pts = n_cells + 1;
    let mut pos: Vec<f32> = vec![0.0; 3 * n_pts * n_pts];
    let mut idx: Vec<u16> = vec![0; 6 * n_cells * n_cells];

    let size = 2.0 / n_cells as f32;

    for z in 0..n_pts {
        for x in 0..n_pts {
            // compute vertices
            let p_start = 3 * (z * n_pts + x);
            pos[p_start + 0] = -1.0 + x as f32 * size;
            // y is 0
            pos[p_start + 2] = -1.0 + z as f32 * size;

            // compute indices
            if z == n_cells || x == n_cells {
                continue;
            }

            let i_start = 6 * (z * n_cells + x);

            let bottom_left = (z * n_pts + x) as u16;
            let bottom_right = bottom_left + 1;
            let top_left = bottom_left + n_pts as u16;
            let top_right = bottom_right + n_pts as u16;

            idx[i_start + 0] = bottom_left;
            idx[i_start + 1] = bottom_right;
            idx[i_start + 2] = top_left;
            // upper tri of a cell
            idx[i_start + 3] = bottom_right;
            idx[i_start + 4] = top_right;
            idx[i_start + 5] = top_left;
        }
    }

    (pos, idx)
}

pub fn get_grid_y(time: f32) -> Vec<f32> {
    let n_pts = constants::GRID_SIZE + 1;
    let mut y = vec![0.0; n_pts * n_pts];

    let half_grid = n_pts as f32 / 2.0;
    let frequency = 3.0 * std::f32::consts::PI;
    let y_max = 0.15;
    let sin_offset = time / 100.0;

    for z in 0..n_pts {
        for x in 0..n_pts {
            let idx = z * n_pts + x;
            let scaled_x = frequency * (x as f32 - half_grid) / half_grid;
            let scaled_z = frequency * (z as f32 - half_grid) / half_grid;
            y[idx] =
                y_max * ((scaled_x * scaled_x + scaled_z * scaled_z).sqrt() + sin_offset).sin();
        }
    }
    y
}

pub struct Matrices3D {
    pub projection: [f32; 16],
    pub normals_rotation: [f32; 16],
}

// compose projction and normals rotation matrices
pub fn get_3d_matrices(
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
    rotation_x_rad: f32,
    rotation_y_rad: f32,
) -> Matrices3D {
    let mut matrices = Matrices3D {
        projection: [0.0; 16],
        normals_rotation: [0.0; 16],
    };

    // rotation matrix around x
    let mut x_rot: [f32; 16] = [0.0; 16];
    x_rot[0] = 1.0;
    x_rot[15] = 1.0;
    x_rot[5] = rotation_x_rad.cos();
    x_rot[6] = -rotation_x_rad.sin();
    x_rot[9] = rotation_x_rad.sin();
    x_rot[10] = rotation_x_rad.cos();

    // rotation matrix around y
    let mut y_rot: [f32; 16] = [0.0; 16];
    y_rot[5] = 1.0;
    y_rot[15] = 1.0;
    y_rot[0] = rotation_y_rad.cos();
    y_rot[2] = rotation_y_rad.sin();
    y_rot[8] = -rotation_y_rad.sin();
    y_rot[10] = rotation_y_rad.cos();

    let rotation_mat = mat_mult4(x_rot, y_rot);

    let aspect = canvas_width / canvas_height;
    let scale_x = (right - left) / canvas_width;
    let scale_y = (top - bottom) / canvas_height;
    let scale = scale_y;

    let translation_mat = translation_matrix(
        -1.0 + scale_x + 2.0 * left / canvas_width,
        -1.0 + scale_y + 2.0 * bottom / canvas_height,
        constants::Z_PLANE,
    );

    // combine matrices
    let scale_mat = scaling_matrix(scale, scale, 0.0);
    let rotate_and_scale = mat_mult4(rotation_mat, scale_mat);
    let transform_mat = mat_mult4(rotate_and_scale, translation_mat);

    // perspective transform
    let perspective_mat_temp = Perspective3::new(
        aspect,
        constants::FOV_RAD,
        constants::Z_NEAR,
        constants::Z_FAR,
    );
    let mut perspective_mat: [f32; 16] = [0.0; 16];
    perspective_mat.copy_from_slice(perspective_mat_temp.as_matrix().as_slice());

    matrices.projection = mat_mult4(transform_mat, perspective_mat);

    // normals
    let normal_matrix = Matrix4::from_row_slice(&rotation_mat);

    if let Some(inv_mat) = normal_matrix.try_inverse() {
        matrices
            .normals_rotation
            .copy_from_slice(inv_mat.as_slice());
    }

    matrices
}

pub fn get_triangle_normal(
    ax: f32,
    ay: f32,
    az: f32,
    bx: f32,
    by: f32,
    bz: f32,
    cx: f32,
    cy: f32,
    cz: f32,
) -> (f32, f32, f32) {
    let ux = bx - ax;
    let uy = by - ay;
    let uz = bz - az;

    let vx = cx - ax;
    let vy = cy - ay;
    let vz = cz - az;

    let nx = uy * vz - vy * uz;
    let ny = -ux * vz + vx * uz;
    let nz = ux * vy - vx * uy;
    let n_len = (nx * nx + ny * ny + nz * nz).sqrt();

    (nx / n_len, ny / n_len, nz / n_len)
}

pub fn get_grid_normals(n: usize, y_vals: &Vec<f32>) -> Vec<f32> {
    let points_per_row = n + 1;
    let graph_layout_width: f32 = 2.;
    let square_size: f32 = graph_layout_width / n as f32;
    let mut return_var: Vec<f32> = vec![0.; 3 * points_per_row * points_per_row];

    for z in 0..points_per_row {
        for x in 0..points_per_row {
            let y_val_index_a = z * points_per_row + x;
            let return_var_start_index = 3 * y_val_index_a;

            if z == n || x == n {
                return_var[return_var_start_index + 1] = 1.; //default
            } else {
                let y_val_index_b = y_val_index_a + points_per_row;
                let y_val_index_c = y_val_index_a + 1;

                let x_val_1 = square_size * x as f32;
                let x_val_2 = x_val_1 + square_size;

                let z_val_1 = square_size * z as f32;
                let z_val_2 = z_val_1 + square_size;

                let normals = get_triangle_normal(
                    x_val_1,
                    y_vals[y_val_index_a],
                    z_val_1,
                    x_val_1,
                    y_vals[y_val_index_b],
                    z_val_2,
                    x_val_2,
                    y_vals[y_val_index_c],
                    z_val_2,
                );

                return_var[return_var_start_index + 0] = normals.0;
                return_var[return_var_start_index + 1] = normals.1;
                return_var[return_var_start_index + 2] = normals.2;
            }
        }
    }

    return_var
}

pub fn to_js_float32_array(array: &[f32]) -> js_sys::Float32Array {
    let mem_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let array_ptr_start = array.as_ptr() as u32 / 4;
    let js_array = js_sys::Float32Array::new(&mem_buffer)
        .subarray(array_ptr_start, array_ptr_start + array.len() as u32);

    js_array
}

pub fn to_js_uint16_array(array: &[u16]) -> js_sys::Uint16Array {
    let mem_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let array_ptr_start = array.as_ptr() as u32 / 2; // because u16 is 2 bytes?
    let js_array = js_sys::Uint16Array::new(&mem_buffer)
        .subarray(array_ptr_start, array_ptr_start + array.len() as u32);

    js_array
}
