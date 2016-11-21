pub extern crate glium;
extern crate cgmath;

mod types;
pub mod camera;
pub mod transform;
pub mod graphics;
pub mod mesh;
pub use graphics::{Graphics, Gfx, Shader, Renderer};
pub use mesh::Mesh;
pub use types::{Mat4, Scalar};

#[macro_export]
macro_rules! implement_uniforms {
    ($tyname: ty, $( $field: ident ),*) => {
        impl $crate::glium::uniforms::Uniforms for $tyname {
            fn visit_values<'a, F>(&'a self, mut f: F)
                where F: FnMut(&str, $crate::glium::uniforms::UniformValue<'a>)
            {
                use $crate::glium::uniforms::AsUniformValue;
                $(f(stringify!($field), (&self.$field).as_uniform_value());)*
            }
        }
    };
}