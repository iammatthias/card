use std::env;

mod card;
mod print;
mod colours;

use card::Card;
use print::print;
use colours::colours;


// output card
fn card () {
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
