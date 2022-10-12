use std::process::exit;

mod linear_with_sgd;

fn main() {
    let res = linear_with_sgd::run();

    exit(match res {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    })

}
