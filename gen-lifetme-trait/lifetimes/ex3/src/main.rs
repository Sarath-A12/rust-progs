struct ImportantExcerpt<'a> {
    part: &'a str,
} // an isntance of ImportantExcerpt can't outlive the reference it holds in its part field
fn main() {
    let novel = String::from("For you, anything. What??");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // All string literals have a "static lifetime"
    let s: &'static str = "I have a static lifetime";
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    } //no reference so ok

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    } // third rule applies here
}

// lifetime elision - set of rules which try to determine the lifetime
// of the variables
// -> Rule 1 - each parameter gets their own lifetimes
// -> Rule 2 - If one input - all outputs have same lifetime
// -> Rule 3 - If self is one of the inputs , lifetime of self is assigned to all output parameters
//
