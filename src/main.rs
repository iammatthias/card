// use curl::http;
// use serde_json::Value;

use std::env;

mod card;
mod print;
mod colours;

use card::Card;
use print::print;
use colours::colours;


// output card
fn card () {
  // let url = "https://iammatthias.com/api/feed/";
  // let resp = http::handle()
  //   .get(url)
  //   .exec()
  //   .unwrap_or_else(|e| {
  //     panic!("Failed to get {}; error is {}", url, e);
  // });

  // if resp.get_code() != 200 {
  //   println!("Unable to handle HTTP response code {}", resp.get_code());
  //     return;
  // }

  // let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
  //   panic!("Failed to parse response from {}; error is {}", url, e);
  // });

  // let json: Value = serde_json::from_str(body).unwrap_or_else(|e| {
  //   panic!("Failed to parse json; error is {}", e);
  // });
    

  let card = Card {
    name: "Matthias Jordan",
    title: "Growth",
    handle: "iammatthias",
    company: "Tordnado",
    portfolio: "iammatthias.com",
    resume: "iammatthias.com/resume",
    twitter: "twitter.com/",
    github: "github.com/",
  };

  print(card);
}

fn main () {
  // collect args
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => {
      card();
    }
    2 => {
      let arg = &args[1];
      match arg.as_str() {
        "colours" => colours(),
        "colors" => colours(),
        _ => println!("This is not a valid argument. please use none or `colours`"),
      }
    }
    _ => {
      card();
    }
  }
}
