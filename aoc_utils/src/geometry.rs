use num::PrimInt;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point2D<T: PrimInt + Default + Display> {
    x: T,
    y: T,
}

impl<T: PrimInt + Default + Display> Default for Point2D<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: PrimInt + Default + Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: PrimInt + Default + Display> Point2D<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn up(&self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y + T::one(),
        })
    }

    fn down(&self) -> Option<Self> {
        match self.y.checked_sub(&T::one()) {
            Some(y) => Some(Self { x: self.x, y }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_creation() {
        let point_usize = Point2D::<usize>::new(1, 2);
        assert_eq!(point_usize.x, 1usize);
        assert_eq!(point_usize.y, 2usize);

        let point_u8 = Point2D::<u8>::new(3, 4);
        assert_eq!(point_u8.x, 3u8);
        assert_eq!(point_u8.y, 4u8);

        let point_i8 = Point2D::<i8>::new(-1, -5);
        assert_eq!(point_i8.x, -1i8);
        assert_eq!(point_i8.y, -5i8);
    }

    #[test]
    fn test_defaults() {
        assert_eq!(Point2D::<usize>::default(), Point2D::<usize>::new(0, 0));
        assert_eq!(Point2D::<i128>::default(), Point2D::<i128>::new(0, 0));
        assert_eq!(Point2D::<u32>::default(), Point2D::<u32>::new(0, 0));
    }

    #[test]
    fn test_display() {
        let point = Point2D::<usize>::new(1, 2);
        assert_eq!(format!("{point}"), "(1, 2)");

        let other_point = Point2D::<i8>::new(-4, -3);
        assert_eq!(format!("{other_point}"), "(-4, -3)");
    }

    #[test]
    fn test_up() {
        let point = Point2D::<usize>::default();
        assert_eq!(point.up().unwrap(), Point2D::<usize>::new(0, 1));
        assert_eq!(
            point.up().unwrap().up().unwrap(),
            Point2D::<usize>::new(0, 2)
        );
    }

    #[test]
    fn test_down() {
        let point = Point2D::<usize>::new(1, 1);
        assert_eq!(point.down().unwrap(), Point2D::<usize>::new(1, 0));
    }

    #[test]
    fn test_down_on_non_negative_int_type() {
        let point = Point2D::<usize>::default();
        assert_eq!(point.down(), None);
    }
}
