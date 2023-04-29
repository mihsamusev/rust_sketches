extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
mod app;
mod constants;
mod glsetup;
mod glutils;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    gl: GL,
    program_flat: programs::Color2D,
    program_grad: programs::Color2DGrad,
    program_graph3d: programs::Graph3D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = glsetup::init_webgl_conetxt().unwrap();
        Self {
            program_flat: programs::Color2D::new(&gl),
            program_grad: programs::Color2DGrad::new(&gl),
            program_graph3d: programs::Graph3D::new(&gl),
            gl,
        }
    }

    pub fn update(
        &mut self,
        time: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) -> Result<(), JsValue> {
        app::update_dynamic_data(time, canvas_height, canvas_width);
        Ok(())
    }

    pub fn render(&self) {
        let app_state = app::get_app_state();
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        // flat rectangle
        self.program_flat.render(
            &self.gl,
            app_state.bottom,
            app_state.top,
            app_state.left,
            app_state.right,
            app_state.canvas_height,
            app_state.canvas_width,
        );

        // gradient rectangle
        // let border = 20.0;
        // self.program_grad.render(
        //     &self.gl,
        //     app_state.bottom + border,
        //     app_state.top - border,
        //     app_state.left + border,
        //     app_state.right - border,
        //     app_state.canvas_height,
        //     app_state.canvas_width,
        // );

        // grid graph
        self.program_graph3d.render(
            &self.gl,
            app_state.bottom,
            app_state.top,
            app_state.left,
            app_state.right,
            app_state.canvas_height,
            app_state.canvas_width,
            app_state.rotation_x_rad,
            app_state.rotation_y_rad,
            &glutils::get_grid_y(app_state.time),
        );
    }
}
