use super::super::{
    glutils,
    shaders::{fragment, vertex},
};
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlUniformLocation};

pub struct Color2D {
    program: WebGlProgram,
    vert_array_size: usize,
    vert_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program =
            glutils::link_program(&gl, vertex::color_2d::SHADER, fragment::color_2d::SHADER)
                .unwrap();

        // create a rectangle
        let vert_rect: [f32; 12] = [0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vert_location = vert_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vert_location, vert_location + vert_rect.len() as u32);

        let buffer_rect = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        Self {
            vert_array_size: vert_rect.len(),
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            vert_buffer: buffer_rect,
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

        // use buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vert_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0.0, 0.0, 0.5, 1.0);
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

        gl.draw_arrays(GL::TRIANGLES, 0, (self.vert_array_size / 2) as i32);
    }
}
