mod canvas;
mod websocket;

use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    websocket::connect_websocket()?;
    let (canvas, gl) = canvas::create_context()?;
    let vertex_shader = create_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        "#version 300 es
        in vec4 a_position;
        out vec4 v_colour;
        void main() {
          gl_Position = a_position;
          v_colour = gl_Position * 0.5 + 0.5;
        }",
    )?;
    let fragment_shader = create_shader(
        &gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        "#version 300 es
        precision highp float;
        in vec4 v_colour;
        out vec4 out_colour;
        void main() {
          out_colour = v_colour;
        }",
    )?;
    let shader_program = create_program(&gl, &vertex_shader, &fragment_shader)?;
    draw_scene(
        &gl,
        canvas.client_width(),
        canvas.client_height(),
        shader_program,
    )?;
    Ok(())
}

fn create_program(
    gl: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, JsValue> {
    let program = gl.create_program().expect("Failed to create program.");
    gl.attach_shader(&program, &vertex_shader);
    gl.attach_shader(&program, &fragment_shader);
    gl.link_program(&program);
    let success = gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .expect("Failed to cast boolean.");
    if success {
        Ok(program)
    } else {
        let error = gl
            .get_program_info_log(&program)
            .expect("Failed to get program info log.");
        console::error_1(&error.into());
        Err(JsValue::from("Failed to link program."))
    }
}

fn create_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    shader_src: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .expect("Failed to create shader.");
    gl.shader_source(&shader, shader_src);
    gl.compile_shader(&shader);
    let success = gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .expect("Failed to cast boolean.");
    if success {
        Ok(shader)
    } else {
        let error = gl
            .get_shader_info_log(&shader)
            .expect("Failed to get shader info log.");
        console::error_1(&error.into());
        Err(JsValue::from("Failed to compile shader."))
    }
}

fn draw_scene(
    gl: &WebGl2RenderingContext,
    width: i32,
    height: i32,
    shader_program: WebGlProgram,
) -> Result<(), JsValue> {
    let pos_attr_loc = gl.get_attrib_location(&shader_program, "a_position") as u32;
    let pos_buffer = gl.create_buffer().expect("Failed to create buffer.");
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&pos_buffer));
    let positions: Vec<f32> = vec![0.0, 0.0, 0.0, 0.5, 0.7, 0.0];
    unsafe {
        let vertices = js_sys::Float32Array::view(&positions[..]);
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    let vao = gl
        .create_vertex_array()
        .expect("Failed to create vertex array.");
    gl.bind_vertex_array(Some(&vao));
    gl.enable_vertex_attrib_array(pos_attr_loc);
    gl.vertex_attrib_pointer_with_i32(pos_attr_loc, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.viewport(0, 0, width, height);
    gl.use_program(Some(&shader_program));
    gl.bind_vertex_array(Some(&vao));
    gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3);
    Ok(())
}
