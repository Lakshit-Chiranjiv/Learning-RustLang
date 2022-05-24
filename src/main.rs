mod print;
mod vars;
mod types;
mod strings;
mod assertions;

fn main() {
  print::run();
  vars::run();
  types::run();
  strings::run();
  assertions::run();
}