///
/// Traits are similar to a feature often called interfaces in other languages, although with some differences.
/// 
/// A type’s behavior consists of the methods we can call on that type.
/// Different types share the same behavior if we can call the same methods on all of those types.
/// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
/// 

/*pub trait Summary {
    fn summarize(&self) -> String;
}*/

/// trait with default
pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("(Read more on author...)")
    }
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/* multiple traits
pub fn notify<T: Summary>(item1: &T, item2: &T) {
...}
 
pub fn notify<T: Summary + Display>(item: &T) {
...
/}

// where syntax 
 fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
U: Clone + Debug
{

**/

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
/////////
pub fn test_trait_summarize() {
    let tweet = returns_summarizable();

    println!("");

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available trait default!");
    notify(&article);
    println!("1 new tweet");
    notify(&tweet);
}