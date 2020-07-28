use mat::*;
use plane::Plane;
use point::Point3;
use shape::Shape;

#[test]
fn test_intersect_plane_point() {
    //plane point intersection
    {
        let a = Point3::init(&[2f64, 1f64, 2f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([2f64, 1f64, 2f64])));
            }
            _ => panic!("unexpected result for plane point intersection"),
        }
    }
    //plane point intersection
    {
        let a = Point3::init(&[2f64, 1f64, 2f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match b.get_intersect(&a) {
            (true, Some(loc)) => {
                assert!(Mat3x1::from(loc).equal(&Mat3x1::new([2f64, 1f64, 2f64])));
            }
            _ => panic!("unexpected result for plane point intersection"),
        }
    }

    //plane point no intersection
    {
        let a = Point3::init(&[2f64, 1.05f64, 2f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for plane point no intersection"),
        }
    }

    //plane point no intersection
    {
        let a = Point3::init(&[2f64, 0.99f64, 2f64]);
        let b = Plane::init(&[1f64, 1f64, 1f64], &[0f64, 1f64, 0f64]);
        match a.get_intersect(&b) {
            (false, None) => (),
            _ => panic!("unexpected result for plane point no intersection"),
        }
    }
}
