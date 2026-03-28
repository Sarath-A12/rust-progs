use std::fmt::{Debug, Display};

use traits::{SocialPost, Summary};
// IMPORTANT: for implementing traits to types - you either need the
// type or the trait to be local to your crate. If not it'll not work.
// Coherence problem
pub fn notify(item: &impl Summary) {
    // accepts any type that has implemented the Summary trait
    println!("Breaking news! {}", item.summarize());
}

//Another version (we call it the bitter version)
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    // Eat 5 star
}

// Multiple trait bounds
pub fn notify3(item: &(impl Summary + Display)) {}
// Or use the bitter version
//

fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {}
//use where clause instead

fn some_func_easy<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

// returning a trait
//
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
    /*
     * Important point: It can either return SocialPost OR NewsArticle. It can't return both as per
     * code
     */
}
fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("huhhhh wt??"),
        reply: false,
        repost: false,
    };
    println!("1 new post {}", post.summarize());
}
