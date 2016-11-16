use cgmath::{Vector3, Quaternion, Zero, One};
use glium::uniforms::{AsUniformValue, UniformValue};
use types::{Scalar, Mat4};

pub struct Transform {
    /// Object scale, default `1`.
    pub scale: Scalar,
    /// Object translation, default `(0, 0, 0)`.
    pub position: Vector3<Scalar>,
    /// Object rotation.
    pub rotation: Quaternion<Scalar>,
    pub matrix: Mat4,
}


impl Transform {
    pub fn new() -> Transform {
        let trans = Vector3::zero();
        let scale = Scalar::one();
        let rot = Quaternion::zero();
        let mat = Transform::build_matrix(trans, scale, rot);
        Transform {
            scale: scale,
            position: trans,
            rotation: rot,
            matrix: mat,
        }
    }

    fn build_matrix(trans: Vector3<Scalar>, scale: Scalar, rot: Quaternion<Scalar>)
        -> Mat4
    {
        Mat4::from_translation(trans) * Mat4::from(rot) * Mat4::from_scale(scale)
    }
}

impl<'a> AsUniformValue for &'a Transform {
    fn as_uniform_value(&self) -> UniformValue {
        let matrix: &[[Scalar; 4]; 4] = self.matrix.as_ref();
        matrix.as_uniform_value()
    }
}
