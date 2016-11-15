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

    fn vertex() -> &'static str {
        "#version 330 core\nvoid main(void) {}"
    }

    fn fragment() -> &'static str {
        "#version 330 core\nvoid main(void) {}"
    }
}


#[test]
fn renderer() {
    let gfx = rier::graphics::Graphics::new(create_display()).gfx();
    rier::graphics::Renderer::<Empty>::new(gfx.clone()).unwrap();
}
