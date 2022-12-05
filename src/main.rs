mod blog;

fn main() {
    println!("Hello, rust learner!\n");
    display_ownership::read_only();
    rectangles();
    blog();
}

fn rectangles() {
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

fn blog() {
    let post = blog::build_post(Some("My very first post!\n".to_string()));
    let post = blog::add_text(post, "I ate a salad for lunch today");
    let post = blog::request_review(post);

    let review_post = |post| -> blog::ReviewResult {
        match rand::Rng::gen_bool(&mut rand::thread_rng(), 0.66) {
            true => blog::approve(post),
            false => blog::reject(post, "Stupid content".to_string()),
        }
    };

    match review_post(post) {
        blog::ReviewResult::Approve(approved_post) => {
            println!("Approved!\nPost: {}", approved_post.content())
        }
        blog::ReviewResult::Reject(rejected_post) => {
            println!("Rejected!\nReason: {}", rejected_post.reason())
        }
    };
}
