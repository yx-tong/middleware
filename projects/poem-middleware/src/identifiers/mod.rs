use semantic_id::{Semantic64, SemanticKey};

pub struct UserKey;

pub type UserId = Semantic64<UserKey>;

impl SemanticKey for UserKey {
    const KEY: &'static str = "user";
}

pub struct TagKey;

pub type TagId = Semantic64<TagKey>;

impl SemanticKey for TagKey {
    const KEY: &'static str = "tag";
}

pub struct MessageKey;

pub type MessageId = Semantic64<MessageKey>;

impl SemanticKey for MessageKey {
    const KEY: &'static str = "message";
}

pub struct AttachmentKey;

pub type AttachmentId = Semantic64<AttachmentKey>;

impl SemanticKey for AttachmentKey {
    const KEY: &'static str = "attachment";
}

pub struct LogKey;

pub type LogId = Semantic64<LogKey>;

impl SemanticKey for LogKey {
    const KEY: &'static str = "log";
}

pub struct CommentKey;

pub type CommentId = Semantic64<CommentKey>;

impl SemanticKey for CommentKey {
    const KEY: &'static str = "comment";
}
