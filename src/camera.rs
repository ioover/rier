use cgmath::Ortho;
use glium::uniforms::{AsUniformValue, UniformValue};
use transform::Transform;
use types::{Scalar, Mat4};


/// Orthogonal 2D Camera.
pub struct Camera2D {
    pub transform: Transform,
    matrix: Mat4,
}


impl Camera2D {
    pub fn new(width: u32, height: u32) -> Camera2D {
        let transform = Transform::new();

        Camera2D {
            matrix: Camera2D::build_matrix(width, height, &transform.matrix),
            transform: transform,
        }
    }

    fn build_matrix(width: u32, height: u32, transform: &Mat4) -> Mat4 {
        let (w, h) = (width as Scalar, height as Scalar);
        let ortho = Ortho {
            left: 0.0,
            right: w,
            bottom: 0.0,
            top: h,
            near: -1.0,
            far: 1.0,
        };
        Mat4::from(ortho) * transform
    }
}

impl<'a> AsUniformValue for &'a Camera2D {
    fn as_uniform_value(&self) -> UniformValue {
        let matrix: &[[Scalar; 4]; 4] = self.matrix.as_ref();
        matrix.as_uniform_value()
    }
}
