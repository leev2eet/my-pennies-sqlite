use std::env;

mod cli;

fn process_args(args: &Vec<String>) {
  if args.len() == 0 {
    panic!("Cannot do anything for now");
  }

  for arg in args {
    match arg.as_str() {

      "--version" | "-v" => cli::version::exec(),

      _ => panic!("Unknown argument '{arg}'"),

    }
  }
}

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  process_args(&args);
}
