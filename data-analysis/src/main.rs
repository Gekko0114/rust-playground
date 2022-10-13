use std::process::exit;
use std::env::args;

mod linear_with_sgd;
mod simple_nn;
fn main() {
    let args: Vec<String> = args().collect();
    let model = if args.len() < 2 {
        None
    } else {
        Some(args[1].as_str())
    };

    let res = match model {
        None => {println!("Run cargo run [nn|linear sdg] to get outputs", ); Ok(())},
        Some("nn") => simple_nn::run(),
        Some(_) => linear_with_sgd::run(),
    };

    exit(match res {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    })

}
