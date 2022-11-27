pub mod types;

pub fn calculate_area(rectangle: types::Rectangle) -> types::RectangleArea {
    rectangle.width * rectangle.height
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