use std::time::{SystemTime, UNIX_EPOCH};

pub trait Summary {
  fn summarize(&self) -> String;

  // default function that I can't get to work
  fn now(&self) -> bool {
    let start = SystemTime::now();
    let since_the_epoch = start
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
    println!("Current time is {}", since_the_epoch.as_secs());
    true
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

impl Tweet {
  pub fn new(username: String, content: String, reply: bool, retweet: bool) -> Tweet {
    Tweet {
      username,
      content,
      reply,
      retweet,
    }
  }
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub fn pls_work(s: &str) {
  println!("arg: {}", s);
}
