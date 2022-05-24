mod print;
mod vars;
mod types;
mod strings;
mod assertions;
mod tuples;

fn main() {
  print::run();
  vars::run();
  types::run();
  strings::run();
  assertions::run();
  tuples::run();
}