#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub height: RectanglSide,
    pub width: RectanglSide,
}
pub type RectanglSide = u32;
pub type RectangleArea = u32;

#[cfg(test)]
mod equality_tests {
    use super::*;

    #[test]
    fn given_same_size() {
        let left = Rectangle {
            width: 1 as RectanglSide,
            height: 2 as RectanglSide,
        };
        let right = Rectangle {
            width: 1 as RectanglSide,
            height: 2 as RectanglSide,
        };
        assert_eq!(left, right)
    }

    #[test]
    fn given_different_width() {
        let left = Rectangle {
            width: 1 as RectanglSide,
            height: 22 as RectanglSide,
        };
        let right = Rectangle {
            width: 1 as RectanglSide,
            height: 2 as RectanglSide,
        };
        assert_ne!(left, right)
    }

    #[test]
    fn given_different_height() {
        let left = Rectangle {
            width: 1111 as RectanglSide,
            height: 2 as RectanglSide,
        };
        let right = Rectangle {
            width: 1 as RectanglSide,
            height: 2 as RectanglSide,
        };
        assert_ne!(left, right)
    }

    #[test]
    fn given_different_both_sides() {
        let left = Rectangle {
            width: 1111 as RectanglSide,
            height: 2222 as RectanglSide,
        };
        let right = Rectangle {
            width: 1 as RectanglSide,
            height: 2 as RectanglSide,
        };
        assert_ne!(left, right)
    }
}