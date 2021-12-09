mod color;
mod tuple;

use tuple::{Point, Tuple, Vector};
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuple_equals_works() {
        let a = Tuple::new(0, 0, 0, 1);
        let b = Tuple::new(
            0.00000000000000001,
            0.00000000000000001,
            0.000000000000000000000001,
            1.0,
        );
        assert!(a.eq(&b));
    }

    #[test]
    fn sub_point_point_works() {
        let a = Point::new(3, 2, 1);
        let b = Point::new(5, 6, 7);
        assert_eq!(a - b, Vector::new(-2, -4, -6));
    }

    #[test]
    fn sub_point_vector_works() {
        let a = Point::new(3, 2, 1);
        let b = Vector::new(5, 6, 7);

        assert_eq!(a - b, Point::new(-2, -4, -6));
    }

    #[test]
    fn sub_vector_vector_works() {
        let a = Vector::new(3, 2, 1);
        let b = Vector::new(5, 6, 7);

        assert_eq!(a - b, Vector::new(-2, -4, -6));
    }

    #[test]
    fn mul_vector_num_works() {
        let a = Tuple::new(1, -2, 3, -4);
        let b = 3.5;

        assert_eq!(a * b, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn div_vector_num_works() {
        let a = Tuple::new(1, -2, 3, -4);
        let b = 2;

        assert_eq!(a / b, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_works() {
        let a = Vector::new(1, 2, 3);

        assert_eq!(a.magnitude(), (14.0_f64).sqrt());
    }

    #[test]
    fn normalize_works() {
        let mut a = Vector::new(1, 2, 3);
        let b = Vector::new(0.26726, 0.53452, 0.80178);
        assert_eq!(a.to_normalized_vector(), b);
    }

    #[test]
    fn dot_works() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);

        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_works() {
        let a = Vector::new(1, 2, 3);
        let b = Vector::new(2, 3, 4);

        assert_eq!(a.cross(&b), Vector::new(-1, 2, -1));
    }
}
