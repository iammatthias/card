extern crate ansi_term;

use ansi_term::Colour::Fixed;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// general metadata
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
  pub name: String,
  pub email: String,
  pub handle: String,
  pub role: String,
  pub company: String,
  pub web: String,
  pub twitter: String,
  pub git: String,
}

fn print_metadata(data: Metadata) {
  let blue = 39;
  let pink = 202;
  let fixed_blue = Fixed(blue);
  let fixed_pink = Fixed(pink);
  let fixed_blue_bold = fixed_blue.bold();
  let fixed_pink_bold = fixed_pink.bold();
  let top = format!(
    "{}{}",
    fixed_pink.paint("╭"),
    fixed_pink.paint(["─"; 64].join(""))
  );
  let space = format!("{}", [" "; 4].join(""));
  let edge = fixed_pink.paint("│");
  println!("{}", top);
  println!("{}{}", edge, " ");
  println!(
    "{}{}{} / {}",
    edge,
    space,
    fixed_blue_bold.paint(data.name),
    fixed_pink_bold.paint(data.handle)
  );
  println!("{}{}", edge, " ");
  println!("{}{}{} @ {}", edge, space, data.role, data.company);
  println!("{}{}", edge, " ");
  println!("{}{}web: {}", edge, space, data.web);
  println!("{}{}twitter: {}", edge, space, data.twitter);
  println!("{}{}git: {}", edge, space, data.git);
  println!("{}{}", edge, " ");
}

async fn metadata() {
  let url = "https://iammatthias.com/api/feed/";
  let client = reqwest::Client::new();
  let response = client.get(url).send().await.unwrap();
  match response.status() {
    reqwest::StatusCode::OK => {
      match response.json::<Metadata>().await {
        Ok(parsed) => print_metadata(parsed),
        Err(_) => println!("Hm, the response didn't match the shape we expected."),
      };
    }
    reqwest::StatusCode::UNAUTHORIZED => {
      println!("Need to grab a new token");
    }
    other => {
      panic!("Uh oh! Something unexpected happened: {:?}", other);
    }
  };
}

// recent content
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
  pub version: String,
  pub title: String,
  pub home_page_url: String,
  pub feed_url: String,
  pub description: String,
  pub author: Author,
  pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
  pub name: String,
  pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
  pub id: String,
  pub title: String,
  pub date_modified: String,
}

fn print_feed(feed: Feed) {
  print_content(feed.items.iter().take(4).collect())
}

fn print_content(items: Vec<&Item>) {
  let pink = 202;
  let fixed_pink = Fixed(pink);
  let space = format!("{}", [" "; 2].join(""));
  let edge = fixed_pink.paint("│");
  for item in items {
    println!("{}{}꩜ {}", edge, space, item.title);
    println!("{}{}  {}", edge, space, item.id);
    println!("{}{}", edge, " ")
  }
}

async fn feed() {
  let url = "https://iammatthias.com/api/feed/json/";
  let client = reqwest::Client::new();
  let response = client.get(url).send().await.unwrap();
  match response.status() {
    reqwest::StatusCode::OK => {
      match response.json::<Feed>().await {
        Ok(parsed) => print_feed(parsed),
        Err(_) => println!("Hm, the response didn't match the shape we expected."),
      };
    }
    reqwest::StatusCode::UNAUTHORIZED => {
      println!("Need to grab a new token");
    }
    other => {
      panic!("Uh oh! Something unexpected happened: {:?}", other);
    }
  };
}

#[tokio::main]
async fn card() {
  metadata().await;
  feed().await;
}

#[wasm_bindgen(start)]
pub fn main() {
  card();
}
