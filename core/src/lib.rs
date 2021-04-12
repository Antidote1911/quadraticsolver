use std::io::{stdout, stdin, Write};

fn delta(a: &f64, b: &f64, c: &f64) -> f64 {
    /*
     * determinant of a quadratic eq is b²-4ac (coeffb²-(4*coeffa*coeffc))
     *
     * finding the determinant is quite important because with
     * this value, one can determine if the roots are natural
     * or imaginary.
     */

    b.powi(2) - (4.0* a *c)
}

fn get_coeff() -> (f64, f64, f64) {
    /*
     * get the coefficents from stdin
     *
     * returns a tuple of coefficients and constants
     * the order goes by this
     * (coefficient of x^2, coefficient of x, constant)
     */

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


pub fn calculate_root() -> (f64, f64, bool) {
    /*
     * calculate the root of the quadratic equation
     * get coefficients directly from get_coeff
     *
     * the returned values are root, root, is_real bool
     */

    let mut is_real = true;

    // get the coefficients
    let coeffs = get_coeff();
    let a = coeffs.0;
    let b = coeffs.1;
    let c = coeffs.2;

    println!(
        "the quadratic equation is {}x^2 + {}x + {}",a,b,c
    );

    // calculate the determinant
    let delta = delta(&a, &b, &c);
    println!("delta is: {}", delta);

    // check if it real
    if delta < 0.0 { is_real = false; }

    let (x1, x2): (f64, f64);
    x1 = ((-1.0*b) + (delta as f64).sqrt() as f64) / (2.0*a);
    x2 = ((-1.0*b) - (delta as f64).sqrt() as f64) / (2.0*a);

    (x1, x2, is_real)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determinant_test() {
        println!("{}", delta(&4.0, &7.0, &4.0));
    }

    #[test]
    fn input_test() {
        get_coeff();
    }

    #[test]
    fn root_test() {
        let kek = calculate_root();
        println!("{:?}", kek);
    }
}
