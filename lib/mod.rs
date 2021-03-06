pub mod i_bound;
pub mod i_comparable;
pub mod i_intersect;
pub mod i_shape;
pub mod i_vicinity;

pub mod dualquat;
pub mod mat;
pub mod quat;

pub mod bound;
pub mod bound_sphere;

pub mod line;
pub mod plane;
pub mod point;
///shape implementations
pub mod ray;
pub mod rbox;
pub mod sphere;
pub mod triprism;

pub mod intersect_gjk;

#[cfg(test)]
mod test;
