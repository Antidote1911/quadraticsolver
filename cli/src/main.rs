extern crate escore;

use std::io::{stdout, stdin, Write};


fn main() {
    let coeff = get_coeff();

    // formating the equation
    let mut equation = "{0}x^2{1}x{2}".replace("{0}", &coeff.0.to_string());
    let mut b_str = coeff.1.to_string();
    let mut c_str = coeff.2.to_string();
    if coeff.0.is_sign_positive() {
        b_str.insert(0, '+');
    }
    equation = equation.replace("{1}", &b_str);

    if coeff.2.is_sign_positive() {
        c_str.insert(0, '+');
    }
    equation = equation.replace("{2}", &c_str);
    //// end formating

    println!("roots for {} are:", equation);

    let roots = escore::calculate_root(&coeff.0, &coeff.1, &coeff.2);


    // if delta is > 0
    if roots.3 {
        println!("delta is: {}", roots.0);
        println!("x1 = {} and x2 = {}", roots.1.to_string(), roots.2.to_string());
    } else {
        println!("Delta < 0. No real solution");
    }
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
        std::process::exit(0);
    }

    let b: f64 = input(&"enter b: ");
    let c: f64 = input(&"enter c: ");
    (a, b, c)
}
