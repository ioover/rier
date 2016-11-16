use cgmath::{Vector3, Matrix4, Quaternion, Zero, One};
use glium::uniforms::{AsUniformValue, UniformValue};

type Scalar = f32;

pub struct Transform {
    /// Object scale, default `1`.
    pub scale: Scalar,
    /// Object translation, default `(0, 0, 0)`.
    pub position: Vector3<Scalar>,
    /// Object rotation.
    pub rotation: Quaternion<Scalar>,
    pub matrix: Matrix4<Scalar>,
}


impl Transform {
    pub fn new() -> Transform {
        let trans = Vector3::zero();
        let scale = Scalar::one();
        let rot = Quaternion::zero();
        let mat = Matrix4::from_translation(trans)*Matrix4::from(rot)*Matrix4::from_scale(scale);
        Transform {
            scale: scale,
            position: trans,
            rotation: rot,
            matrix: mat,
        }
    }
}

impl<'a> AsUniformValue for &'a Transform {
    fn as_uniform_value(&self) -> UniformValue {
        let matrix: &[[Scalar; 4]; 4] = self.matrix.as_ref();
        matrix.as_uniform_value()
    }
}
