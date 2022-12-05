pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

pub enum ReviewResult {
    Reject(RejectedPost),
    Approve(ApprovedPost),
}

pub struct RejectedPost {
    content: String,
    reason: String,
}

impl RejectedPost {
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

pub struct ApprovedPost {
    content: String,
}

impl ApprovedPost {
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub fn build_post(content: Option<String>) -> DraftPost {
    DraftPost {
        content: content.unwrap_or("".to_string()),
    }
}

pub fn add_text(post: DraftPost, text: &str) -> DraftPost {
    DraftPost {
        content: post.content + text,
    }
}

pub fn request_review(post: DraftPost) -> PendingReviewPost {
    PendingReviewPost {
        content: post.content,
    }
}

pub fn reject(post: PendingReviewPost, reason: String) -> ReviewResult {
    ReviewResult::Reject(RejectedPost {
        reason,
        content: post.content,
    })
}

pub fn approve(post: PendingReviewPost) -> ReviewResult {
    ReviewResult::Approve(ApprovedPost {
        content: post.content,
    })
}
