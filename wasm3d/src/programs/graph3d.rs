use crate::{
    constants, glutils,
    shaders::{fragment, vertex},
};
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

pub struct Graph3D {
    pub program: WebGlProgram,
    pub position_buf: WebGlBuffer,
    pub indices_buf: WebGlBuffer,
    pub normals_buf: WebGlBuffer,
    pub index_count: i32,
    pub u_opacity: WebGlUniformLocation,
    pub u_projection: WebGlUniformLocation,
    pub u_normals_rotation: WebGlUniformLocation,
    pub y_buffer: WebGlBuffer,
}

impl Graph3D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        // get a program
        let program = glutils::link_program(
            &gl,
            &vertex::graph3d::SHADER,
            &fragment::color_by_vertex::SHADER,
        )
        .unwrap();

        // positions and indices
        //let verts: [f32; 8] = [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];
        //let idxs: [u16; 6] = [0, 1, 2, 2, 1, 3];
        let (verts, idxs) = glutils::get_grid_positions(constants::GRID_SIZE);

        let vert_js_array = glutils::to_js_float32_array(&verts);
        let vert_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vert_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_js_array, GL::STATIC_DRAW);

        // put indeices into buffer
        let idx_js_array = glutils::to_js_uint16_array(&idxs);
        let idx_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&idx_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &idx_js_array,
            GL::STATIC_DRAW,
        );

        // output self
        Self {
            u_normals_rotation: gl
                .get_uniform_location(&program, "uNormalsRotation")
                .unwrap(),
            u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            y_buffer: gl
                .create_buffer()
                .ok_or("failed to create y buffer")
                .unwrap(),
            normals_buf: gl
                .create_buffer()
                .ok_or("failed to create normals buffer")
                .unwrap(),
            position_buf: vert_buffer,
            indices_buf: idx_buffer,
            index_count: idxs.len() as i32,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
        rotation_x_rad: f32,
        rotation_y_rad: f32,
        y_values: &Vec<f32>,
    ) {
        gl.use_program(Some(&self.program));

        // populate matrix uniforms
        let matrices = glutils::get_3d_matrices(
            bottom,
            top,
            left,
            right,
            canvas_height,
            canvas_width,
            rotation_x_rad,
            rotation_y_rad,
        );

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, &matrices.projection);
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.u_normals_rotation),
            false,
            &matrices.normals_rotation,
        );

        // populate opacity uniform
        gl.uniform1f(Some(&self.u_opacity), 1.0);

        // populate vertex buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buf));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        // populate y buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 1, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let y_jsarray = glutils::to_js_float32_array(y_values);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &y_jsarray, GL::DYNAMIC_DRAW);

        // populate normals buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buf));
        gl.vertex_attrib_pointer_with_i32(2, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(2);

        let normals = glutils::get_grid_normals(constants::GRID_SIZE, &y_values);
        let normals_jsarray = glutils::to_js_float32_array(&normals);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &normals_jsarray, GL::DYNAMIC_DRAW);

        // draw object
        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);
    }
}
