#[macro_use] extern crate rier;
#[macro_use] extern crate glium;
extern crate image;
extern crate cgmath;
use cgmath::One;


#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

implement_vertex! {Vertex, position, texcoord}


struct Shader;

impl rier::graphics::Shader for Shader {
    type Vertex = Vertex;
    type Uniforms = Sprite;

    fn vertex() -> &'static str {
r#"#version 330 core
uniform mat4 matrix;
in vec2 position;
in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    v_texcoord = texcoord;
}
"#
    }

    fn fragment() -> &'static str {
r#"#version 330 core
uniform sampler2D tex;
in vec2 v_texcoord;
out vec4 f_color;
void main() {
    f_color = texture(tex, v_texcoord);
}"#
    }
}


struct Sprite {
    matrix: rier::Mat4,
    tex: glium::texture::CompressedSrgbTexture2d,
    mesh: rier::Mesh<Vertex>,
}

implement_uniforms!{Sprite, matrix, tex}

fn main() {
    use glium::DisplayBuild;

    let display = glium::glutin::WindowBuilder::new()
        .with_title("triangle")
        .with_dimensions(32, 32)
        .build_glium()
        .unwrap();
    
    let texture = {
        use std::io::Cursor;
        let raw_image = Cursor::new(&include_bytes!("assets/icon.png")[..]);
        let image = image::load(raw_image, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        glium::texture::CompressedSrgbTexture2d::new(&display, image).unwrap()
    };
    let target = rier::graphics::Target::from_surface(display.draw());
    let renderer = rier::graphics::Renderer::<Shader, _>::new(display.clone(), target.clone()).unwrap();
    let mesh = rier::mesh::Mesh::new(&display, glium::index::PrimitiveType::TriangleStrip, &[
        Vertex { position: [-1.,  1.], texcoord: [0., 1.] },
        Vertex { position: [ 1.,  1.], texcoord: [1., 1.] },
        Vertex { position: [-1., -1.], texcoord: [0., 0.] },
        Vertex { position: [ 1., -1.], texcoord: [1., 0.] },
    ]).unwrap();
    
    let sprite = Sprite {
        matrix: rier::Mat4::one(),
        tex: texture,
        mesh: mesh,
    };
    'main: loop {
        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => break 'main,
                _ => (),
            }
        }
        renderer.draw(&sprite.mesh, &sprite).unwrap();
        target.swap_buffers(&display).unwrap();
    }
    target.finish().unwrap();

}
