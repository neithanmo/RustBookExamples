
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl NewsArticle {
    fn memberMethod(){
        println!( "Article member method");
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Tweet {
    fn memberMethod(){
        println!("tweet member method");
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

// only receive as parameter only Summary implementers
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
    //item.memberMethod();//wont compile
}

pub fn notifyGeneralType<T: Summary>(item: T) {//will accept any type which implement the Summarize trait, ut we cant call T owned methods 
    println!("Breaking news! {}", item.summarize());
    //item.memberMethod();wont compile.
}


fn main() {

    let tweet = Tweet {
                    username: String::from("horse_ebooks"),
                    content: String::from("of course, as you probably already know, people"),
                    reply: false,
                    retweet: false,
                };
    println!("1 new tweet: {}", tweet.summarize());
    //notify(tweet);//not erros tweet implement trait Summarize but if call to memberMethod is commented
    //notifyGeneralType(tweet);//compile, notifyGeneral can receive any type T which implement Summerize
}
