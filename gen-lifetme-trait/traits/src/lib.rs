//trait definitions are a way to group together similar types
pub trait Summary {
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_from_author())
    }

    fn summarize_from_author(&self) -> String;
}

//Now if I use
// impl Summary for NewsArticle {} it'll use the default implementation

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_from_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_from_author(&self) -> String {
        format!("@{}", self.username)
    }
}
