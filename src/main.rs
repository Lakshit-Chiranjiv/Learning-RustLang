mod print;
mod vars;
mod types;
mod strings;
mod assertions;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;

fn main() {
  print::run();
  vars::run();
  types::run();
  strings::run();
  assertions::run();
  tuples::run();
  arrays::run();
  vectors::run();
  conditionals::run();
  loops::run();
}