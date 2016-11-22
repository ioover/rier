#[macro_use] extern crate glium;
extern crate rier;
use glium::glutin::WindowBuilder;
use glium::{Display, DisplayBuild};

fn create_display() -> Display {
    WindowBuilder::new()
        .with_title("Rier Test")
        .with_dimensions(800, 600)
        .with_visibility(false)
        .build_glium()
        .unwrap()
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    texcoords: [f32; 2],
}

implement_vertex!(Vertex, position, texcoords);

struct Empty;


impl rier::graphics::Shader for Empty {
    type Vertex = Vertex;
    type Uniforms = glium::uniforms::EmptyUniforms;

    fn vertex() -> &'static str {
        "#version 330 core\nvoid main(void) {}"
    }

    fn fragment() -> &'static str {
        "#version 330 core\nvoid main(void) {}"
    }
}


#[test]
fn renderer() {
    let display = create_display();
    let target = rier::graphics::Target::from_surface(display.draw());
    rier::graphics::Renderer::<Empty, _>::new(display, target.clone()).unwrap();
    target.finish().unwrap();
}
