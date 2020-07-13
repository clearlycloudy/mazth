use ndarray::prelude::*;

use bound::IBound;
use shape::{IShape, ShapeType};
use vicinity::IVicinity;

use bound_aabb::AxisAlignedBBox;
use mat::*;

#[derive(Debug, Clone)]
pub struct Plane {
    pub _offset: Matrix1D,
    pub _normal: Matrix1D,
    pub _bound: AxisAlignedBBox,
    pub _vicinity: f64,
}

impl Plane {
    pub fn init(offset: &[f64], normal: &[f64]) -> Plane {
        assert!(offset.len() == 3);
        assert!(normal.len() == 3);
        Plane {
            _offset: arr1(&[offset[0], offset[1], offset[2]]),
            _normal: normalize_vec_l2_1d(&arr1(&[normal[0], normal[1], normal[2]]).view()),
            _bound: AxisAlignedBBox::init(
                ShapeType::Plane,
                &[&offset[0..3], &normal[0..3]].concat(),
            ),
            _vicinity: 0.000001f64,
        }
    }
}

impl IShape for Plane {
    fn get_shape_data(&self) -> Vec<f64> {
        vec![
            self._offset[0],
            self._offset[1],
            self._offset[2],
            self._normal[0],
            self._normal[1],
            self._normal[2],
        ]
    }
    fn get_type(&self) -> ShapeType {
        ShapeType::Plane
    }
    fn get_bound(&self) -> &dyn IBound {
        &self._bound
    }
    // this shall test for intersection of bounding shapes first before procedding to test intersection using algorithms of higher complexity
    fn get_intersect(&self, other: &dyn IShape) -> (bool, Option<Matrix1D>) {
        if !self.get_bound().intersect(other.get_bound()) {
            return (false, None);
        } else {
            match other.get_type() {
                ShapeType::Plane => {
                    unimplemented!();
                }
                ShapeType::Ray => {
                    //see Ray3 for ray plane intersection
                    return other.get_intersect(self);
                }
                ShapeType::Sphere => {
                    //see sphere for sphere plane intersection
                    return other.get_intersect(self);
                }
                ShapeType::Point => {
                    let other_shape_data = other.get_shape_data();
                    let b_off = arr1(&[
                        other_shape_data[0],
                        other_shape_data[1],
                        other_shape_data[2],
                    ]);
                    let k = self._normal.dot(&self._offset);
                    let c = self._normal.dot(&b_off);
                    let d = k - c;
                    if !self.within_vicinity(d, 0f64) {
                        return (false, None);
                    }
                    return (true, Some(b_off));
                }
                _ => {
                    unimplemented!();
                }
            }
        }
    }
    fn get_support(&self, _v: &Matrix1D) -> Option<Matrix1D> {
        None
    }
}

impl IVicinity<f64> for Plane {
    fn set_vicinity(&mut self, epsilon: f64) {
        self._vicinity = epsilon.abs();
    }
    fn within_vicinity(&self, a: f64, b: f64) -> bool {
        if a + self._vicinity >= b && a - self._vicinity <= b {
            true
        } else {
            false
        }
    }
}
