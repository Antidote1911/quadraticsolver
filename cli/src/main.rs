extern crate escore;


fn main() {
    // get roots

    let roots = escore::calculate_root();

    if roots.2 {
        println!("the roots are :");        
        println!("x1 is: {} and x2 is: {}", roots.0.to_string(),roots.1.to_string());
    } else {
        println!("Delta < 0. No real solution");
    }

    println!();
}
