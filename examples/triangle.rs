extern crate rier;
#[macro_use] extern crate glium;
use glium::uniforms::EmptyUniforms;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex! {Vertex, position, color}

struct Shader;

impl rier::graphics::Shader for Shader {
    type Vertex = Vertex;
    type Uniforms = EmptyUniforms;

    fn vertex() -> &'static str {
        r#"
        #version 330 core
        in vec2 position;
        in vec3 color;
        out vec3 v_color;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            v_color = color;
        }
        "#
    }

    fn fragment() -> &'static str {
        r#"
        #version 330 core
        in vec3 v_color;
        out vec4 f_color;
        void main() {
            f_color = vec4(v_color, 1.0);
        }
        "#
    }
}



fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_title("triangle")
        .with_dimensions(800, 600)
        .build_glium()
        .unwrap();
    let target = rier::graphics::Target::from_surface(display.draw());
    let renderer = rier::graphics::Renderer::<Shader, _>::new(display.clone(), target.clone()).unwrap();
    let mesh = rier::mesh::Mesh::new(&display, glium::index::PrimitiveType::TrianglesList, &[
            Vertex { position: [-1., -1.], color: [0., 1., 0.] },
            Vertex { position: [ 0.,  1.], color: [0., 0., 1.] },
            Vertex { position: [ 1., -1.], color: [1., 0., 0.] },
        ]).unwrap();

    'main: loop {
        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => break 'main,
                _ => (),
            }
        }
        renderer.draw(&mesh, &EmptyUniforms).unwrap();
        target.swap_buffers(&display).unwrap();
    }
    target.finish().unwrap();
}
