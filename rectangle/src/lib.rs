pub mod types;

pub fn calculate_area(rectangle: types::Rectangle) -> types::RectangleArea {
    rectangle.width * rectangle.height
}

pub fn left_fit_to_right(left: types::Rectangle, right: types::Rectangle) -> bool {
    let fits_directly = left.width <= right.width && left.height <= right.height;
    let fits_rotated = left.width <= right.height && left.height <= right.width;

    fits_directly || fits_rotated
}

#[cfg(test)]
mod left_fit_to_right_tests {
    use super::*;

    #[test]
    fn given_fits_directly() {
        let left = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };
        let right = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };

        let when_asking_left_fit_to_right = left_fit_to_right(left, right);
        assert_eq!(when_asking_left_fit_to_right, true)
    }

    #[test]
    fn given_fits_rotated() {
        let left = types::Rectangle {
            width: 2 as types::RectanglSide,
            height: 1 as types::RectanglSide,
        };
        let right = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };

        let when_asking_left_fit_to_right = left_fit_to_right(left, right);
        assert_eq!(when_asking_left_fit_to_right, true)
    }

    #[test]
    fn given_width_dont_fit() {
        let left = types::Rectangle {
            width: 22 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };
        let right = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };

        let when_asking_left_fit_to_right = left_fit_to_right(left, right);
        assert_eq!(when_asking_left_fit_to_right, false)
    }

    #[test]
    fn given_height_dont_fit() {
        let left = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 33 as types::RectanglSide,
        };
        let right = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };

        let when_asking_left_fit_to_right = left_fit_to_right(left, right);
        assert_eq!(when_asking_left_fit_to_right, false)
    }

    #[test]
    fn given_both_size_dont_fit() {
        let left = types::Rectangle {
            width: 22 as types::RectanglSide,
            height: 33 as types::RectanglSide,
        };
        let right = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };

        let when_asking_left_fit_to_right = left_fit_to_right(left, right);
        assert_eq!(when_asking_left_fit_to_right, false)
    }
}

#[cfg(test)]
mod calculate_area_tests {
    use super::*;

    #[test]
    fn given_rectangle() {
        let rectangle = types::Rectangle {
            width: 1 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };
        let when_calculating_area = calculate_area(rectangle);
        assert_eq!(when_calculating_area, 2 as types::RectangleArea);
    }

    #[test]
    fn given_empty_rectangle() {
        let rectangle = types::Rectangle {
            width: 0 as types::RectanglSide,
            height: 2 as types::RectanglSide,
        };
        let when_calculating_area = calculate_area(rectangle);
        assert_eq!(when_calculating_area, 0 as types::RectangleArea);
    }
}