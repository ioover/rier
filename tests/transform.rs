extern crate rier;
extern crate cgmath;

use rier::transform::Transform;
use cgmath::{Matrix4, One};


#[test]
fn new_transform() {
    let trans = Transform::new();
    assert!(trans.matrix == Matrix4::one());
}
