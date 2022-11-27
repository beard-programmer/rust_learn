fn main() {
    println!("Hello, rust learner!\n");
    display_ownership::read_only();

    let scale = 2;
    let rectangle = rectangle::types::Rectangle {
        width: dbg!(1 * scale) as rectangle::types::RectanglSide,
        height: 2 as rectangle::types::RectanglSide,
    };

    dbg!(&rectangle);
    dbg!(rectangle::calculate_area(rectangle));

    let other_rectangle = dbg!(rectangle::types::Rectangle {
        width: (1 * scale) as rectangle::types::RectanglSide,
        height: 2 as rectangle::types::RectanglSide,
    });

    let area: rectangle::types::RectangleArea = rectangle::calculate_area(other_rectangle);
    dbg!(area);
}
