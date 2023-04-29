use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

// have many read only copues and update never mutates
lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_height: f32, canvas_width: f32) {
    let mut data = APP_STATE.lock().unwrap();

    let min_size = canvas_height.min(canvas_width);
    let display_size = 0.9 * min_size;

    *data = Arc::new(AppState {
        canvas_height,
        canvas_width,
        time,
        bottom: (canvas_height - display_size) / 2.0,
        top: (canvas_height + display_size) / 2.0,
        left: (canvas_width - display_size) / 2.0,
        right: (canvas_width + display_size) / 2.0,
        ..*data.clone()
    })
}

pub fn get_app_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
    let mut data = APP_STATE.lock().unwrap();
    *data = Arc::new(AppState {
        mouse_down: is_down,
        mouse_x: x,
        mouse_y: data.canvas_height - y,
        ..*data.clone()
    })
}

pub fn update_mouse_position(x: f32, y: f32) {
    let mut data = APP_STATE.lock().unwrap();

    let inv_y = data.canvas_height - y;
    let dx = x - data.mouse_x;
    let dy = inv_y - data.mouse_y;
    let mut dx_rot = 0.0;
    let mut dy_rot = 0.0;
    if data.mouse_down {
        dx_rot = std::f32::consts::PI * dy / data.canvas_height;
        dy_rot = std::f32::consts::PI * dx / data.canvas_width;
    }

    *data = Arc::new(AppState {
        mouse_x: x,
        mouse_y: inv_y,
        rotation_x_rad: data.rotation_x_rad - dx_rot,
        rotation_y_rad: data.rotation_y_rad + dy_rot,
        ..*data.clone()
    })
}

#[derive(Default)]
pub struct AppState {
    pub canvas_height: f32,
    pub canvas_width: f32,
    pub bottom: f32,
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub time: f32,
    pub mouse_down: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub rotation_x_rad: f32,
    pub rotation_y_rad: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mouse_down: false,
            mouse_x: -1.0,
            mouse_y: -1.0,
            rotation_x_rad: 0.5,
            rotation_y_rad: 0.5,
            ..Self::default()
        }
    }
}

pub struct CanvasState {
    pub height: f32,
    pub width: f32,
}
pub struct MouseState {
    pub is_down: bool,
    pub x: f32,
    pub y: f32,
}
