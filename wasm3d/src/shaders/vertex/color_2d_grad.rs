pub const SHADER: &str = r#"
    attribute vec4 aPosition;
    attribute vec4 aColor;

    varying lowp vec4 vColor;

    uniform mat4 uTransform;
    void main() {
        vColor = aColor;
        gl_Position = uTransform * aPosition;
    }
"#;
