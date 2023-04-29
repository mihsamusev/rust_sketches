use super::super::{
    glutils,
    shaders::{fragment, vertex},
};
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

pub struct Color2DGrad {
    program: WebGlProgram,
    idx_array_size: usize,
    vert_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2DGrad {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = glutils::link_program(
            &gl,
            vertex::color_2d_grad::SHADER,
            fragment::color_by_vertex::SHADER,
        )
        .unwrap();

        // create a rectangle vertices and ccw indices
        let vert_rect: [f32; 8] = [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];
        let idx_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let vert_mem_buf = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vert_location = vert_rect.as_ptr() as u32 / 4; // because f32 is 4 bytes?
        let vert_array = js_sys::Float32Array::new(&vert_mem_buf)
            .subarray(vert_location, vert_location + vert_rect.len() as u32);

        let idx_mem_buf = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let idx_location = idx_rect.as_ptr() as u32 / 2; // because u16 is 2 bytes?
        let idx_array = js_sys::Uint16Array::new(&idx_mem_buf)
            .subarray(idx_location, idx_location + idx_rect.len() as u32);

        let vert_buf = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vert_buf));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let idx_buf = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&idx_buf));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &idx_array,
            GL::STATIC_DRAW,
        );

        Self {
            idx_array_size: idx_rect.len(),
            color_buffer: gl
                .create_buffer()
                .ok_or("Failed to create color buffer")
                .unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            vert_buffer: vert_buf,
            program,
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
    ) {
        gl.use_program(Some(&self.program));

        // set vertex buffer at index
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vert_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        // set color buffer at index 1
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let colors_rect: [f32; 4 * 4] = [
            1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ];

        let color_mem_buf = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let color_location = colors_rect.as_ptr() as u32 / 4; // because f32 is 4 bytes?
        let color_array = js_sys::Float32Array::new(&color_mem_buf)
            .subarray(color_location, color_location + colors_rect.len() as u32);
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &color_array, GL::DYNAMIC_DRAW);

        gl.uniform1f(Some(&self.u_opacity), 1.0);

        let translation = glutils::translation_matrix(
            2.0 * left / canvas_width - 1.0,
            2.0 * bottom / canvas_height - 1.0,
            0.0,
        );

        let scaling = glutils::scaling_matrix(
            2.0 * (right - left) / canvas_width,
            2.0 * (top - bottom) / canvas_height,
            0.0,
        );

        let transform = glutils::mat_mult4(scaling, translation);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform);

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            self.idx_array_size as i32,
            GL::UNSIGNED_SHORT,
            0,
        )
    }
}
