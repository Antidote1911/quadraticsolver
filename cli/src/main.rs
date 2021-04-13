extern crate escore;

use std::io::{stdout, stdin, Write};


fn main() {
    let coeff = get_coeff();
    println!(
        "the quadratic equation is {}x^2 + {}x + {}", &coeff.0, &coeff.1, &coeff.2
    );


    let roots = escore::calculate_root(&coeff.0, &coeff.1, &coeff.2);


    // if delta is <0
    if roots.3 {
        println!("delta is: {}", roots.0);
        println!("Solutions are :");
        println!("x1 = {} and x2 = {}", roots.1.to_string(), roots.2.to_string());
    } else {
        println!("Delta < 0. No real solution");
    }

    println!();
}

fn get_coeff() -> (f64, f64, f64) {

    // closure to print string
    let printstr = |string: &str| {
        print!("{}", string);

        stdout()
            .flush()
            .unwrap();
    };

    // closure to get input. specifically returns f64
    let input = |prompt: &str| {
        let mut num = String::new();

        printstr(prompt);

        // read from stdin
        stdin()
            .read_line(&mut num)
            .expect("can't read from stdin");

        // convert string to f64
        let num: f64 = num
            .trim()
            .parse()
            .unwrap_or_else(|_| 0.0);

        num
    };
    // coefficient a
    let a: f64 = input(&"enter a: ");

    if a == 0.0 {
        println!("If A=0, this is not a quadratic equation !");
        std::process::exit(1);
    }

    let b: f64 = input(&"enter b: ");
    let c: f64 = input(&"enter c: ");
    (a, b, c)
}