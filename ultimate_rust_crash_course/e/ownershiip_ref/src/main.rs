// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });    

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn inspect(x: &String) {
    if (*x).ends_with("s") {
        println!("plural") 
    } else {
        println!("singular")
    }
}

fn change(x: &mut String) {
    if (*x).ends_with("s") {
        return;
    } else {
        (*x).push_str("s");
    }
}

fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        return true;
    } else {
        return false;
    }
}

fn bedazzle(s: &mut String) {
    *s = String::from("sparkly");
}