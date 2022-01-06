extern crate ansi_term;

use ansi_term::Colour::Fixed;

use crate::Card;

pub fn print(card: Card) {
  let protocol = "https://";

  let blue = 51;
  let pink = 1;
  let fixed_blue = Fixed(blue);
  let fixed_pink = Fixed(pink);
  let fixed_blue_bold = fixed_blue.bold();
  let fixed_pink_bold = fixed_pink.bold();

  let edge = fixed_pink.paint("│");
  let top = format!("{}{}{}", "╭", ["─"; 64].join(""), "╮");
  let bottom = format!("{}{}{}", "╰", ["─"; 64].join(""), "╯");

  println!("
  {}
  {}                                                                {}
  {} {} / {}                                  {}
  {}                                                                {}
  {} {}     {} @ {}                                    {}
  {}                                                                {}
  {} {}      {}{}                              {}
  {} {}   {}{}                       {}
  {} {}  {}{}{}                      {}
  {} {}      {}{}{}                       {}
  {}                                                                {}
  {}
  ",
  fixed_pink.paint(top),
  edge,
  edge,
  edge,
  fixed_blue_bold.paint(card.name),
  fixed_pink_bold.paint(card.handle),
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("work:"),
  card.title,
  card.company,
  edge,
  edge,
  edge,
  edge,
  fixed_blue_bold.paint("web:"),
  protocol,
  card.portfolio,
  edge,
  edge,
  fixed_blue_bold.paint("resume:"),
  protocol,
  card.resume,
  edge,
  edge,
  fixed_blue_bold.paint("twitter:"),
  protocol,
  card.twitter,
  card.handle,
  edge,
  edge,
  fixed_blue_bold.paint("git:"),
  protocol,
  card.github,
  card.handle,
  edge,
  edge,
  edge,
  fixed_pink.paint(bottom)
  );
}