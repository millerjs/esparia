
use std::ops::{
    Add,
    Mul,
};

pub use float::{
    One,
    Zero,
};

use vecmath::{
    Matrix3,
    Matrix4,
    Vector3,
    Vector4,
    row_mat3_mul,
};

use types::{
    Mat3,
    Vec3,
};

use vecmath;

/// Rotation matrix X
#[inline(always)]
pub fn mat_rot_x(theta: f64) -> Mat3 {
    let theta = - theta;
    [
        [ 1.0,     0.0,          0.0    ],
        [ 0.0, theta.cos(), -theta.sin()],
        [ 0.0, theta.sin(),  theta.cos()],
    ]
}

/// Rotation matrix Y
#[inline(always)]
pub fn mat_rot_y(theta: f64) -> Mat3 {
    let theta = - theta;
    [
        [theta.cos(),  0.0, theta.sin()],
        [    0.0,      1.0,     0.0    ],
        [-theta.sin(), 0.0, theta.cos()],
    ]
}

/// Rotation matrix Z
#[inline(always)]
pub fn mat_rot_z(theta: f64) -> Mat3 {
    let theta = - theta;
    [
        [theta.cos(), -theta.sin(), 0.0],
        [theta.sin(),  theta.cos(), 0.0],
        [0.0,             0.0,      1.0],
    ]
}

#[inline(always)]
pub fn mat_rotation(rotation: Vec3) -> Mat3 {
    let a = mat_rot_x(rotation[0]);
    let b = mat_rot_y(rotation[1]);
    let c = mat_rot_z(rotation[2]);
    row_mat3_mul(a, row_mat3_mul(b, c))
}

#[inline(always)]
pub fn mat3xv3_mul<T>(
    a: Matrix3<T>,
    b: Vector3<T>,
) -> Vector3<T>
    where T: Copy + Add<T, Output = T> + Mul<T, Output = T>
{
    [
        a[0][0]*b[0] + a[0][1]*b[0] + a[0][2]*b[0],
        a[1][0]*b[1] + a[1][1]*b[1] + a[1][2]*b[1],
        a[2][0]*b[2] + a[2][1]*b[2] + a[2][2]*b[2],
    ]
}
