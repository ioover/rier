extern crate rier;
#[macro_use] extern crate glium;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex! {Vertex, position, color}

struct Shader;

impl rier::graphics::Shader for Shader {
    type Vertex = Vertex;

    fn vertex() -> &'static str {
        r#"
        #version 330 core
        uniform mat4 matrix;
        in vec2 position;
        in vec3 color;
        out vec3 v_color;
        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
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
    let gfx = rier::graphics::Graphics::new(display).gfx();
    let renderer = rier::graphics::Renderer::<Shader>::new(gfx.clone()).unwrap();

    let mesh = rier::mesh::Mesh::new(&renderer, &[
            Vertex { position: [-1., -1.], color: [0., 1., 0.] },
            Vertex { position: [ 0.,  1.], color: [0., 0., 1.] },
            Vertex { position: [ 1., -1.], color: [1., 0., 0.] },
        ]).unwrap();

    'main: loop {
        for event in gfx.display.poll_events() {
            match event {
                glium::glutin::Event::Closed => break 'main,
                _ => (),
            }
        }
        let matrix: [[f32; 4]; 4] = [
            [1., 0., 0., 0.,],
            [0., 1., 0., 0.,],
            [0., 0., 1., 0.,],
            [0., 0., 0., 1.,],
        ];
        renderer.draw(&mesh, &uniform! { matrix: matrix }).unwrap();
        renderer.gfx.swap_buffers();
    }
}